use std::collections::{BTreeMap, BTreeSet, VecDeque};

use num_bigint::BigInt;
use num_traits::Signed;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::VerifiedArtifact;
use crate::cnl_ast::*;
use crate::cnl_lexer::{
    Sentence, UNICODE_DATA_VERSION, lex_and_enumerate, valid_term, validate_and_split,
};
use crate::diagnostic::{Diagnostic, OrderedDiagnostics, normalize_diagnostics};
use crate::pipeline::{CompileOutcome, CompileSuccess, CompileThrough};
use crate::resource::{
    Ledger, Resource, canonical_ir_limit, expression_depth_limit, public_diagnostics_limit,
};

const IR_SCHEMA: &str = include_str!("../../../../04_design/ir/apls-cnl-ir-0.1.schema.json");

#[derive(Clone)]
struct DeclRecord {
    declaration: Declaration,
    span: ByteSpan,
}

#[derive(Clone)]
struct PropertyInfo {
    value_type: ValueType,
    unit_ref: Option<String>,
}

#[derive(Clone)]
struct StateModelInfo {
    owner_ref: String,
    states: BTreeMap<String, String>,
    initial_ref: String,
}

#[derive(Default)]
struct Graph {
    symbols: Vec<SymbolRef>,
    entities: BTreeMap<String, Value>,
    units: BTreeMap<String, Value>,
    properties: BTreeMap<String, Value>,
    property_info: BTreeMap<String, PropertyInfo>,
    actions: BTreeMap<String, Value>,
    action_targets: BTreeMap<String, String>,
    events: BTreeMap<String, Value>,
    aliases: BTreeMap<String, Value>,
    state_models: BTreeMap<String, Value>,
    state_info: BTreeMap<String, StateModelInfo>,
    declaration_sources: BTreeMap<String, Vec<Provenance>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Provenance {
    sentence: ByteSpan,
    roles: Vec<(String, ByteSpan)>,
}

struct Frame {
    kind: &'static str,
    value: Value,
    provenance: Provenance,
}

pub fn compile(bytes: &[u8], logical_path: &str, through: CompileThrough) -> CompileOutcome {
    match compile_inner(bytes, logical_path, through) {
        Ok(artifact) => CompileOutcome::Accepted(artifact),
        Err(diagnostics) => {
            let diagnostics = finalize_diagnostics(diagnostics);
            if diagnostics.iter().any(|d| d.code == "APLS-T0006") {
                CompileOutcome::InternalFailure(diagnostics)
            } else if diagnostics.iter().any(|d| d.code.starts_with("APLS-T")) {
                CompileOutcome::ToolFailure(diagnostics)
            } else {
                CompileOutcome::Rejected(diagnostics)
            }
        }
    }
}

fn finalize_diagnostics(mut diagnostics: OrderedDiagnostics) -> OrderedDiagnostics {
    normalize_diagnostics(&mut diagnostics);
    if diagnostics.len() > crate::limits::MAX_DIAGNOSTICS {
        diagnostics.truncate(crate::limits::MAX_DIAGNOSTICS - 1);
        diagnostics.push(public_diagnostics_limit());
    }
    diagnostics
}

fn compile_inner(
    bytes: &[u8],
    path: &str,
    through: CompileThrough,
) -> Result<CompileSuccess, OrderedDiagnostics> {
    let (source, sentences) = validate_and_split(bytes, path).map_err(one)?;
    let mut ledger = Ledger::default();
    validate_profile(source, &sentences[0], &mut ledger, path).map_err(one)?;
    let mut declaration_records = Vec::new();
    let mut declaration_sentence_indexes = BTreeSet::new();
    for sentence in sentences.iter().skip(1) {
        let streams =
            lex_and_enumerate(sentence, 1, &[], &mut ledger, path, source).map_err(one)?;
        let mut candidates = BTreeSet::new();
        for stream in streams {
            if let Ok(parsed) =
                crate::apls_grammar::DeclarationEntryParser::new().parse(stream.into_iter().map(Ok))
            {
                for span in declaration_syntax_node_spans(&parsed.value, sentence.span) {
                    ledger
                        .add(
                            Resource::SyntaxNodes,
                            1,
                            Some(sentence.index),
                            Some(span),
                            path,
                            source,
                        )
                        .map_err(one)?;
                }
                candidates.insert(parsed.value);
            }
        }
        if candidates.len() > 1 {
            return Err(one(ambiguity(
                path,
                source,
                sentence.span,
                "declaration",
                candidates.iter().map(|v| format!("{v:?}")).collect(),
            )));
        }
        if let Some(declaration) = candidates.into_iter().next() {
            declaration_sentence_indexes.insert(sentence.index);
            declaration_records.push(DeclRecord {
                declaration,
                span: sentence.span,
            });
        }
    }
    let graph = build_graph(&declaration_records, path, source)?;
    let mut frame_groups: BTreeMap<&'static str, BTreeMap<String, (Value, BTreeSet<Provenance>)>> =
        BTreeMap::new();
    let mut transitions_for_checks = Vec::new();
    for sentence in sentences
        .iter()
        .skip(1)
        .filter(|s| !declaration_sentence_indexes.contains(&s.index))
    {
        let streams = lex_and_enumerate(sentence, 2, &graph.symbols, &mut ledger, path, source)
            .map_err(one)?;
        let mut parsed = Vec::new();
        let mut parse_constraint_error = None;
        for stream in streams {
            if let Ok(candidate) =
                crate::apls_grammar::NormativeEntryParser::new().parse(stream.into_iter().map(Ok))
            {
                if let Some(span) = normative_spacing_error(&candidate.value, source) {
                    parse_constraint_error.get_or_insert_with(|| {
                        Diagnostic::source(
                            "APLS-E1101",
                            "literal spacing does not match the controlled grammar",
                            path,
                            source,
                            span,
                        )
                    });
                    continue;
                }
                let expression_depth = normative_expression_depth(&candidate.value);
                if expression_depth > crate::limits::MAX_EXPRESSION_DEPTH {
                    return Err(one(expression_depth_limit(
                        expression_depth,
                        sentence.index,
                        sentence.span,
                        path,
                        source,
                    )));
                }
                for span in normative_syntax_node_spans(&candidate.value, sentence.span, source) {
                    ledger
                        .add(
                            Resource::SyntaxNodes,
                            1,
                            Some(sentence.index),
                            Some(span),
                            path,
                            source,
                        )
                        .map_err(one)?;
                }
                parsed.push(candidate.value);
            }
        }
        if parsed.is_empty() {
            if let Some(error) = parse_constraint_error {
                return Err(one(error));
            }
            return Err(diagnose_unparsed(sentence, &graph, path, source));
        }
        if matches!(through, CompileThrough::Parse) {
            continue;
        }
        let mut canonical = BTreeMap::<Vec<u8>, (Frame, BTreeSet<Provenance>)>::new();
        let mut failures = Vec::new();
        for candidate in parsed {
            match process_candidate(
                candidate,
                sentence.index,
                sentence.span,
                &graph,
                path,
                source,
                &mut ledger,
            ) {
                Ok(frame) => {
                    let provenance = frame.provenance.clone();
                    canonical
                        .entry(canonical_bytes(&frame.value))
                        .and_modify(|(_, provenances)| {
                            provenances.insert(provenance.clone());
                        })
                        .or_insert_with(|| (frame, BTreeSet::from([provenance])));
                }
                Err(error) => failures.push(error),
            }
        }
        if canonical.is_empty() {
            if failures.is_empty() {
                failures.push(Diagnostic::source(
                    "APLS-E1101",
                    "no valid semantic candidate",
                    path,
                    source,
                    sentence.span,
                ));
            }
            suppress_derived_diagnostics(&mut failures);
            return Err(failures);
        }
        if canonical.len() > 1 {
            let frames: Vec<_> = canonical.values().take(2).collect();
            return Err(one(frame_ambiguity(
                path,
                source,
                sentence.span,
                frames[0],
                frames[1],
            )));
        }
        let (frame, provenances) = canonical.into_values().next().expect("one canonical frame");
        if frame.kind == "transition" {
            transitions_for_checks.extend(
                provenances
                    .iter()
                    .cloned()
                    .map(|provenance| (frame.value.clone(), provenance)),
            );
        }
        let id = frame.value["id"].as_str().expect("frame ID").to_owned();
        let entry = frame_groups
            .entry(frame.kind)
            .or_default()
            .entry(id)
            .or_insert_with(|| (frame.value.clone(), BTreeSet::new()));
        if entry.0 != frame.value {
            return Err(one(Diagnostic::tool("APLS-T0006", "semantic ID collision")));
        }
        entry.1.extend(provenances);
    }
    if matches!(through, CompileThrough::Parse) {
        return Ok(CompileSuccess::Parsed);
    }
    if matches!(through, CompileThrough::Check) {
        validate_transitions(&graph, &transitions_for_checks, path, source)?;
        return Ok(CompileSuccess::Checked);
    }
    let artifact = build_ir(
        bytes,
        path,
        source,
        graph,
        frame_groups,
        transitions_for_checks,
    )?;
    Ok(CompileSuccess::Verified(artifact))
}

fn validate_profile(
    source: &str,
    sentence: &Sentence<'_>,
    ledger: &mut Ledger,
    path: &str,
) -> Result<(), Diagnostic> {
    let text = sentence.text;
    let parts = ["本规范采用", "APLS", "简体中文语言版本", "0.1。"];
    let mut at = 0;
    for (index, part) in parts.iter().enumerate() {
        if !text[at..].starts_with(part) {
            return Err(Diagnostic::source(
                "APLS-E1104",
                "the exact language profile declaration is required",
                path,
                source,
                sentence.span,
            ));
        }
        at += part.len();
        if index + 1 < parts.len() {
            let before = at;
            while at < text.len() {
                let ch = text[at..].chars().next().expect("boundary");
                if !matches!(ch, ' ' | '\n' | '\r') {
                    break;
                }
                at += ch.len_utf8();
            }
            if at == before {
                return Err(Diagnostic::source(
                    "APLS-E1104",
                    "profile declaration requires structural whitespace",
                    path,
                    source,
                    sentence.span,
                ));
            }
        }
    }
    if at != text.len() {
        return Err(Diagnostic::source(
            "APLS-E1104",
            "the exact language profile declaration is required",
            path,
            source,
            sentence.span,
        ));
    }
    let streams = lex_and_enumerate(sentence, 0, &[], ledger, path, source)?;
    let mut success = 0;
    for stream in streams {
        if crate::apls_grammar::ProfileParser::new()
            .parse(stream.into_iter().map(Ok))
            .is_ok()
        {
            success += 1;
            ledger.add(
                Resource::SyntaxNodes,
                1,
                Some(0),
                Some(sentence.span),
                path,
                source,
            )?;
        }
    }
    if success != 1 {
        return Err(Diagnostic::source(
            "APLS-E1104",
            "the exact language profile declaration is required",
            path,
            source,
            sentence.span,
        ));
    }
    Ok(())
}

fn build_graph(
    records: &[DeclRecord],
    path: &str,
    source: &str,
) -> Result<Graph, OrderedDiagnostics> {
    let reserved: BTreeSet<&str> = [
        "本规范采用",
        "APLS",
        "简体中文语言版本",
        "是",
        "单位",
        "设备",
        "组件",
        "传感器",
        "执行器",
        "类型的",
        "可观测",
        "可写",
        "可观测且可写",
        "属性",
        "单位为",
        "支持的",
        "动作",
        "事件",
        "的别名",
        "实体",
        "的状态包括",
        "初始状态是",
        "和",
        "当",
        "如果",
        "时",
        "系统",
        "必须",
        "不得",
        "禁止",
        "从",
        "状态进入",
        "状态",
        "安全要求",
        "验收要求",
        "必须在",
        "内成立",
        "说明",
        "并且",
        "等于",
        "不等于",
        "低于",
        "不高于",
        "高于",
        "不低于",
        "处于",
        "收到",
        "真",
        "假",
        "摄氏度",
        "毫秒",
        "秒",
        "分钟",
        "布尔",
        "整数",
        "小数",
        "百分比",
        "文本",
        "时长",
    ]
    .into_iter()
    .collect();
    let mut seen = BTreeMap::<(SymbolKind, String), ByteSpan>::new();
    let mut graph = Graph::default();
    let mut entity_names = BTreeMap::new();
    let mut unit_names = BTreeMap::new();
    let mut property_names = BTreeMap::new();
    let mut event_names = BTreeMap::new();
    for record in records {
        let header = declaration_header(&record.declaration);
        if !valid_term(&header.1) || reserved.contains(header.1.as_str()) {
            return Err(one(Diagnostic::source(
                "APLS-E1008",
                "declared term is invalid or reserved",
                path,
                source,
                header.2,
            )));
        }
        if let Some(previous) = seen.insert((header.0, header.1.clone()), header.2) {
            let mut error = Diagnostic::source(
                "APLS-E1202",
                "duplicate declaration name in one symbol category",
                path,
                source,
                header.2,
            );
            error
                .related_source_spans
                .push(crate::diagnostic::SourceSpan::from_bytes(
                    path, source, previous,
                ));
            return Err(one(error));
        }
        match &record.declaration {
            Declaration::Entity { name, kind } => {
                entity_names.insert(name.value.clone(), format!("entity:{}", name.value));
                graph.entities.insert(format!("entity:{}", name.value), json!({"id":format!("entity:{}",name.value),"display_name":name.value,"entity_kind":kind.value.as_str()}));
            }
            Declaration::Unit { name } => {
                unit_names.insert(name.value.clone(), format!("unit:{}", name.value));
                graph
                    .units
                    .insert(format!("unit:{}", name.value), nominal_unit(&name.value));
            }
            Declaration::Property { name, .. } => {
                property_names.insert(name.value.clone(), format!("property:{}", name.value));
            }
            Declaration::Event { name } => {
                event_names.insert(name.value.clone(), format!("event:{}", name.value));
                graph.events.insert(
                    format!("event:{}", name.value),
                    json!({"id":format!("event:{}",name.value),"display_name":name.value}),
                );
            }
            _ => {}
        }
    }
    let mut action_headers = BTreeMap::<String, Vec<String>>::new();
    for record in records {
        if let Declaration::Action { name, target } = &record.declaration {
            if entity_names.contains_key(&target.value) {
                action_headers
                    .entry(name.value.clone())
                    .or_default()
                    .push(format!("action:{}:{}", target.value, name.value));
            }
        }
    }
    let builtin_units: BTreeMap<String, String> = ["%", "毫秒", "秒", "分钟", "摄氏度"]
        .into_iter()
        .map(|name| (name.into(), format!("unit:{name}")))
        .collect();
    for record in records {
        let declaration_source = declaration_provenance(&record.declaration, record.span);
        match &record.declaration {
            Declaration::Entity { name, .. } => add_source(
                &mut graph,
                &format!("entity:{}", name.value),
                declaration_source,
            ),
            Declaration::Unit { name } => add_source(
                &mut graph,
                &format!("unit:{}", name.value),
                declaration_source,
            ),
            Declaration::Property {
                name,
                value_type,
                access,
                unit,
            } => {
                let raw_unit = unit.as_ref().map(|v| v.value.as_str());
                let canonical_unit =
                    property_unit(value_type.value, raw_unit, &unit_names, &builtin_units)
                        .map_err(|message| {
                            one(Diagnostic::source(
                                "APLS-E1401",
                                message,
                                path,
                                source,
                                unit.as_ref().map_or(name.span, |v| v.span),
                            ))
                        })?;
                let id = format!("property:{}", name.value);
                let (observable, writable) = access.value.flags();
                graph.properties.insert(id.clone(), json!({"id":id,"display_name":name.value,"value_type":value_type.value.as_str(),"observable":observable,"writable":writable,"unit_ref":canonical_unit}));
                graph.property_info.insert(
                    id.clone(),
                    PropertyInfo {
                        value_type: value_type.value,
                        unit_ref: canonical_unit,
                    },
                );
                add_source(&mut graph, &id, declaration_source);
            }
            Declaration::Action { name, target } => {
                let Some(target_id) = entity_names.get(&target.value) else {
                    return Err(one(Diagnostic::source(
                        "APLS-E1204",
                        "action target must name a declared entity",
                        path,
                        source,
                        target.span,
                    )));
                };
                let id = format!("action:{}:{}", target.value, name.value);
                graph.actions.insert(
                    id.clone(),
                    json!({"id":id,"display_name":name.value,"target_ref":target_id}),
                );
                graph.action_targets.insert(id.clone(), target_id.clone());
                add_source(&mut graph, &id, declaration_source);
            }
            Declaration::Event { name } => add_source(
                &mut graph,
                &format!("event:{}", name.value),
                declaration_source,
            ),
            Declaration::Alias {
                alias,
                kind,
                target,
            } => {
                let target_id = match kind.value {
                    AliasKind::Entity => entity_names.get(&target.value).cloned(),
                    AliasKind::Property => property_names.get(&target.value).cloned(),
                    AliasKind::Event => event_names.get(&target.value).cloned(),
                    AliasKind::Unit => unit_names
                        .get(&target.value)
                        .or_else(|| builtin_units.get(&target.value))
                        .cloned(),
                    AliasKind::Action => {
                        let matches = action_headers
                            .get(&target.value)
                            .cloned()
                            .unwrap_or_default();
                        if matches.len() == 1 {
                            matches.into_iter().next()
                        } else {
                            None
                        }
                    }
                };
                let Some(target_id) = target_id else {
                    return Err(one(Diagnostic::source(
                        "APLS-E1204",
                        "alias target does not resolve uniquely in its declared category",
                        path,
                        source,
                        target.span,
                    )));
                };
                let id = format!("alias:{}:{}", kind.value.as_str(), alias.value);
                graph.aliases.insert(id.clone(), json!({"id":id,"display_name":alias.value,"target_kind":kind.value.as_str(),"target_ref":target_id}));
                add_source(&mut graph, &id, declaration_source);
            }
            Declaration::State {
                owner,
                states,
                initial,
            } => {
                let Some(owner_id) = entity_names.get(&owner.value) else {
                    return Err(one(Diagnostic::source(
                        "APLS-E1207",
                        "state model owner must name a declared entity",
                        path,
                        source,
                        owner.span,
                    )));
                };
                if states.len() < 2 {
                    return Err(one(Diagnostic::source(
                        "APLS-E1103",
                        "state model requires at least two states",
                        path,
                        source,
                        record.span,
                    )));
                }
                let mut state_map = BTreeMap::new();
                let mut state_values = Vec::new();
                for state in states {
                    if state_map.contains_key(&state.value) {
                        return Err(one(Diagnostic::source(
                            "APLS-E1202",
                            "state names must be unique inside their owner",
                            path,
                            source,
                            state.span,
                        )));
                    }
                    let id = format!("state:{}:{}", owner.value, state.value);
                    state_map.insert(state.value.clone(), id.clone());
                    state_values.push(json!({"id":id,"display_name":state.value}));
                    add_source(
                        &mut graph,
                        &id,
                        provenance(
                            record.span,
                            vec![("sentence", record.span), ("state", state.span)],
                        ),
                    );
                }
                let Some(initial_ref) = state_map.get(&initial.value).cloned() else {
                    return Err(one(Diagnostic::source(
                        "APLS-E1207",
                        "initial state must occur in the declared state list",
                        path,
                        source,
                        initial.span,
                    )));
                };
                state_values.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
                let id = format!("state-model:{}", owner.value);
                graph.state_models.insert(id.clone(), json!({"id":id,"owner_ref":owner_id,"states":state_values,"initial_state_ref":initial_ref}));
                graph.state_info.insert(
                    id.clone(),
                    StateModelInfo {
                        owner_ref: owner_id.clone(),
                        states: state_map,
                        initial_ref,
                    },
                );
                add_source(&mut graph, &id, declaration_source);
            }
        }
    }
    for (id, value) in &graph.entities {
        graph.symbols.push(symbol(
            id,
            value["display_name"].as_str().unwrap(),
            SymbolKind::Entity,
            None,
            None,
        ));
    }
    for (id, value) in &graph.properties {
        graph.symbols.push(symbol(
            id,
            value["display_name"].as_str().unwrap(),
            SymbolKind::Property,
            None,
            None,
        ));
    }
    for (id, value) in &graph.actions {
        graph.symbols.push(symbol(
            id,
            value["display_name"].as_str().unwrap(),
            SymbolKind::Action,
            None,
            value["target_ref"].as_str().map(Into::into),
        ));
    }
    for (id, value) in &graph.events {
        graph.symbols.push(symbol(
            id,
            value["display_name"].as_str().unwrap(),
            SymbolKind::Event,
            None,
            None,
        ));
    }
    for (id, value) in &graph.units {
        graph.symbols.push(symbol(
            id,
            value["display_name"].as_str().unwrap(),
            SymbolKind::Unit,
            None,
            None,
        ));
    }
    for (name, id) in &builtin_units {
        graph
            .symbols
            .push(symbol(id, name, SymbolKind::Unit, None, None));
    }
    for model in graph.state_info.values() {
        for (name, id) in &model.states {
            graph.symbols.push(symbol(
                id,
                name,
                SymbolKind::State,
                Some(model.owner_ref.clone()),
                None,
            ));
        }
    }
    for alias in graph.aliases.values() {
        let target = alias["target_ref"].as_str().unwrap();
        let kind = match alias["target_kind"].as_str().unwrap() {
            "entity" => SymbolKind::Entity,
            "property" => SymbolKind::Property,
            "action" => SymbolKind::Action,
            "event" => SymbolKind::Event,
            "unit" => SymbolKind::Unit,
            _ => unreachable!(),
        };
        let base = graph.symbols.iter().find(|s| s.id == target).cloned();
        if let Some(mut reference) = base {
            reference.display_name = alias["display_name"].as_str().unwrap().into();
            graph.symbols.push(reference);
        } else if kind == SymbolKind::Unit {
            graph.symbols.push(symbol(
                target,
                alias["display_name"].as_str().unwrap(),
                kind,
                None,
                None,
            ));
        }
    }
    graph.symbols.sort();
    graph.symbols.dedup();
    Ok(graph)
}

fn declaration_header(value: &Declaration) -> (SymbolKind, String, ByteSpan) {
    match value {
        Declaration::Entity { name, .. } => (SymbolKind::Entity, name.value.clone(), name.span),
        Declaration::Unit { name } => (SymbolKind::Unit, name.value.clone(), name.span),
        Declaration::Property { name, .. } => (SymbolKind::Property, name.value.clone(), name.span),
        Declaration::Action { name, .. } => (SymbolKind::Action, name.value.clone(), name.span),
        Declaration::Event { name } => (SymbolKind::Event, name.value.clone(), name.span),
        Declaration::Alias { alias, kind, .. } => {
            (kind.value.symbol_kind(), alias.value.clone(), alias.span)
        }
        Declaration::State { owner, .. } => (SymbolKind::State, owner.value.clone(), owner.span),
    }
}

fn property_unit(
    value_type: ValueType,
    raw: Option<&str>,
    nominal: &BTreeMap<String, String>,
    builtin: &BTreeMap<String, String>,
) -> Result<Option<String>, &'static str> {
    let nominal_id = raw.and_then(|name| nominal.get(name)).cloned();
    let builtin_id = raw.and_then(|name| builtin.get(name)).cloned();
    match value_type {
        ValueType::Boolean | ValueType::Text if raw.is_none() => Ok(None),
        ValueType::Integer if raw.is_none() => Ok(None),
        ValueType::Integer if nominal_id.is_some() => Ok(nominal_id),
        ValueType::Decimal if raw.is_none() => Ok(None),
        ValueType::Decimal if raw == Some("摄氏度") => Ok(Some("unit:摄氏度".into())),
        ValueType::Decimal if nominal_id.is_some() => Ok(nominal_id),
        ValueType::Percentage if raw.is_none() || raw == Some("%") => Ok(Some("unit:%".into())),
        ValueType::Duration if raw.is_none() || matches!(raw, Some("毫秒" | "秒" | "分钟")) => {
            Ok(Some("unit:毫秒".into()))
        }
        _ => {
            let _ = builtin_id;
            Err("property type and declared unit are incompatible")
        }
    }
}

fn normative_spacing_error(sentence: &NormativeSentence, source: &str) -> Option<ByteSpan> {
    fn condition_error(condition: &Condition, source: &str) -> Option<ByteSpan> {
        condition.atoms.iter().find_map(|atom| match atom {
            AtomicCondition::Comparison { right, .. }
                if !literal_spacing_is_valid(right, source) =>
            {
                Some(right.span)
            }
            _ => None,
        })
    }

    match sentence {
        NormativeSentence::Rule { condition, .. }
        | NormativeSentence::Invariant { condition, .. } => condition_error(condition, source),
        NormativeSentence::Transition { trigger, .. } => condition_error(trigger, source),
        NormativeSentence::Acceptance {
            trigger,
            expected,
            deadline,
        } => condition_error(trigger, source)
            .or_else(|| condition_error(expected, source))
            .or_else(|| (!literal_spacing_is_valid(deadline, source)).then_some(deadline.span)),
        NormativeSentence::Informative { .. } => None,
    }
}

fn process_candidate(
    candidate: NormativeSentence,
    sentence_index: usize,
    span: ByteSpan,
    graph: &Graph,
    path: &str,
    source: &str,
    ledger: &mut Ledger,
) -> Result<Frame, Diagnostic> {
    bind_candidate(&candidate, graph, path, source)?;
    ledger.add(
        Resource::BoundFrames,
        1,
        Some(sentence_index),
        Some(span),
        path,
        source,
    )?;
    type_candidate(&candidate, span, graph, path, source)?;
    ledger.add(
        Resource::TypedFrames,
        1,
        Some(sentence_index),
        Some(span),
        path,
        source,
    )?;
    let frame = normalize_candidate(candidate, span, graph, path, source)?;
    ledger.add(
        Resource::CanonicalFrames,
        1,
        Some(sentence_index),
        Some(span),
        path,
        source,
    )?;
    Ok(frame)
}

fn bind_candidate(
    sentence: &NormativeSentence,
    graph: &Graph,
    path: &str,
    source: &str,
) -> Result<(), Diagnostic> {
    let missing = |message: &'static str, span: ByteSpan| {
        Diagnostic::source("APLS-E1204", message, path, source, span)
    };
    let bind_actor = |actor: &ActorRef| match actor {
        ActorRef::System(_) => Ok(()),
        ActorRef::Entity(entity) if graph.entities.contains_key(&entity.id) => Ok(()),
        ActorRef::Entity(entity) => Err(missing("entity reference is not declared", entity.span)),
    };
    let bind_condition = |condition: &Condition| -> Result<(), Diagnostic> {
        for atom in &condition.atoms {
            match atom {
                AtomicCondition::Comparison {
                    property, right, ..
                } => {
                    if !graph.property_info.contains_key(&property.id) {
                        return Err(missing("property reference is not declared", property.span));
                    }
                    if let RawLiteralKind::Quantity { unit, .. } = &right.kind {
                        let builtin = matches!(
                            unit.id.as_str(),
                            "unit:%" | "unit:毫秒" | "unit:秒" | "unit:分钟" | "unit:摄氏度"
                        );
                        if !builtin && !graph.units.contains_key(&unit.id) {
                            return Err(missing("unit reference is not declared", unit.span));
                        }
                    }
                }
                AtomicCondition::State { entity, state, .. } => {
                    if !graph.entities.contains_key(&entity.id) {
                        return Err(missing("entity reference is not declared", entity.span));
                    }
                    state_owner_matches(entity, state, path, source)?;
                }
                AtomicCondition::Event {
                    receiver, event, ..
                } => {
                    bind_actor(receiver)?;
                    if !graph.events.contains_key(&event.id) {
                        return Err(missing("event reference is not declared", event.span));
                    }
                }
            }
        }
        Ok(())
    };

    match sentence {
        NormativeSentence::Rule {
            condition,
            behavior,
            ..
        } => {
            bind_condition(condition)?;
            bind_actor(&behavior.actor)?;
            if !graph.action_targets.contains_key(&behavior.action.id) {
                return Err(missing(
                    "action reference is not declared",
                    behavior.action.span,
                ));
            }
            if !graph.entities.contains_key(&behavior.target.id) {
                return Err(missing(
                    "action target entity is not declared",
                    behavior.target.span,
                ));
            }
        }
        NormativeSentence::Transition {
            trigger,
            entity,
            source_state,
            target_state,
        } => {
            bind_condition(trigger)?;
            if !graph.entities.contains_key(&entity.id) {
                return Err(missing("entity reference is not declared", entity.span));
            }
            state_owner_matches(entity, source_state, path, source)?;
            state_owner_matches(entity, target_state, path, source)?;
        }
        NormativeSentence::Invariant {
            condition,
            required_entity,
            required_state,
        } => {
            bind_condition(condition)?;
            if !graph.entities.contains_key(&required_entity.id) {
                return Err(missing(
                    "entity reference is not declared",
                    required_entity.span,
                ));
            }
            state_owner_matches(required_entity, required_state, path, source)?;
        }
        NormativeSentence::Acceptance {
            trigger, expected, ..
        } => {
            bind_condition(trigger)?;
            bind_condition(expected)?;
        }
        NormativeSentence::Informative { .. } => {}
    }
    Ok(())
}

fn type_candidate(
    sentence: &NormativeSentence,
    span: ByteSpan,
    graph: &Graph,
    path: &str,
    source: &str,
) -> Result<(), Diagnostic> {
    let type_condition = |condition: &Condition| -> Result<(), Diagnostic> {
        for atom in &condition.atoms {
            if let AtomicCondition::Comparison {
                property,
                operator,
                right,
                ..
            } = atom
            {
                let info = graph.property_info.get(&property.id).ok_or_else(|| {
                    Diagnostic::source(
                        "APLS-E1204",
                        "property reference is not declared",
                        path,
                        source,
                        property.span,
                    )
                })?;
                typed_literal(info, operator.value, right, path, source)?;
            }
        }
        Ok(())
    };

    match sentence {
        NormativeSentence::Rule {
            condition,
            behavior,
            ..
        } => {
            type_condition(condition)?;
            let target = graph
                .action_targets
                .get(&behavior.action.id)
                .ok_or_else(|| {
                    Diagnostic::source(
                        "APLS-E1204",
                        "action reference is not declared",
                        path,
                        source,
                        behavior.action.span,
                    )
                })?;
            if target != &behavior.target.id {
                return Err(Diagnostic::source(
                    "APLS-E1206",
                    "action and target do not match the action declaration",
                    path,
                    source,
                    behavior.span,
                ));
            }
        }
        NormativeSentence::Transition {
            trigger,
            source_state,
            target_state,
            ..
        } => {
            type_condition(trigger)?;
            if source_state.id == target_state.id {
                return Err(Diagnostic::source(
                    "APLS-E1403",
                    "transition source and target states must differ",
                    path,
                    source,
                    span,
                ));
            }
        }
        NormativeSentence::Invariant { condition, .. } => type_condition(condition)?,
        NormativeSentence::Acceptance {
            trigger,
            expected,
            deadline,
        } => {
            type_condition(trigger)?;
            type_condition(expected)?;
            let typed_deadline = duration_literal(deadline, path, source)?;
            if !decimal_positive(typed_deadline["value"].as_str().unwrap()) {
                return Err(Diagnostic::source(
                    "APLS-E1307",
                    "acceptance deadline must be greater than zero",
                    path,
                    source,
                    deadline.span,
                ));
            }
        }
        NormativeSentence::Informative { .. } => {}
    }
    Ok(())
}

fn normalize_candidate(
    sentence: NormativeSentence,
    span: ByteSpan,
    graph: &Graph,
    path: &str,
    source: &str,
) -> Result<Frame, Diagnostic> {
    let provenance = normative_provenance(&sentence, span);
    let (kind, payload) = match sentence {
        NormativeSentence::Rule {
            condition,
            modality,
            behavior,
        } => {
            let condition = normalize_condition(condition, graph, path, source)?;
            let target = graph
                .action_targets
                .get(&behavior.action.id)
                .ok_or_else(|| {
                    Diagnostic::source(
                        "APLS-E1204",
                        "action reference is not declared",
                        path,
                        source,
                        behavior.action.span,
                    )
                })?;
            if target != &behavior.target.id {
                return Err(Diagnostic::source(
                    "APLS-E1206",
                    "action and target do not match the action declaration",
                    path,
                    source,
                    behavior.span,
                ));
            }
            (
                "rule",
                json!({"condition":condition,"modality":modality.value.as_str(),"behavior":{"actor_ref":behavior.actor.id(),"action_ref":behavior.action.id,"target_ref":behavior.target.id}}),
            )
        }
        NormativeSentence::Transition {
            trigger,
            entity,
            source_state,
            target_state,
        } => {
            state_owner_matches(&entity, &source_state, path, source)?;
            state_owner_matches(&entity, &target_state, path, source)?;
            if source_state.id == target_state.id {
                return Err(Diagnostic::source(
                    "APLS-E1403",
                    "transition source and target states must differ",
                    path,
                    source,
                    span,
                ));
            }
            (
                "transition",
                json!({"trigger":normalize_condition(trigger,graph,path,source)?,"entity_ref":entity.id,"source_state_ref":source_state.id,"target_state_ref":target_state.id}),
            )
        }
        NormativeSentence::Invariant {
            condition,
            required_entity,
            required_state,
        } => {
            state_owner_matches(&required_entity, &required_state, path, source)?;
            (
                "invariant",
                json!({"scope":"safety","condition":normalize_condition(condition,graph,path,source)?,"required_state":{"kind":"state_predicate","entity_ref":required_entity.id,"state_ref":required_state.id}}),
            )
        }
        NormativeSentence::Acceptance {
            trigger,
            expected,
            deadline,
        } => {
            let deadline_source_span = deadline.span;
            let deadline = duration_literal(&deadline, path, source)?;
            if !decimal_positive(deadline["value"].as_str().unwrap()) {
                return Err(Diagnostic::source(
                    "APLS-E1307",
                    "acceptance deadline must be greater than zero",
                    path,
                    source,
                    deadline_source_span,
                ));
            }
            (
                "acceptance",
                json!({"trigger":normalize_condition(trigger,graph,path,source)?,"expected":normalize_condition(expected,graph,path,source)?,"deadline":deadline}),
            )
        }
        NormativeSentence::Informative { text } => (
            "info",
            json!({"classification":"informative","kind":"note","text":text.value,"subject_ref":null}),
        ),
    };
    let id = anonymous_id(kind, &payload);
    let mut object = payload.as_object().expect("payload object").clone();
    object.insert("id".into(), Value::String(id));
    Ok(Frame {
        kind,
        value: Value::Object(object),
        provenance,
    })
}

fn normalize_condition(
    condition: Condition,
    graph: &Graph,
    path: &str,
    source: &str,
) -> Result<Value, Diagnostic> {
    let mut items = BTreeMap::<Vec<u8>, Value>::new();
    for atom in condition.atoms {
        let value = normalize_atom(atom, graph, path, source)?;
        items.insert(canonical_bytes(&value), value);
    }
    let values: Vec<_> = items.into_values().collect();
    Ok(if values.len() == 1 {
        values.into_iter().next().unwrap()
    } else {
        json!({"kind":"conjunction","items":values})
    })
}

fn normalize_atom(
    atom: AtomicCondition,
    graph: &Graph,
    path: &str,
    source: &str,
) -> Result<Value, Diagnostic> {
    match atom {
        AtomicCondition::Comparison {
            property,
            operator,
            right,
            ..
        } => {
            let info = graph.property_info.get(&property.id).ok_or_else(|| {
                Diagnostic::source(
                    "APLS-E1204",
                    "property reference is not declared",
                    path,
                    source,
                    property.span,
                )
            })?;
            let right = typed_literal(info, operator.value, &right, path, source)?;
            Ok(
                json!({"kind":"comparison","property_ref":property.id,"operator":operator.value.as_str(),"right":right}),
            )
        }
        AtomicCondition::State {
            entity,
            state,
            span: _,
        } => {
            state_owner_matches(&entity, &state, path, source)?;
            Ok(json!({"kind":"state_predicate","entity_ref":entity.id,"state_ref":state.id}))
        }
        AtomicCondition::Event {
            receiver, event, ..
        } => Ok(json!({"kind":"event_receipt","receiver_ref":receiver.id(),"event_ref":event.id})),
    }
}

fn typed_literal(
    property: &PropertyInfo,
    operator: ComparisonOp,
    literal: &RawLiteral,
    path: &str,
    source: &str,
) -> Result<Value, Diagnostic> {
    if !literal_spacing_is_valid(literal, source) {
        return Err(Diagnostic::source(
            "APLS-E1101",
            "literal spacing does not match the controlled grammar",
            path,
            source,
            literal.span,
        ));
    }
    let equality_only = matches!(property.value_type, ValueType::Boolean | ValueType::Text);
    if equality_only && !matches!(operator, ComparisonOp::Eq | ComparisonOp::Ne) {
        return Err(Diagnostic::source(
            "APLS-E1401",
            "property type does not allow an ordering comparison",
            path,
            source,
            literal.span,
        ));
    }
    let reject = || {
        Diagnostic::source(
            "APLS-E1401",
            "literal type or unit is incompatible with the property declaration",
            path,
            source,
            literal.span,
        )
    };
    match (
        &property.value_type,
        property.unit_ref.as_deref(),
        &literal.kind,
    ) {
        (ValueType::Boolean, None, RawLiteralKind::Boolean(value)) => {
            Ok(json!({"kind":"boolean","value":value}))
        }
        (ValueType::Integer, None, RawLiteralKind::Integer(value)) => {
            Ok(json!({"kind":"integer","value":canonical_integer(value)}))
        }
        (
            ValueType::Decimal,
            None,
            RawLiteralKind::Integer(value) | RawLiteralKind::Decimal(value),
        ) => Ok(json!({"kind":"decimal","value":canonical_decimal(value)})),
        (ValueType::Text, None, RawLiteralKind::Text(value)) => {
            Ok(json!({"kind":"text","value":value}))
        }
        (ValueType::Percentage, Some("unit:%"), RawLiteralKind::Percentage(value)) => Ok(quantity(
            "decimal",
            &canonical_decimal(value),
            "dimension:percentage",
            "unit:%",
        )),
        (ValueType::Decimal, Some("unit:摄氏度"), RawLiteralKind::Temperature(value)) => {
            Ok(quantity(
                "decimal",
                &canonical_decimal(value),
                "dimension:temperature",
                "unit:摄氏度",
            ))
        }
        (ValueType::Duration, Some("unit:毫秒"), RawLiteralKind::Duration { value, unit }) => {
            Ok(quantity(
                "decimal",
                &duration_millis(value, unit).ok_or_else(reject)?,
                "dimension:time",
                "unit:毫秒",
            ))
        }
        (ValueType::Integer, Some(unit_ref), RawLiteralKind::Quantity { value, unit })
            if unit.id == unit_ref && !value.contains('.') =>
        {
            Ok(quantity(
                "integer",
                &canonical_integer(value),
                &format!("dimension:nominal:{}", unit_ref.trim_start_matches("unit:")),
                unit_ref,
            ))
        }
        (ValueType::Decimal, Some(unit_ref), RawLiteralKind::Quantity { value, unit })
            if unit.id == unit_ref && unit_ref != "unit:摄氏度" =>
        {
            Ok(quantity(
                "decimal",
                &canonical_decimal(value),
                &format!("dimension:nominal:{}", unit_ref.trim_start_matches("unit:")),
                unit_ref,
            ))
        }
        _ => Err(reject()),
    }
}

fn duration_literal(literal: &RawLiteral, path: &str, source: &str) -> Result<Value, Diagnostic> {
    if !literal_spacing_is_valid(literal, source) {
        return Err(Diagnostic::source(
            "APLS-E1101",
            "duration spacing does not match the controlled grammar",
            path,
            source,
            literal.span,
        ));
    }
    match &literal.kind {
        RawLiteralKind::Duration { value, unit } => duration_millis(value, unit)
            .map(|v| quantity("decimal", &v, "dimension:time", "unit:毫秒"))
            .ok_or_else(|| {
                Diagnostic::source(
                    "APLS-E1401",
                    "invalid duration literal",
                    path,
                    source,
                    literal.span,
                )
            }),
        _ => Err(Diagnostic::source(
            "APLS-E1401",
            "acceptance deadline must be a duration",
            path,
            source,
            literal.span,
        )),
    }
}
fn literal_spacing_is_valid(literal: &RawLiteral, source: &str) -> bool {
    let gap = match &literal.kind {
        RawLiteralKind::Percentage(value) => {
            &source[literal.span.start_byte + value.len()..literal.span.end_byte - 1]
        }
        RawLiteralKind::Temperature(value) => {
            &source[literal.span.start_byte + value.len()..literal.span.end_byte - "摄氏度".len()]
        }
        RawLiteralKind::Duration { value, unit } => {
            &source[literal.span.start_byte + value.len()..literal.span.end_byte - unit.len()]
        }
        RawLiteralKind::Quantity { value, unit } => {
            let gap = &source[literal.span.start_byte + value.len()..unit.span.start_byte];
            return grammar_ws1(gap);
        }
        _ => return true,
    };
    gap.is_empty() || gap == " "
}
fn grammar_ws1(gap: &str) -> bool {
    let bytes = gap.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b' ' | b'\n' => index += 1,
            b'\r' if bytes.get(index + 1) == Some(&b'\n') => index += 2,
            _ => return false,
        }
    }
    !bytes.is_empty()
}
fn quantity(numeric_type: &str, value: &str, dimension: &str, unit: &str) -> Value {
    json!({"kind":"quantity","numeric_type":numeric_type,"value":value,"dimension_id":dimension,"canonical_unit_ref":unit})
}
fn duration_millis(value: &str, unit: &str) -> Option<String> {
    let factor = match unit {
        "毫秒" => 1,
        "s" => 1000,
        "秒" => 1000,
        "分钟" => 60000,
        _ => return None,
    };
    multiply_decimal(value, factor)
}
fn multiply_decimal(value: &str, factor: i64) -> Option<String> {
    let negative = value.starts_with('-');
    let value = value.trim_start_matches('-');
    let mut parts = value.split('.');
    let whole = parts.next()?;
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some() {
        return None;
    }
    let digits = format!("{whole}{fraction}");
    let mut number = digits.parse::<BigInt>().ok()? * BigInt::from(factor);
    if negative {
        number = -number;
    }
    let scale = fraction.len();
    let sign = if number.is_negative() { "-" } else { "" };
    let mut digits = number.abs().to_string();
    if scale > 0 {
        if digits.len() <= scale {
            digits = format!("{}{}", "0".repeat(scale + 1 - digits.len()), digits);
        }
        let split = digits.len() - scale;
        digits.insert(split, '.');
    }
    Some(canonical_decimal(&format!("{sign}{digits}")))
}
fn canonical_integer(value: &str) -> String {
    if value == "-0" {
        "0".into()
    } else {
        value.into()
    }
}
fn canonical_decimal(value: &str) -> String {
    let negative = value.starts_with('-');
    let mut body = value.trim_start_matches('-').to_owned();
    if body.contains('.') {
        while body.ends_with('0') {
            body.pop();
        }
        if body.ends_with('.') {
            body.pop();
        }
    }
    if body.chars().all(|c| c == '0' || c == '.') {
        return "0".into();
    }
    if negative { format!("-{body}") } else { body }
}
fn decimal_positive(value: &str) -> bool {
    !value.starts_with('-') && canonical_decimal(value) != "0"
}

fn state_owner_matches(
    entity: &SymbolRef,
    state: &SymbolRef,
    path: &str,
    source: &str,
) -> Result<(), Diagnostic> {
    if state.owner_ref.as_deref() == Some(entity.id.as_str()) {
        Ok(())
    } else {
        Err(Diagnostic::source(
            "APLS-E1207",
            "state does not belong to the referenced entity",
            path,
            source,
            state.span,
        ))
    }
}

fn validate_transitions(
    graph: &Graph,
    transitions: &[(Value, Provenance)],
    path: &str,
    source: &str,
) -> Result<(), OrderedDiagnostics> {
    let mut diagnostics = Vec::new();
    let mut grouped = BTreeMap::<(String, String, Vec<u8>), BTreeMap<String, Vec<ByteSpan>>>::new();
    for (value, provenance) in transitions {
        grouped
            .entry((
                value["entity_ref"].as_str().unwrap().into(),
                value["source_state_ref"].as_str().unwrap().into(),
                canonical_bytes(&value["trigger"]),
            ))
            .or_default()
            .entry(value["target_state_ref"].as_str().unwrap().into())
            .or_default()
            .push(provenance.sentence);
    }
    for targets in grouped.values() {
        if targets.len() > 1 {
            let mut spans: Vec<_> = targets.values().flatten().copied().collect();
            spans.sort();
            spans.dedup();
            let mut d = Diagnostic::source(
                "APLS-E1403",
                "same transition trigger selects different target states",
                path,
                source,
                spans[0],
            );
            d.missing_or_ambiguous_roles.push("target_state".into());
            for span in spans.into_iter().skip(1) {
                d.related_source_spans
                    .push(crate::diagnostic::SourceSpan::from_bytes(
                        path, source, span,
                    ));
            }
            diagnostics.push(d);
        }
    }
    for model in graph.state_info.values() {
        let mut adjacent = BTreeMap::<String, BTreeSet<String>>::new();
        let mut transition_spans = Vec::new();
        for (value, provenance) in transitions
            .iter()
            .filter(|(v, _)| v["entity_ref"] == model.owner_ref)
        {
            transition_spans.push(provenance.sentence);
            adjacent
                .entry(value["source_state_ref"].as_str().unwrap().into())
                .or_default()
                .insert(value["target_state_ref"].as_str().unwrap().into());
        }
        let mut reached = BTreeSet::new();
        let mut queue = VecDeque::from([model.initial_ref.clone()]);
        while let Some(state) = queue.pop_front() {
            if reached.insert(state.clone()) {
                for next in adjacent.get(&state).into_iter().flatten() {
                    queue.push_back(next.clone());
                }
            }
        }
        transition_spans.sort();
        transition_spans.dedup();
        for id in model.states.values().filter(|id| !reached.contains(*id)) {
            let span = graph
                .declaration_sources
                .get(id)
                .and_then(|p| p.first())
                .and_then(|p| {
                    p.roles
                        .iter()
                        .find(|(role, _)| role == "state")
                        .map(|(_, span)| *span)
                })
                .unwrap_or(ByteSpan::new(0, source.len()));
            let mut diagnostic = Diagnostic::source(
                "APLS-E1402",
                "state is unreachable from the initial state",
                path,
                source,
                span,
            );
            diagnostic.related_source_spans.extend(
                transition_spans
                    .iter()
                    .copied()
                    .map(|span| crate::diagnostic::SourceSpan::from_bytes(path, source, span)),
            );
            diagnostics.push(diagnostic);
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn build_ir(
    bytes: &[u8],
    path: &str,
    source: &str,
    mut graph: Graph,
    frames: BTreeMap<&'static str, BTreeMap<String, (Value, BTreeSet<Provenance>)>>,
    transitions: Vec<(Value, Provenance)>,
) -> Result<VerifiedArtifact, OrderedDiagnostics> {
    validate_transitions(&graph, &transitions, path, source)?;
    let needs_percent = graph.properties.values().any(|v| v["unit_ref"] == "unit:%")
        || frames
            .values()
            .flatten()
            .any(|(_, (v, _))| canonical_bytes(v).windows(6).any(|w| w == b"unit:%"));
    let needs_time = graph
        .properties
        .values()
        .any(|v| v["unit_ref"] == "unit:毫秒")
        || frames.contains_key("acceptance");
    let needs_temp = graph
        .properties
        .values()
        .any(|v| v["unit_ref"] == "unit:摄氏度");
    if needs_percent {
        graph.units.insert(
            "unit:%".into(),
            builtin_unit("%", "dimension:percentage", "unit:%"),
        );
    }
    if needs_time {
        graph.units.insert(
            "unit:毫秒".into(),
            builtin_unit("毫秒", "dimension:time", "unit:毫秒"),
        );
    }
    if needs_temp {
        graph.units.insert(
            "unit:摄氏度".into(),
            builtin_unit("摄氏度", "dimension:temperature", "unit:摄氏度"),
        );
    }
    let entities = values(&graph.entities);
    let units = values(&graph.units);
    let properties = values(&graph.properties);
    let actions = values(&graph.actions);
    let events = values(&graph.events);
    let aliases = values(&graph.aliases);
    let state_models = values(&graph.state_models);
    let rules = frame_values(&frames, "rule");
    let transition_values = frame_values(&frames, "transition");
    let invariants = frame_values(&frames, "invariant");
    let acceptance = frame_values(&frames, "acceptance");
    let info = frame_values(&frames, "info");
    let projection = json!({"language_profile":"apls-zh-CN-0.1","ir_schema_version":"apls-cnl-ir-0.1","entities":entities,"units":units,"properties":properties,"actions":actions,"events":events,"state_models":state_models,"rules":rules,"transitions":transition_values,"invariants":invariants,"acceptance":acceptance});
    let semantic_hash = domain_hash(
        "APLS-CNL-DOCUMENT-SEMANTIC-0.1",
        &canonical_bytes(&projection),
    );
    let source_id = format!("source:{path}");
    let source_hash = hex_digest(bytes);
    let mut ir = json!({"header":{"language_profile":"apls-zh-CN-0.1","ir_schema_version":"apls-cnl-ir-0.1","canonicalization_profile":"APLS-CNL-C14N-0.1","compiler_id":"apls","compiler_version":"0.1.0","unicode_data_version":format!("{}.{}.{}",UNICODE_DATA_VERSION.0,UNICODE_DATA_VERSION.1,UNICODE_DATA_VERSION.2),"unicode_normalization":"NFC","semantic_hash_algorithm":"sha256","semantic_hash":format!("sha256:{semantic_hash}"),"status":"verified"},
        "source_manifest":{"files":[{"id":source_id,"logical_path":path,"sha256":source_hash}]},"entities":entities,"units":units,"properties":properties,"actions":actions,"events":events,"surface_aliases":aliases,"state_models":state_models,"rules":rules,"transitions":transition_values,"invariants":invariants,"acceptance":acceptance,"informative_items":info,"source_map":[]});
    let source_map = build_source_map(&ir, &graph.declaration_sources, &frames, &source_id);
    ir["source_map"] = Value::Array(source_map);
    let output = canonical_bytes(&ir);
    if output.len() > crate::limits::MAX_CANONICAL_IR_BYTES {
        return Err(one(canonical_ir_limit(output.len())));
    }
    let reparsed: Value = serde_json::from_slice(&output).map_err(|_| {
        one(Diagnostic::tool(
            "APLS-T0006",
            "canonical IR did not reparse",
        ))
    })?;
    let schema: Value = serde_json::from_str(IR_SCHEMA).map_err(|_| {
        one(Diagnostic::tool(
            "APLS-T0006",
            "embedded CNL IR schema is invalid",
        ))
    })?;
    let validator = jsonschema::validator_for(&schema).map_err(|_| {
        one(Diagnostic::tool(
            "APLS-T0006",
            "embedded CNL IR schema did not compile",
        ))
    })?;
    if !validator.is_valid(&reparsed) {
        return Err(one(Diagnostic::tool(
            "APLS-T0006",
            "generated CNL IR failed schema revalidation",
        )));
    }
    cross_validate(&reparsed, source).map_err(one)?;
    Ok(VerifiedArtifact::new(output))
}

fn build_source_map(
    ir: &Value,
    declarations: &BTreeMap<String, Vec<Provenance>>,
    frames: &BTreeMap<&'static str, BTreeMap<String, (Value, BTreeSet<Provenance>)>>,
    source_id: &str,
) -> Vec<Value> {
    let arrays = [
        ("entities", "entities"),
        ("units", "units"),
        ("properties", "properties"),
        ("actions", "actions"),
        ("events", "events"),
        ("surface_aliases", "surface_aliases"),
        ("state_models", "state_models"),
    ];
    let mut result = Vec::new();
    for (field, pointer) in arrays {
        for (index, node) in ir[field].as_array().unwrap().iter().enumerate() {
            let id = node["id"].as_str().unwrap();
            if let Some(provenance) = declarations.get(id) {
                result.push(source_entry(
                    id,
                    &format!("/{pointer}/{index}"),
                    provenance.iter().cloned(),
                    source_id,
                ));
            }
            if field == "state_models" {
                for (state_index, state) in node["states"].as_array().unwrap().iter().enumerate() {
                    let state_id = state["id"].as_str().unwrap();
                    if let Some(provenance) = declarations.get(state_id) {
                        result.push(source_entry(
                            state_id,
                            &format!("/{pointer}/{index}/states/{state_index}"),
                            provenance.iter().cloned(),
                            source_id,
                        ));
                    }
                }
            }
        }
    }
    for (kind, field) in [
        ("rule", "rules"),
        ("transition", "transitions"),
        ("invariant", "invariants"),
        ("acceptance", "acceptance"),
        ("info", "informative_items"),
    ] {
        if let Some(group) = frames.get(kind) {
            for (index, node) in ir[field].as_array().unwrap().iter().enumerate() {
                let id = node["id"].as_str().unwrap();
                let provenance = &group[id].1;
                result.push(source_entry(
                    id,
                    &format!("/{field}/{index}"),
                    provenance.iter().cloned(),
                    source_id,
                ));
            }
        }
    }
    result.sort_by(|a, b| a["semantic_id"].as_str().cmp(&b["semantic_id"].as_str()));
    result
}
fn source_entry(
    id: &str,
    pointer: &str,
    items: impl Iterator<Item = Provenance>,
    source_id: &str,
) -> Value {
    let mut by_sentence = BTreeMap::<ByteSpan, BTreeSet<(String, ByteSpan)>>::new();
    for item in items {
        by_sentence
            .entry(item.sentence)
            .or_default()
            .extend(item.roles);
    }
    let provenance = by_sentence
        .into_iter()
        .map(|(sentence, roles)| {
            json!({
                "source_id": source_id,
                "sentence_span": {
                    "start_byte": sentence.start_byte,
                    "end_byte": sentence.end_byte
                },
                "role_spans": roles.into_iter().map(|(role, span)| json!({
                    "role": role,
                    "span": {
                        "start_byte": span.start_byte,
                        "end_byte": span.end_byte
                    }
                })).collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();
    json!({"semantic_id":id,"ir_pointer":pointer,"provenance":provenance})
}
fn cross_validate(ir: &Value, source: &str) -> Result<(), Diagnostic> {
    let failure = |message| Diagnostic::tool("APLS-T0006", message);
    if ir["header"]["unicode_data_version"] != "17.0.0"
        || ir["header"]["unicode_normalization"] != "NFC"
    {
        return Err(failure("Unicode normalization header is invalid"));
    }
    let source_file = &ir["source_manifest"]["files"][0];
    if source_file["id"] != format!("source:{}", source_file["logical_path"].as_str().unwrap())
        || source_file["sha256"] != hex_digest(source.as_bytes())
    {
        return Err(failure("source manifest identity or digest is invalid"));
    }
    let mut ids = BTreeSet::<String>::new();
    let mut expected_source_ids = BTreeSet::<String>::new();
    let entity_names: BTreeMap<_, _> = ir["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| {
            (
                node["id"].as_str().unwrap().to_owned(),
                node["display_name"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    for field in [
        "entities",
        "units",
        "properties",
        "actions",
        "events",
        "surface_aliases",
        "state_models",
        "rules",
        "transitions",
        "invariants",
        "acceptance",
        "informative_items",
    ] {
        let mut previous: Option<&str> = None;
        for node in ir[field].as_array().unwrap() {
            let id = node["id"].as_str().unwrap();
            if previous.is_some_and(|value| value.as_bytes() >= id.as_bytes()) {
                return Err(failure("IR node arrays are not strictly ID-sorted"));
            }
            previous = Some(id);
            if !ids.insert(id.to_owned()) {
                return Err(failure("duplicate semantic ID in verified IR"));
            }
            if !(field == "units" && node["unit_kind"] == "builtin") {
                expected_source_ids.insert(id.to_owned());
            }
            if field == "state_models" {
                let owner = node["owner_ref"].as_str().unwrap();
                let owner_name = entity_names
                    .get(owner)
                    .ok_or_else(|| failure("state-model owner reference is invalid"))?;
                if id != format!("state-model:{owner_name}") {
                    return Err(failure("state-model semantic ID is invalid"));
                }
                let mut state_ids = BTreeSet::new();
                for state in node["states"].as_array().unwrap() {
                    let state_id = state["id"].as_str().unwrap();
                    if state_id
                        != format!(
                            "state:{owner_name}:{}",
                            state["display_name"].as_str().unwrap()
                        )
                    {
                        return Err(failure("state semantic ID is invalid"));
                    }
                    if !ids.insert(state_id.to_owned()) || !state_ids.insert(state_id.to_owned()) {
                        return Err(failure("duplicate state ID in verified IR"));
                    }
                    expected_source_ids.insert(state_id.to_owned());
                }
                if !state_ids.contains(node["initial_state_ref"].as_str().unwrap()) {
                    return Err(failure("initial state reference is invalid"));
                }
            }
        }
    }
    for node in ir["entities"].as_array().unwrap() {
        if node["id"] != format!("entity:{}", node["display_name"].as_str().unwrap()) {
            return Err(failure("entity semantic ID is invalid"));
        }
    }
    for node in ir["properties"].as_array().unwrap() {
        if node["id"] != format!("property:{}", node["display_name"].as_str().unwrap()) {
            return Err(failure("property semantic ID is invalid"));
        }
    }
    for node in ir["events"].as_array().unwrap() {
        if node["id"] != format!("event:{}", node["display_name"].as_str().unwrap()) {
            return Err(failure("event semantic ID is invalid"));
        }
    }
    for node in ir["units"].as_array().unwrap() {
        if node["id"] != format!("unit:{}", node["display_name"].as_str().unwrap()) {
            return Err(failure("unit semantic ID is invalid"));
        }
    }
    for node in ir["actions"].as_array().unwrap() {
        let target_name = entity_names
            .get(node["target_ref"].as_str().unwrap())
            .ok_or_else(|| failure("action target reference is invalid"))?;
        if node["id"]
            != format!(
                "action:{target_name}:{}",
                node["display_name"].as_str().unwrap()
            )
        {
            return Err(failure("action semantic ID is invalid"));
        }
    }
    let entity_ids: BTreeSet<_> = ir["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect();
    let unit_ids: BTreeSet<_> = ir["units"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect();
    let unit_info: BTreeMap<_, _> = ir["units"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| {
            (
                node["id"].as_str().unwrap(),
                (
                    node["unit_kind"].as_str().unwrap(),
                    node["dimension_id"].as_str().unwrap(),
                ),
            )
        })
        .collect();
    let property_ids: BTreeSet<_> = ir["properties"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect();
    let action_targets: BTreeMap<_, _> = ir["actions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| {
            (
                node["id"].as_str().unwrap(),
                node["target_ref"].as_str().unwrap(),
            )
        })
        .collect();
    let event_ids: BTreeSet<_> = ir["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect();
    let mut state_owners = BTreeMap::new();
    for model in ir["state_models"].as_array().unwrap() {
        let owner = model["owner_ref"].as_str().unwrap();
        if !entity_ids.contains(owner) {
            return Err(failure("state-model owner reference is invalid"));
        }
        for state in model["states"].as_array().unwrap() {
            state_owners.insert(state["id"].as_str().unwrap(), owner);
        }
    }
    for unit in ir["units"].as_array().unwrap() {
        let id = unit["id"].as_str().unwrap();
        let canonical = unit["canonical_unit_ref"].as_str().unwrap();
        if !unit_ids.contains(canonical) {
            return Err(failure("unit canonical reference is invalid"));
        }
        if unit["unit_kind"] == "nominal"
            && (canonical != id
                || unit["dimension_id"]
                    != format!(
                        "dimension:nominal:{}",
                        unit["display_name"].as_str().unwrap()
                    )
                || unit["scale"] != "1"
                || unit["offset"] != "0")
        {
            return Err(failure("nominal unit identity is invalid"));
        }
        if unit["unit_kind"] == "builtin" {
            let expected = match id {
                "unit:%" => ("dimension:percentage", "unit:%"),
                "unit:摄氏度" => ("dimension:temperature", "unit:摄氏度"),
                "unit:毫秒" => ("dimension:time", "unit:毫秒"),
                _ => return Err(failure("unknown built-in unit")),
            };
            if unit["dimension_id"] != expected.0
                || unit["canonical_unit_ref"] != expected.1
                || unit["scale"] != "1"
                || unit["offset"] != "0"
            {
                return Err(failure("built-in unit materialization is invalid"));
            }
        }
    }
    for property in ir["properties"].as_array().unwrap() {
        if let Some(unit_ref) = property["unit_ref"].as_str() {
            if !unit_ids.contains(unit_ref) {
                return Err(failure("property unit reference is invalid"));
            }
        }
        validate_ir_property(property, &unit_info).map_err(&failure)?;
    }
    let property_info: BTreeMap<_, _> = ir["properties"]
        .as_array()
        .unwrap()
        .iter()
        .map(|property| (property["id"].as_str().unwrap(), property))
        .collect();
    for node in ir["surface_aliases"].as_array().unwrap() {
        let target = node["target_ref"].as_str().unwrap();
        let category_matches = match node["target_kind"].as_str().unwrap() {
            "entity" => entity_ids.contains(target),
            "property" => property_ids.contains(target),
            "action" => action_targets.contains_key(target),
            "event" => event_ids.contains(target),
            "unit" => unit_ids.contains(target),
            _ => false,
        };
        if node["id"]
            != format!(
                "alias:{}:{}",
                node["target_kind"].as_str().unwrap(),
                node["display_name"].as_str().unwrap()
            )
            || !category_matches
        {
            return Err(failure("surface alias identity or target is invalid"));
        }
    }
    for (field, kind) in [
        ("rules", "rule"),
        ("transitions", "transition"),
        ("invariants", "invariant"),
        ("acceptance", "acceptance"),
        ("informative_items", "info"),
    ] {
        for node in ir[field].as_array().unwrap() {
            let mut payload = node.as_object().unwrap().clone();
            payload.remove("id");
            if node["id"] != anonymous_id(kind, &Value::Object(payload)) {
                return Err(failure("anonymous semantic ID is invalid"));
            }
        }
    }
    for rule in ir["rules"].as_array().unwrap() {
        validate_ir_condition(
            &rule["condition"],
            &entity_ids,
            &unit_ids,
            &property_info,
            &unit_info,
            &event_ids,
            &state_owners,
        )
        .map_err(&failure)?;
        let behavior = &rule["behavior"];
        let actor = behavior["actor_ref"].as_str().unwrap();
        let action = behavior["action_ref"].as_str().unwrap();
        let target = behavior["target_ref"].as_str().unwrap();
        if (actor != "builtin:system" && !entity_ids.contains(actor))
            || action_targets.get(action).copied() != Some(target)
            || !entity_ids.contains(target)
        {
            return Err(failure("rule behavior reference is invalid"));
        }
    }
    for transition in ir["transitions"].as_array().unwrap() {
        validate_ir_condition(
            &transition["trigger"],
            &entity_ids,
            &unit_ids,
            &property_info,
            &unit_info,
            &event_ids,
            &state_owners,
        )
        .map_err(&failure)?;
        let entity = transition["entity_ref"].as_str().unwrap();
        let source_state = transition["source_state_ref"].as_str().unwrap();
        let target_state = transition["target_state_ref"].as_str().unwrap();
        if state_owners.get(source_state).copied() != Some(entity)
            || state_owners.get(target_state).copied() != Some(entity)
        {
            return Err(failure("transition state reference is invalid"));
        }
        if source_state == target_state {
            return Err(failure("transition source and target states must differ"));
        }
    }
    let mut transition_targets = BTreeMap::<(&str, &str, Vec<u8>), BTreeSet<&str>>::new();
    for transition in ir["transitions"].as_array().unwrap() {
        transition_targets
            .entry((
                transition["entity_ref"].as_str().unwrap(),
                transition["source_state_ref"].as_str().unwrap(),
                canonical_bytes(&transition["trigger"]),
            ))
            .or_default()
            .insert(transition["target_state_ref"].as_str().unwrap());
    }
    if transition_targets.values().any(|targets| targets.len() > 1) {
        return Err(failure(
            "same transition trigger selects different target states",
        ));
    }
    for model in ir["state_models"].as_array().unwrap() {
        let owner = model["owner_ref"].as_str().unwrap();
        let mut adjacent = BTreeMap::<&str, BTreeSet<&str>>::new();
        for transition in ir["transitions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|transition| transition["entity_ref"] == owner)
        {
            adjacent
                .entry(transition["source_state_ref"].as_str().unwrap())
                .or_default()
                .insert(transition["target_state_ref"].as_str().unwrap());
        }
        let mut reached = BTreeSet::new();
        let mut queue = VecDeque::from([model["initial_state_ref"].as_str().unwrap()]);
        while let Some(state) = queue.pop_front() {
            if reached.insert(state) {
                queue.extend(adjacent.get(state).into_iter().flatten().copied());
            }
        }
        if model["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|state| !reached.contains(state["id"].as_str().unwrap()))
        {
            return Err(failure("state is unreachable from the initial state"));
        }
    }
    for invariant in ir["invariants"].as_array().unwrap() {
        validate_ir_condition(
            &invariant["condition"],
            &entity_ids,
            &unit_ids,
            &property_info,
            &unit_info,
            &event_ids,
            &state_owners,
        )
        .map_err(&failure)?;
        validate_ir_condition(
            &invariant["required_state"],
            &entity_ids,
            &unit_ids,
            &property_info,
            &unit_info,
            &event_ids,
            &state_owners,
        )
        .map_err(&failure)?;
    }
    for acceptance in ir["acceptance"].as_array().unwrap() {
        for key in ["trigger", "expected"] {
            validate_ir_condition(
                &acceptance[key],
                &entity_ids,
                &unit_ids,
                &property_info,
                &unit_info,
                &event_ids,
                &state_owners,
            )
            .map_err(&failure)?;
        }
        validate_ir_deadline(&acceptance["deadline"], &unit_ids, &unit_info).map_err(&failure)?;
    }
    let mut materialized_units: BTreeSet<&str> = ir["units"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|unit| unit["unit_kind"] == "nominal")
        .map(|unit| unit["id"].as_str().unwrap())
        .collect();
    for property in ir["properties"].as_array().unwrap() {
        if let Some(unit_ref) = property["unit_ref"].as_str() {
            if unit_info
                .get(unit_ref)
                .is_some_and(|(kind, _)| *kind == "builtin")
            {
                materialized_units.insert(unit_ref);
            }
        }
    }
    for field in ["rules", "transitions", "invariants", "acceptance"] {
        for node in ir[field].as_array().unwrap() {
            collect_quantity_units(node, &unit_info, &mut materialized_units);
        }
    }
    if materialized_units != unit_ids {
        return Err(failure("unit materialization closure is not exact"));
    }
    let source_id = ir["source_manifest"]["files"][0]["id"].as_str().unwrap();
    let allowed_roles: BTreeSet<&str> = [
        "sentence",
        "name",
        "entity_kind",
        "value_type",
        "observable",
        "writable",
        "unit",
        "alias",
        "target_kind",
        "target",
        "owner",
        "state",
        "initial_state",
        "condition",
        "condition_left",
        "condition_operator",
        "condition_right",
        "condition_entity",
        "condition_state",
        "condition_receiver",
        "condition_event",
        "modality",
        "behavior",
        "behavior_actor",
        "behavior_action",
        "behavior_target",
        "trigger",
        "trigger_left",
        "trigger_operator",
        "trigger_right",
        "trigger_entity",
        "trigger_state",
        "trigger_receiver",
        "trigger_event",
        "entity",
        "source_state",
        "target_state",
        "scope",
        "required_state",
        "required_state_entity",
        "required_state_state",
        "expected",
        "expected_left",
        "expected_operator",
        "expected_right",
        "expected_entity",
        "expected_state",
        "expected_receiver",
        "expected_event",
        "deadline",
        "text",
    ]
    .into_iter()
    .collect();
    let mut mapped = BTreeSet::new();
    let mut previous_semantic_id: Option<&str> = None;
    for entry in ir["source_map"].as_array().unwrap() {
        let semantic_id = entry["semantic_id"].as_str().unwrap();
        if previous_semantic_id
            .is_some_and(|previous| previous.as_bytes() >= semantic_id.as_bytes())
        {
            return Err(failure("source-map entries are not strictly ID-sorted"));
        }
        previous_semantic_id = Some(semantic_id);
        if !mapped.insert(semantic_id.to_owned()) {
            return Err(failure("duplicate source-map semantic ID"));
        }
        let pointer = entry["ir_pointer"].as_str().unwrap();
        if ir.pointer(pointer).and_then(|node| node["id"].as_str()) != Some(semantic_id) {
            return Err(failure(
                "source-map pointer does not identify its semantic node",
            ));
        }
        let mut previous_provenance = None;
        for p in entry["provenance"].as_array().unwrap() {
            if p["source_id"] != source_id {
                return Err(failure("source-map source ID is inconsistent"));
            }
            let span = &p["sentence_span"];
            let start = span["start_byte"].as_u64().unwrap() as usize;
            let end = span["end_byte"].as_u64().unwrap() as usize;
            if start >= end
                || end > source.len()
                || !source.is_char_boundary(start)
                || !source.is_char_boundary(end)
                || !source[start..end].ends_with('。')
                || source[start..end]
                    .chars()
                    .next()
                    .is_some_and(|ch| matches!(ch, ' ' | '\n' | '\r'))
            {
                return Err(failure("invalid source-map byte span"));
            }
            let provenance_key = (
                p["source_id"].as_str().unwrap(),
                start,
                end,
                canonical_bytes(&p["role_spans"]),
            );
            if previous_provenance
                .as_ref()
                .is_some_and(|previous| previous >= &provenance_key)
            {
                return Err(failure("source-map provenance is not strictly sorted"));
            }
            previous_provenance = Some(provenance_key);
            let mut previous_role: Option<(&str, usize, usize)> = None;
            let mut has_sentence_role = false;
            for role in p["role_spans"].as_array().unwrap() {
                if !allowed_roles.contains(role["role"].as_str().unwrap()) {
                    return Err(failure("unknown source-map role"));
                }
                let role_name = role["role"].as_str().unwrap();
                let role_start = role["span"]["start_byte"].as_u64().unwrap() as usize;
                let role_end = role["span"]["end_byte"].as_u64().unwrap() as usize;
                if role_start < start
                    || role_start >= role_end
                    || role_end > end
                    || !source.is_char_boundary(role_start)
                    || !source.is_char_boundary(role_end)
                {
                    return Err(failure("invalid source-map role span"));
                }
                let role_key = (role_name, role_start, role_end);
                if previous_role.is_some_and(|previous| previous >= role_key) {
                    return Err(failure("source-map role spans are not strictly sorted"));
                }
                previous_role = Some(role_key);
                if role_name == "sentence" && role_start == start && role_end == end {
                    has_sentence_role = true;
                }
            }
            if !has_sentence_role {
                return Err(failure("source-map sentence role is missing"));
            }
        }
    }
    if mapped != expected_source_ids {
        return Err(failure("source-map coverage is not exact"));
    }
    let projection = json!({"language_profile":ir["header"]["language_profile"],"ir_schema_version":ir["header"]["ir_schema_version"],"entities":ir["entities"],"units":ir["units"],"properties":ir["properties"],"actions":ir["actions"],"events":ir["events"],"state_models":ir["state_models"],"rules":ir["rules"],"transitions":ir["transitions"],"invariants":ir["invariants"],"acceptance":ir["acceptance"]});
    let expected_hash = format!(
        "sha256:{}",
        domain_hash(
            "APLS-CNL-DOCUMENT-SEMANTIC-0.1",
            &canonical_bytes(&projection)
        )
    );
    if ir["header"]["semantic_hash"] != expected_hash {
        return Err(failure("document semantic hash is invalid"));
    }
    for field in ["rules", "transitions", "invariants", "acceptance"] {
        for node in ir[field].as_array().unwrap() {
            for key in ["condition", "trigger", "expected"] {
                if let Some(condition) = node.get(key) {
                    validate_condition_order(condition).map_err(&failure)?;
                }
            }
        }
    }
    Ok(())
}

fn validate_ir_property(
    property: &Value,
    units: &BTreeMap<&str, (&str, &str)>,
) -> Result<(), &'static str> {
    let unit_ref = property["unit_ref"].as_str();
    let nominal = unit_ref.is_some_and(|unit| {
        units
            .get(unit)
            .is_some_and(|(unit_kind, _)| *unit_kind == "nominal")
    });
    let valid = match property["value_type"].as_str().unwrap() {
        "boolean" | "text" => unit_ref.is_none(),
        "integer" => unit_ref.is_none() || nominal,
        "decimal" => unit_ref.is_none() || nominal || unit_ref == Some("unit:摄氏度"),
        "percentage" => unit_ref == Some("unit:%"),
        "duration" => unit_ref == Some("unit:毫秒"),
        _ => false,
    };
    valid
        .then_some(())
        .ok_or("property type and unit are incompatible")
}

fn validate_ir_quantity<'a>(
    value: &Value,
    unit_ids: &BTreeSet<&'a str>,
    units: &BTreeMap<&'a str, (&'a str, &'a str)>,
) -> Result<(), &'static str> {
    let unit_ref = value["canonical_unit_ref"].as_str().unwrap();
    if !unit_ids.contains(unit_ref) {
        return Err("quantity canonical unit reference is invalid");
    }
    if match units.get(unit_ref) {
        Some((_, dimension)) => *dimension != value["dimension_id"],
        None => true,
    } {
        return Err("quantity dimension and canonical unit are incompatible");
    }
    Ok(())
}

fn validate_ir_comparison<'a>(
    property: &Value,
    operator: &str,
    right: &Value,
    unit_ids: &BTreeSet<&'a str>,
    units: &BTreeMap<&'a str, (&'a str, &'a str)>,
) -> Result<(), &'static str> {
    validate_ir_literal_value(right)?;
    let value_type = property["value_type"].as_str().unwrap();
    let unit_ref = property["unit_ref"].as_str();
    if matches!(value_type, "boolean" | "text") && !matches!(operator, "eq" | "ne") {
        return Err("property type does not allow the comparison operator");
    }
    let literal_matches = match (value_type, unit_ref, right["kind"].as_str().unwrap()) {
        ("boolean", None, "boolean") | ("text", None, "text") => true,
        ("integer", None, "integer") | ("decimal", None, "decimal") => true,
        ("integer", Some(unit), "quantity") => {
            right["numeric_type"] == "integer"
                && right["canonical_unit_ref"] == unit
                && units.get(unit).is_some_and(|(kind, _)| *kind == "nominal")
        }
        ("decimal", Some(unit), "quantity") => {
            right["numeric_type"] == "decimal" && right["canonical_unit_ref"] == unit
        }
        ("percentage", Some("unit:%"), "quantity")
        | ("duration", Some("unit:毫秒"), "quantity") => {
            right["numeric_type"] == "decimal" && right["canonical_unit_ref"] == unit_ref.unwrap()
        }
        _ => false,
    };
    if !literal_matches {
        return Err("comparison literal is incompatible with its property");
    }
    if right["kind"] == "quantity" {
        validate_ir_quantity(right, unit_ids, units)?;
    }
    Ok(())
}

fn validate_ir_literal_value(literal: &Value) -> Result<(), &'static str> {
    let canonical = match literal["kind"].as_str().unwrap() {
        "integer" => canonical_integer(literal["value"].as_str().unwrap()),
        "decimal" => canonical_decimal(literal["value"].as_str().unwrap()),
        "quantity" if literal["numeric_type"] == "integer" => {
            canonical_integer(literal["value"].as_str().unwrap())
        }
        "quantity" => canonical_decimal(literal["value"].as_str().unwrap()),
        "boolean" | "text" => return Ok(()),
        _ => return Err("unknown typed literal kind"),
    };
    (literal["value"].as_str() == Some(canonical.as_str()))
        .then_some(())
        .ok_or("typed literal value is not canonical")
}

fn validate_ir_deadline<'a>(
    deadline: &Value,
    unit_ids: &BTreeSet<&'a str>,
    units: &BTreeMap<&'a str, (&'a str, &'a str)>,
) -> Result<(), &'static str> {
    validate_ir_literal_value(deadline)?;
    validate_ir_quantity(deadline, unit_ids, units)?;
    if deadline["kind"] != "quantity"
        || deadline["numeric_type"] != "decimal"
        || deadline["dimension_id"] != "dimension:time"
        || deadline["canonical_unit_ref"] != "unit:毫秒"
        || !decimal_positive(deadline["value"].as_str().unwrap())
    {
        return Err("acceptance deadline is not a positive canonical time quantity");
    }
    Ok(())
}

fn collect_quantity_units<'a>(
    value: &'a Value,
    units: &BTreeMap<&'a str, (&'a str, &'a str)>,
    materialized: &mut BTreeSet<&'a str>,
) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_quantity_units(item, units, materialized);
            }
        }
        Value::Object(object) => {
            if value["kind"] == "quantity" {
                if let Some(unit_ref) = value["canonical_unit_ref"].as_str() {
                    if units
                        .get(unit_ref)
                        .is_some_and(|(kind, _)| *kind == "builtin")
                    {
                        materialized.insert(unit_ref);
                    }
                }
            }
            for child in object.values() {
                collect_quantity_units(child, units, materialized);
            }
        }
        _ => {}
    }
}

fn validate_ir_condition<'a>(
    condition: &'a Value,
    entity_ids: &BTreeSet<&'a str>,
    unit_ids: &BTreeSet<&'a str>,
    properties: &BTreeMap<&'a str, &'a Value>,
    units: &BTreeMap<&'a str, (&'a str, &'a str)>,
    event_ids: &BTreeSet<&'a str>,
    state_owners: &BTreeMap<&'a str, &'a str>,
) -> Result<(), &'static str> {
    match condition["kind"].as_str().unwrap() {
        "comparison" => {
            let property = properties
                .get(condition["property_ref"].as_str().unwrap())
                .ok_or("comparison property reference is invalid")?;
            validate_ir_comparison(
                property,
                condition["operator"].as_str().unwrap(),
                &condition["right"],
                unit_ids,
                units,
            )
        }
        "state_predicate" => {
            let entity = condition["entity_ref"].as_str().unwrap();
            if !entity_ids.contains(entity)
                || state_owners
                    .get(condition["state_ref"].as_str().unwrap())
                    .copied()
                    != Some(entity)
            {
                return Err("state predicate reference is invalid");
            }
            Ok(())
        }
        "event_receipt" => {
            let receiver = condition["receiver_ref"].as_str().unwrap();
            if (receiver != "builtin:system" && !entity_ids.contains(receiver))
                || !event_ids.contains(condition["event_ref"].as_str().unwrap())
            {
                return Err("event receipt reference is invalid");
            }
            Ok(())
        }
        "conjunction" => {
            for item in condition["items"].as_array().unwrap() {
                validate_ir_condition(
                    item,
                    entity_ids,
                    unit_ids,
                    properties,
                    units,
                    event_ids,
                    state_owners,
                )?;
            }
            Ok(())
        }
        _ => Err("unknown condition kind"),
    }
}

fn validate_condition_order(condition: &Value) -> Result<(), &'static str> {
    if condition["kind"] != "conjunction" {
        return Ok(());
    }
    let items = condition["items"]
        .as_array()
        .ok_or("invalid conjunction items")?;
    if items.len() < 2 {
        return Err("single-item conjunction is forbidden");
    }
    let bytes: Vec<_> = items.iter().map(canonical_bytes).collect();
    if bytes.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("conjunction items are not strictly canonical-sorted");
    }
    Ok(())
}

fn declaration_syntax_node_spans(value: &Declaration, sentence: ByteSpan) -> Vec<ByteSpan> {
    let mut spans = vec![sentence];
    if let Declaration::State { states, .. } = value {
        spans.extend(states.iter().map(|state| state.span));
    }
    spans
}
fn normative_expression_depth(value: &NormativeSentence) -> usize {
    fn condition_depth(condition: &Condition) -> usize {
        let atom_depth = condition
            .atoms
            .iter()
            .map(|atom| match atom {
                AtomicCondition::Comparison { .. } => 2,
                AtomicCondition::State { .. } | AtomicCondition::Event { .. } => 1,
            })
            .max()
            .unwrap_or(0);
        atom_depth + usize::from(condition.atoms.len() > 1)
    }
    match value {
        NormativeSentence::Rule { condition, .. }
        | NormativeSentence::Invariant { condition, .. } => condition_depth(condition),
        NormativeSentence::Transition { trigger, .. } => condition_depth(trigger),
        NormativeSentence::Acceptance {
            trigger, expected, ..
        } => condition_depth(trigger).max(condition_depth(expected)),
        NormativeSentence::Informative { .. } => 0,
    }
}
fn normative_syntax_node_spans(
    value: &NormativeSentence,
    sentence: ByteSpan,
    source: &str,
) -> Vec<ByteSpan> {
    fn push_condition(spans: &mut Vec<ByteSpan>, condition: &Condition) {
        if condition.atoms.len() > 1 {
            spans.push(condition.span);
        }
        for atom in &condition.atoms {
            spans.push(atom.span());
            if let AtomicCondition::Comparison { right, .. } = atom {
                spans.push(right.span);
            }
        }
    }
    fn required_state_span(
        entity: &SymbolRef,
        state: &SymbolRef,
        sentence: ByteSpan,
        source: &str,
    ) -> ByteSpan {
        let mut end = state.span.end_byte;
        while end < sentence.end_byte {
            let ch = source[end..].chars().next().expect("UTF-8 boundary");
            if !matches!(ch, ' ' | '\n' | '\r') {
                break;
            }
            end += ch.len_utf8();
        }
        if source[end..sentence.end_byte].starts_with("状态") {
            end += "状态".len();
        }
        ByteSpan::new(entity.span.start_byte, end)
    }
    let mut spans = vec![sentence];
    match value {
        NormativeSentence::Rule {
            condition,
            behavior,
            ..
        } => {
            push_condition(&mut spans, condition);
            spans.push(behavior.span);
        }
        NormativeSentence::Transition { trigger, .. } => push_condition(&mut spans, trigger),
        NormativeSentence::Invariant {
            condition,
            required_entity,
            required_state,
        } => {
            push_condition(&mut spans, condition);
            spans.push(required_state_span(
                required_entity,
                required_state,
                sentence,
                source,
            ));
        }
        NormativeSentence::Acceptance {
            trigger,
            expected,
            deadline,
        } => {
            push_condition(&mut spans, trigger);
            push_condition(&mut spans, expected);
            spans.push(deadline.span);
        }
        NormativeSentence::Informative { .. } => {}
    }
    spans
}
fn symbol(
    id: &str,
    name: &str,
    kind: SymbolKind,
    owner_ref: Option<String>,
    target_ref: Option<String>,
) -> SymbolRef {
    SymbolRef {
        id: id.into(),
        display_name: name.into(),
        kind,
        owner_ref,
        target_ref,
        span: ByteSpan::new(0, 0),
    }
}
fn declaration_provenance(declaration: &Declaration, sentence: ByteSpan) -> Provenance {
    let mut roles = vec![("sentence", sentence)];
    match declaration {
        Declaration::Entity { name, kind } => {
            roles.extend([("name", name.span), ("entity_kind", kind.span)]);
        }
        Declaration::Unit { name } | Declaration::Event { name } => {
            roles.push(("name", name.span));
        }
        Declaration::Property {
            name,
            value_type,
            access,
            unit,
        } => {
            roles.extend([("name", name.span), ("value_type", value_type.span)]);
            match access.value {
                PropertyAccess::Observable => roles.push(("observable", access.span)),
                PropertyAccess::Writable => roles.push(("writable", access.span)),
                PropertyAccess::Both => {
                    roles.extend([("observable", access.span), ("writable", access.span)])
                }
            }
            if let Some(unit) = unit {
                roles.push(("unit", unit.span));
            }
        }
        Declaration::Action { name, target } => {
            roles.extend([("name", name.span), ("target", target.span)]);
        }
        Declaration::Alias {
            alias,
            kind,
            target,
        } => roles.extend([
            ("alias", alias.span),
            ("target_kind", kind.span),
            ("target", target.span),
        ]),
        Declaration::State {
            owner,
            states,
            initial,
        } => {
            roles.push(("owner", owner.span));
            roles.extend(states.iter().map(|state| ("state", state.span)));
            roles.push(("initial_state", initial.span));
        }
    }
    provenance(sentence, roles)
}

fn normative_provenance(sentence: &NormativeSentence, sentence_span: ByteSpan) -> Provenance {
    let mut roles = vec![("sentence", sentence_span)];
    match sentence {
        NormativeSentence::Rule {
            condition,
            modality,
            behavior,
        } => {
            condition_roles(condition, "condition", &mut roles);
            roles.extend([
                ("modality", modality.span),
                ("behavior", behavior.span),
                ("behavior_actor", behavior.actor.span()),
                ("behavior_action", behavior.action.span),
                ("behavior_target", behavior.target.span),
            ]);
        }
        NormativeSentence::Transition {
            trigger,
            entity,
            source_state,
            target_state,
        } => {
            condition_roles(trigger, "trigger", &mut roles);
            roles.extend([
                ("entity", entity.span),
                ("source_state", source_state.span),
                ("target_state", target_state.span),
            ]);
        }
        NormativeSentence::Invariant {
            condition,
            required_entity,
            required_state,
        } => {
            condition_roles(condition, "condition", &mut roles);
            roles.extend([
                ("scope", sentence_span),
                (
                    "required_state",
                    required_entity.span.cover(required_state.span),
                ),
                ("required_state_entity", required_entity.span),
                ("required_state_state", required_state.span),
            ]);
        }
        NormativeSentence::Acceptance {
            trigger,
            expected,
            deadline,
        } => {
            condition_roles(trigger, "trigger", &mut roles);
            condition_roles(expected, "expected", &mut roles);
            roles.push(("deadline", deadline.span));
        }
        NormativeSentence::Informative { text } => roles.push(("text", text.span)),
    }
    provenance(sentence_span, roles)
}

fn condition_roles(
    condition: &Condition,
    prefix: &'static str,
    roles: &mut Vec<(&'static str, ByteSpan)>,
) {
    roles.push((prefix, condition.span));
    for atom in &condition.atoms {
        match (prefix, atom) {
            (
                "condition",
                AtomicCondition::Comparison {
                    property,
                    operator,
                    right,
                    ..
                },
            ) => roles.extend([
                ("condition_left", property.span),
                ("condition_operator", operator.span),
                ("condition_right", right.span),
            ]),
            ("condition", AtomicCondition::State { entity, state, .. }) => roles.extend([
                ("condition_entity", entity.span),
                ("condition_state", state.span),
            ]),
            (
                "condition",
                AtomicCondition::Event {
                    receiver, event, ..
                },
            ) => roles.extend([
                ("condition_receiver", receiver.span()),
                ("condition_event", event.span),
            ]),
            (
                "trigger",
                AtomicCondition::Comparison {
                    property,
                    operator,
                    right,
                    ..
                },
            ) => roles.extend([
                ("trigger_left", property.span),
                ("trigger_operator", operator.span),
                ("trigger_right", right.span),
            ]),
            ("trigger", AtomicCondition::State { entity, state, .. }) => roles.extend([
                ("trigger_entity", entity.span),
                ("trigger_state", state.span),
            ]),
            (
                "trigger",
                AtomicCondition::Event {
                    receiver, event, ..
                },
            ) => roles.extend([
                ("trigger_receiver", receiver.span()),
                ("trigger_event", event.span),
            ]),
            (
                "expected",
                AtomicCondition::Comparison {
                    property,
                    operator,
                    right,
                    ..
                },
            ) => roles.extend([
                ("expected_left", property.span),
                ("expected_operator", operator.span),
                ("expected_right", right.span),
            ]),
            ("expected", AtomicCondition::State { entity, state, .. }) => roles.extend([
                ("expected_entity", entity.span),
                ("expected_state", state.span),
            ]),
            (
                "expected",
                AtomicCondition::Event {
                    receiver, event, ..
                },
            ) => roles.extend([
                ("expected_receiver", receiver.span()),
                ("expected_event", event.span),
            ]),
            _ => unreachable!("closed condition role prefix"),
        }
    }
}
fn provenance(sentence: ByteSpan, roles: Vec<(&str, ByteSpan)>) -> Provenance {
    let mut roles: Vec<_> = roles.into_iter().map(|(r, s)| (r.into(), s)).collect();
    roles.sort();
    roles.dedup();
    Provenance { sentence, roles }
}
fn add_source(graph: &mut Graph, id: &str, provenance: Provenance) {
    graph
        .declaration_sources
        .entry(id.into())
        .or_default()
        .push(provenance)
}
fn nominal_unit(name: &str) -> Value {
    json!({"id":format!("unit:{name}"),"display_name":name,"unit_kind":"nominal","dimension_id":format!("dimension:nominal:{name}"),"canonical_unit_ref":format!("unit:{name}"),"scale":"1","offset":"0"})
}
fn builtin_unit(name: &str, dimension: &str, canonical: &str) -> Value {
    json!({"id":format!("unit:{name}"),"display_name":name,"unit_kind":"builtin","dimension_id":dimension,"canonical_unit_ref":canonical,"scale":"1","offset":"0"})
}
fn values(map: &BTreeMap<String, Value>) -> Vec<Value> {
    map.values().cloned().collect()
}
fn frame_values(
    groups: &BTreeMap<&'static str, BTreeMap<String, (Value, BTreeSet<Provenance>)>>,
    kind: &str,
) -> Vec<Value> {
    groups
        .get(kind)
        .map_or_else(Vec::new, |m| m.values().map(|(v, _)| v.clone()).collect())
}
fn canonical_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("serializable canonical value")
}
fn anonymous_id(kind: &str, payload: &Value) -> String {
    format!(
        "{kind}:sha256:{}",
        domain_hash_with_kind("APLS-CNL-NODE-ID-0.1", kind, &canonical_bytes(payload))
    )
}
fn domain_hash(domain: &str, bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(domain.as_bytes());
    hash.update([0]);
    hash.update(bytes);
    hex_bytes(&hash.finalize())
}
fn domain_hash_with_kind(domain: &str, kind: &str, bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(domain.as_bytes());
    hash.update([0]);
    hash.update(kind.as_bytes());
    hash.update([0]);
    hash.update(bytes);
    hex_bytes(&hash.finalize())
}
fn hex_digest(bytes: &[u8]) -> String {
    hex_bytes(&Sha256::digest(bytes))
}
fn hex_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 15) as usize] as char);
    }
    output
}
fn one(diagnostic: Diagnostic) -> OrderedDiagnostics {
    vec![diagnostic]
}
fn suppress_derived_diagnostics(diagnostics: &mut OrderedDiagnostics) {
    let specific_spans: BTreeSet<_> = diagnostics
        .iter()
        .filter(|diagnostic| !matches!(diagnostic.code.as_str(), "APLS-E1101" | "APLS-E1308"))
        .filter_map(|diagnostic| diagnostic.primary_source_span.clone())
        .collect();
    diagnostics.retain(|diagnostic| {
        !matches!(diagnostic.code.as_str(), "APLS-E1101" | "APLS-E1308")
            || diagnostic
                .primary_source_span
                .as_ref()
                .is_none_or(|span| !specific_spans.contains(span))
    });
}
fn diagnose_unparsed(
    sentence: &Sentence<'_>,
    graph: &Graph,
    path: &str,
    source: &str,
) -> OrderedDiagnostics {
    fn word_span(sentence: &Sentence<'_>, word: &str) -> Option<ByteSpan> {
        sentence.text.find(word).map(|offset| {
            ByteSpan::new(
                sentence.span.start_byte + offset,
                sentence.span.start_byte + offset + word.len(),
            )
        })
    }

    let mut diagnostics = Vec::new();

    let mut quoted_at = 0;
    while let Some(open_relative) = sentence.text[quoted_at..].find('“') {
        let open = quoted_at + open_relative;
        let content_start = open + '“'.len_utf8();
        let Some(close_relative) = sentence.text[content_start..].find('”') else {
            break;
        };
        let close = content_start + close_relative;
        let name = &sentence.text[content_start..close];
        if !graph
            .symbols
            .iter()
            .any(|symbol| symbol.display_name == name)
        {
            let mut diagnostic = Diagnostic::source(
                "APLS-E1201",
                "quoted term has no declaration candidate",
                path,
                source,
                ByteSpan::new(
                    sentence.span.start_byte + open,
                    sentence.span.start_byte + close + '”'.len_utf8(),
                ),
            );
            diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
            diagnostic
                .missing_or_ambiguous_roles
                .push("declared_term".into());
            diagnostics.push(diagnostic);
        }
        quoted_at = close + '”'.len_utf8();
    }

    for operator in ["不等于", "不高于", "不低于", "等于", "低于", "高于"] {
        let Some(operator_at) = sentence.text.find(operator) else {
            continue;
        };
        let before = &sentence.text[..operator_at];
        let start = before.rfind("并且").map_or(0, |index| index + "并且".len());
        let mut term = before[start..].trim_matches([' ', '\n', '\r']);
        for prefix in ["安全要求：当", "安全要求：如果", "安全要求：", "当", "如果"]
        {
            if let Some(rest) = term.strip_prefix(prefix) {
                term = rest.trim_matches([' ', '\n', '\r']);
                break;
            }
        }
        let (name, local_start, local_end) = if term.starts_with('“') && term.ends_with('”') {
            (
                &term['“'.len_utf8()..term.len() - '”'.len_utf8()],
                before.find(term).unwrap_or(start),
                before.find(term).unwrap_or(start) + term.len(),
            )
        } else {
            let local_start = before.rfind(term).unwrap_or(start);
            (term, local_start, local_start + term.len())
        };
        if !name.is_empty() {
            let candidates: Vec<_> = graph
                .symbols
                .iter()
                .filter(|symbol| symbol.display_name == name)
                .collect();
            let span = ByteSpan::new(
                sentence.span.start_byte + local_start,
                sentence.span.start_byte + local_end,
            );
            if candidates.is_empty() {
                let mut diagnostic = Diagnostic::source(
                    "APLS-E1201",
                    "condition term has no declaration candidate",
                    path,
                    source,
                    span,
                );
                diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
                diagnostic
                    .missing_or_ambiguous_roles
                    .push("condition_property".into());
                diagnostics.push(diagnostic);
            } else if !candidates
                .iter()
                .any(|symbol| symbol.kind == SymbolKind::Property)
            {
                let mut diagnostic = Diagnostic::source(
                    "APLS-E1204",
                    "condition comparison requires a declared property",
                    path,
                    source,
                    span,
                );
                diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
                diagnostic
                    .missing_or_ambiguous_roles
                    .push("condition_property".into());
                diagnostics.push(diagnostic);
            }
        }
        break;
    }
    for word in ["适当", "一点", "大概", "差不多", "高", "低"] {
        if let Some(span) = word_span(sentence, word) {
            let mut diagnostic = Diagnostic::source(
                "APLS-E1301",
                "vague threshold, degree, or change amount has no computable meaning",
                path,
                source,
                span,
            );
            diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
            diagnostic
                .missing_or_ambiguous_roles
                .push("precise_value".into());
            diagnostics.push(diagnostic);
        }
    }
    for word in ["它", "该设备", "前者", "后者"] {
        if let Some(span) = word_span(sentence, word) {
            let mut diagnostic = Diagnostic::source(
                "APLS-E1302",
                "reference is not frozen to one declared term",
                path,
                source,
                span,
            );
            diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
            diagnostic
                .missing_or_ambiguous_roles
                .push("referent".into());
            diagnostics.push(diagnostic);
        }
    }
    for word in ["应该", "可以", "尽量"] {
        if let Some(span) = word_span(sentence, word) {
            let mut diagnostic = Diagnostic::source(
                "APLS-E1304",
                "modality must be one of “必须”, “不得”, or “禁止”",
                path,
                source,
                span,
            );
            diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
            diagnostic
                .missing_or_ambiguous_roles
                .push("modality".into());
            diagnostics.push(diagnostic);
        }
    }
    for word in ["或者", "不是", "没有"] {
        if let Some(span) = word_span(sentence, word) {
            let mut diagnostic = Diagnostic::source(
                "APLS-E1306",
                "unsupported disjunction, nested condition, or general-language negation",
                path,
                source,
                span,
            );
            diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
            diagnostic
                .missing_or_ambiguous_roles
                .push("condition_scope".into());
            diagnostics.push(diagnostic);
        }
    }
    for word in ["尽快", "及时", "稍后", "随后"] {
        if let Some(span) = word_span(sentence, word) {
            let mut diagnostic = Diagnostic::source(
                "APLS-E1307",
                "time reference or deadline is not exact",
                path,
                source,
                span,
            );
            diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
            diagnostic
                .missing_or_ambiguous_roles
                .push("exact_time".into());
            diagnostics.push(diagnostic);
        }
    }

    let missing_owner = ["温度", "速度"]
        .into_iter()
        .filter_map(|word| word_span(sentence, word))
        .collect::<Vec<_>>();
    if let Some(span) = missing_owner.first().copied() {
        let mut diagnostic = Diagnostic::source(
            "APLS-E1303",
            "property owner or action target is missing",
            path,
            source,
            span,
        );
        diagnostic.normative_rule_reference = "DES-APLS-CNL-DIAG-001".into();
        diagnostic
            .missing_or_ambiguous_roles
            .extend(["action_target".into(), "property_owner".into()]);
        diagnostics.push(diagnostic);
    }

    if diagnostics.is_empty() {
        diagnostics.push(Diagnostic::source(
            "APLS-E1101",
            "sentence does not match the APLS 0.1 controlled-natural-language grammar",
            path,
            source,
            sentence.span,
        ));
    }
    diagnostics
}
fn ambiguity(
    path: &str,
    source: &str,
    span: ByteSpan,
    kind: &str,
    witnesses: Vec<String>,
) -> Diagnostic {
    let mut d = Diagnostic::source(
        "APLS-E1310",
        "multiple inequivalent canonical meanings remain",
        path,
        source,
        span,
    );
    d.normative_rule_reference = "DEC-017 / DES-APLS-CNL-DIAG-001".into();
    d.missing_or_ambiguous_roles.push("meaning".into());
    let ws:Vec<_>=witnesses.into_iter().take(2).map(|w|{let fp=format!("sha256:{}",domain_hash("APLS-CNL-FRAME-WITNESS-0.1",w.as_bytes()));json!({"frame_kind":if kind=="declaration"{"entity_declaration"}else{"rule"},"semantic_fingerprint":fp,"role_fingerprints":[{"role":"meaning","value_fingerprint":format!("sha256:{}",domain_hash_with_kind("APLS-CNL-ROLE-WITNESS-0.1","meaning",w.as_bytes()))}]})}).collect();
    d.payload = Some(
        json!({"kind":"ambiguity","outcome":"AMBIGUOUS","differing_roles":["meaning"],"witnesses":ws}),
    );
    d
}

fn frame_ambiguity(
    path: &str,
    source: &str,
    span: ByteSpan,
    left: &(Frame, BTreeSet<Provenance>),
    right: &(Frame, BTreeSet<Provenance>),
) -> Diagnostic {
    let left_roles = frame_roles(&left.0);
    let right_roles = frame_roles(&right.0);
    let all_roles: BTreeSet<_> = left_roles
        .keys()
        .chain(right_roles.keys())
        .copied()
        .collect();
    let differing: Vec<_> = all_roles
        .into_iter()
        .filter(|role| left_roles.get(role) != right_roles.get(role))
        .collect();
    let differing_set: BTreeSet<_> = differing.iter().copied().collect();
    let mut differing_spans = [&left.1, &right.1]
        .into_iter()
        .flat_map(|provenances| provenances.iter())
        .flat_map(|provenance| provenance.roles.iter())
        .filter(|(role, _)| differing_set.contains(role.as_str()))
        .map(|(_, span)| *span);
    let primary_span = differing_spans
        .next()
        .map(|first| differing_spans.fold(first, ByteSpan::cover))
        .unwrap_or(span);
    let mut diagnostic = Diagnostic::source(
        "APLS-E1310",
        "multiple inequivalent canonical meanings remain",
        path,
        source,
        primary_span,
    );
    diagnostic.normative_rule_reference = "DEC-017 / DES-APLS-CNL-DIAG-001".into();
    diagnostic.missing_or_ambiguous_roles = differing.iter().map(|role| (*role).into()).collect();
    let witnesses: Vec<_> = [&left.0, &right.0]
        .into_iter()
        .map(|frame| {
            let roles = frame_roles(frame);
            let role_fingerprints: Vec<_> = differing.iter().map(|role| {
                let bytes = roles.get(role).map_or_else(|| b"null".to_vec(), canonical_bytes);
                json!({"role":role,"value_fingerprint":format!("sha256:{}",domain_hash_with_kind("APLS-CNL-ROLE-WITNESS-0.1",role,&bytes))})
            }).collect();
            json!({"frame_kind":schema_frame_kind(frame.kind),"semantic_fingerprint":format!("sha256:{}",domain_hash("APLS-CNL-FRAME-WITNESS-0.1",&canonical_bytes(&frame.value))),"role_fingerprints":role_fingerprints})
        })
        .collect();
    diagnostic.payload = Some(
        json!({"kind":"ambiguity","outcome":"AMBIGUOUS","differing_roles":differing,"witnesses":witnesses}),
    );
    diagnostic
}

fn schema_frame_kind(kind: &str) -> &str {
    match kind {
        "info" => "informative_item",
        value => value,
    }
}

fn frame_roles(frame: &Frame) -> BTreeMap<&'static str, Value> {
    let mut roles = BTreeMap::new();
    match frame.kind {
        "rule" => {
            roles.insert("condition", frame.value["condition"].clone());
            roles.insert("modality", frame.value["modality"].clone());
            roles.insert(
                "behavior_actor",
                frame.value["behavior"]["actor_ref"].clone(),
            );
            roles.insert(
                "behavior_action",
                frame.value["behavior"]["action_ref"].clone(),
            );
            roles.insert(
                "behavior_target",
                frame.value["behavior"]["target_ref"].clone(),
            );
        }
        "transition" => {
            roles.insert("trigger", frame.value["trigger"].clone());
            roles.insert("entity", frame.value["entity_ref"].clone());
            roles.insert("source_state", frame.value["source_state_ref"].clone());
            roles.insert("target_state", frame.value["target_state_ref"].clone());
        }
        "invariant" => {
            roles.insert("condition", frame.value["condition"].clone());
            roles.insert("scope", frame.value["scope"].clone());
            roles.insert("required_state", frame.value["required_state"].clone());
        }
        "acceptance" => {
            roles.insert("trigger", frame.value["trigger"].clone());
            roles.insert("expected", frame.value["expected"].clone());
            roles.insert("deadline", frame.value["deadline"].clone());
        }
        "info" => {
            roles.insert("text", frame.value["text"].clone());
        }
        _ => unreachable!("closed frame kind"),
    }
    roles
}

#[cfg(test)]
mod tests {
    use super::*;
    const PREFIX: &str = "本规范采用 APLS 简体中文语言版本 0.1。\n“水箱”是设备。\n“水泵”是执行器。\n“液位”是百分比类型的可观测属性。\n“启动”是“水泵”支持的动作。\n";

    fn ir(source: &str) -> Value {
        match compile(source.as_bytes(), "main.apls", CompileThrough::Emit) {
            CompileOutcome::Accepted(CompileSuccess::Verified(artifact)) => {
                serde_json::from_slice(artifact.as_bytes()).unwrap()
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn emits_schema_valid_ir() {
        let value = ir(&format!("{PREFIX}当液位低于20%时，系统必须启动水泵。"));
        assert_eq!(value["rules"].as_array().unwrap().len(), 1);
        assert_eq!(value["header"]["unicode_data_version"], "17.0.0");
    }

    #[test]
    fn schema_and_cross_node_revalidation_reject_tampering() {
        let source = format!("{PREFIX}当液位低于20%时，系统必须启动水泵。");
        let value = ir(&source);
        cross_validate(&value, &source).unwrap();

        let schema: Value = serde_json::from_str(IR_SCHEMA).unwrap();
        let validator = jsonschema::validator_for(&schema).unwrap();
        let mut unknown_field = value.clone();
        unknown_field["unexpected"] = json!(true);
        assert!(!validator.is_valid(&unknown_field));

        let mut wrong_header = value.clone();
        wrong_header["header"]["unicode_data_version"] = json!("16.0.0");
        assert!(cross_validate(&wrong_header, &source).is_err());

        let mut missing_unit = value.clone();
        missing_unit["rules"][0]["condition"]["right"]["canonical_unit_ref"] =
            json!("unit:missing");
        assert!(cross_validate(&missing_unit, &source).is_err());

        let mut wrong_pointer = value;
        wrong_pointer["source_map"][0]["ir_pointer"] = json!("/rules/0");
        assert!(cross_validate(&wrong_pointer, &source).is_err());

        let mut reordered_roles = ir(&source);
        let provenance = reordered_roles["source_map"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find_map(|entry| {
                let roles = entry["provenance"][0]["role_spans"].as_array_mut()?;
                (roles.len() > 1).then_some(roles)
            })
            .unwrap();
        provenance.reverse();
        assert_eq!(
            cross_validate(&reordered_roles, &source)
                .unwrap_err()
                .message,
            "source-map role spans are not strictly sorted"
        );
    }

    #[test]
    fn cross_validator_reexecutes_the_semantic_closure_on_output_bytes() {
        fn refresh_anonymous_id(ir: &mut Value, field: &str, index: usize, kind: &str) {
            let mut payload = ir[field][index].as_object().unwrap().clone();
            payload.remove("id");
            ir[field][index]["id"] = json!(anonymous_id(kind, &Value::Object(payload)));
        }
        fn sort_nodes(ir: &mut Value, field: &str) {
            ir[field]
                .as_array_mut()
                .unwrap()
                .sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
        }
        fn failure_message(ir: &Value, source: &str) -> String {
            let error = cross_validate(ir, source).unwrap_err();
            assert_eq!(error.code, "APLS-T0006");
            error.message
        }

        let source = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“升每分钟”是单位。
“液位”是百分比类型的可观测属性。
“启动”是“水泵”支持的动作。
“启动命令”是事件。
“水泵”的状态包括“待机”、“运行”和“故障”，初始状态是“待机”。
当液位低于20%时，系统必须启动水泵。
当系统收到启动命令时，水泵从待机状态进入运行状态。
当系统收到启动命令时，水泵从运行状态进入故障状态。
验收要求：当系统收到启动命令时，水泵处于故障状态必须在2秒内成立。";
        let value = ir(source);
        cross_validate(&value, source).unwrap();

        let mut nominal_unit = value.clone();
        let nominal_index = nominal_unit["units"]
            .as_array()
            .unwrap()
            .iter()
            .position(|unit| unit["id"] == "unit:升每分钟")
            .unwrap();
        nominal_unit["units"][nominal_index]["scale"] = json!("2");
        assert_eq!(
            failure_message(&nominal_unit, source),
            "nominal unit identity is invalid"
        );

        let mut property_matrix = value.clone();
        property_matrix["properties"][0]["value_type"] = json!("boolean");
        assert_eq!(
            failure_message(&property_matrix, source),
            "property type and unit are incompatible"
        );

        let mut comparison_matrix = value.clone();
        comparison_matrix["rules"][0]["condition"]["right"] =
            json!({"kind":"boolean","value":true});
        refresh_anonymous_id(&mut comparison_matrix, "rules", 0, "rule");
        assert_eq!(
            failure_message(&comparison_matrix, source),
            "comparison literal is incompatible with its property"
        );

        let mut noncanonical_literal = value.clone();
        noncanonical_literal["rules"][0]["condition"]["right"]["value"] = json!("20.0");
        refresh_anonymous_id(&mut noncanonical_literal, "rules", 0, "rule");
        assert_eq!(
            failure_message(&noncanonical_literal, source),
            "typed literal value is not canonical"
        );

        let mut same_state = value.clone();
        same_state["transitions"][0]["target_state_ref"] =
            same_state["transitions"][0]["source_state_ref"].clone();
        refresh_anonymous_id(&mut same_state, "transitions", 0, "transition");
        sort_nodes(&mut same_state, "transitions");
        assert_eq!(
            failure_message(&same_state, source),
            "transition source and target states must differ"
        );

        let mut unreachable = value.clone();
        unreachable["transitions"]
            .as_array_mut()
            .unwrap()
            .retain(|transition| transition["target_state_ref"] != "state:水泵:故障");
        assert_eq!(
            failure_message(&unreachable, source),
            "state is unreachable from the initial state"
        );

        let mut conflict = value.clone();
        let first_trigger = conflict["transitions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|transition| transition["source_state_ref"] == "state:水泵:待机")
            .unwrap()["trigger"]
            .clone();
        let conflict_index = conflict["transitions"]
            .as_array()
            .unwrap()
            .iter()
            .position(|transition| transition["source_state_ref"] == "state:水泵:运行")
            .unwrap();
        conflict["transitions"][conflict_index]["source_state_ref"] = json!("state:水泵:待机");
        conflict["transitions"][conflict_index]["trigger"] = first_trigger;
        refresh_anonymous_id(&mut conflict, "transitions", conflict_index, "transition");
        sort_nodes(&mut conflict, "transitions");
        assert_eq!(
            failure_message(&conflict, source),
            "same transition trigger selects different target states"
        );

        let mut deadline = value.clone();
        deadline["acceptance"][0]["deadline"]["value"] = json!("0");
        refresh_anonymous_id(&mut deadline, "acceptance", 0, "acceptance");
        assert_eq!(
            failure_message(&deadline, source),
            "acceptance deadline is not a positive canonical time quantity"
        );

        let mut extra_unit = value;
        extra_unit["units"]
            .as_array_mut()
            .unwrap()
            .push(builtin_unit(
                "摄氏度",
                "dimension:temperature",
                "unit:摄氏度",
            ));
        sort_nodes(&mut extra_unit, "units");
        assert_eq!(
            failure_message(&extra_unit, source),
            "unit materialization closure is not exact"
        );
    }

    #[test]
    fn all_surface_sentence_and_frame_kinds_compile() {
        let source = "本规范采用 APLS 简体中文语言版本 0.1。
“水箱”是设备。
“水泵”是执行器。
“升每分钟”是单位。
“液位”是百分比类型的可观测属性。
“水温”是小数类型的可观测属性，单位为“摄氏度”。
“流量”是小数类型的可观测且可写属性，单位为“升每分钟”。
“急停有效”是布尔类型的可观测属性。
“启动”是“水泵”支持的动作。
“启动命令”是事件。
“泵”是实体“水泵”的别名。
“水泵”的状态包括“待机”和“运行”，初始状态是“待机”。
当液位低于20%并且水温低于35摄氏度时，系统必须启动泵。
当系统收到启动命令时，水泵从待机状态进入运行状态。
安全要求：当急停有效等于真时，水泵必须处于待机状态。
验收要求：当系统收到启动命令时，水泵处于运行状态必须在2秒内成立。
说明：『这是一条非规范说明』。";
        let value = ir(source);
        for field in [
            "rules",
            "transitions",
            "invariants",
            "acceptance",
            "informative_items",
        ] {
            assert_eq!(value[field].as_array().unwrap().len(), 1, "{field}");
        }
        assert!(
            value["units"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["id"] == "unit:升每分钟")
        );
        let unit_ids: BTreeSet<_> = value["units"]
            .as_array()
            .unwrap()
            .iter()
            .map(|unit| unit["id"].as_str().unwrap())
            .collect();
        assert_eq!(
            unit_ids,
            BTreeSet::from(["unit:%", "unit:升每分钟", "unit:摄氏度", "unit:毫秒"])
        );
        let mapped_ids: BTreeSet<_> = value["source_map"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["semantic_id"].as_str().unwrap())
            .collect();
        assert!(mapped_ids.contains("unit:升每分钟"));
        assert!(!mapped_ids.contains("unit:%"));
        assert!(!mapped_ids.contains("unit:摄氏度"));
        assert!(!mapped_ids.contains("unit:毫秒"));
    }

    #[test]
    fn property_and_unit_matrix_accepts_and_rejects_without_guessing() {
        let declarations = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“启动”是“水泵”支持的动作。
“次”是单位。
“升每分钟”是单位。
“有效”是布尔类型的可观测属性。
“次数”是整数类型的可观测属性。
“比率”是小数类型的可观测属性。
“液位”是百分比类型的可观测属性。
“模式”是文本类型的可观测属性。
“等待时长”是时长类型的可观测属性。
“水温”是小数类型的可观测属性，单位为“摄氏度”。
“配额”是整数类型的可观测属性，单位为“次”。
“目标流量”是小数类型的可观测属性，单位为“升每分钟”。
";
        let valid = format!(
            "{declarations}当有效等于真时，系统必须启动水泵。
当次数高于3时，系统必须启动水泵。
当比率等于3时，系统必须启动水泵。
当液位低于20%时，系统必须启动水泵。
当模式等于『自动』时，系统必须启动水泵。
当等待时长不高于2秒时，系统必须启动水泵。
当水温低于35摄氏度时，系统必须启动水泵。
当配额高于3 次时，系统必须启动水泵。
当目标流量高于2.5 升每分钟时，系统必须启动水泵。"
        );
        assert_eq!(ir(&valid)["rules"].as_array().unwrap().len(), 9);

        for invalid in [
            "当有效高于真时，系统必须启动水泵。",
            "当次数等于3.0时，系统必须启动水泵。",
            "当模式高于『自动』时，系统必须启动水泵。",
            "当液位低于20时，系统必须启动水泵。",
            "当等待时长不高于2时，系统必须启动水泵。",
            "当水温低于35时，系统必须启动水泵。",
            "当目标流量高于2.5摄氏度时，系统必须启动水泵。",
        ] {
            match compile(
                format!("{declarations}{invalid}").as_bytes(),
                "main.apls",
                CompileThrough::Check,
            ) {
                CompileOutcome::Rejected(diagnostics) => {
                    assert_eq!(diagnostics[0].code, "APLS-E1401", "{invalid}")
                }
                other => panic!("{invalid}: {other:?}"),
            }
        }
    }

    #[test]
    fn three_natural_condition_leads_have_one_semantic_hash() {
        let endings = [
            "当液位低于20%时，系统必须启动水泵。",
            "如果液位低于20%，系统必须启动水泵。",
            "液位低于20%时，系统必须启动水泵。",
        ];
        let hashes: BTreeSet<_> = endings
            .iter()
            .map(|ending| {
                ir(&format!("{PREFIX}{ending}"))["header"]["semantic_hash"]
                    .as_str()
                    .unwrap()
                    .to_owned()
            })
            .collect();
        assert_eq!(hashes.len(), 1);
    }

    #[test]
    fn multiple_tokenizations_may_converge() {
        let source = format!(
            "{PREFIX}“泵”是实体“水泵”的别名。\n“启动水”是动作“启动”的别名。\n当液位低于20%时，系统必须启动水泵。"
        );
        let value = ir(&source);
        assert_eq!(value["rules"].as_array().unwrap().len(), 1);
        let rule_id = value["rules"][0]["id"].as_str().unwrap();
        let entry = value["source_map"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["semantic_id"] == rule_id)
            .unwrap();
        let provenance = entry["provenance"].as_array().unwrap();
        assert_eq!(provenance.len(), 1);
        let role_text = |role: &str| {
            provenance[0]["role_spans"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|item| item["role"] == role)
                .map(|item| {
                    let start = item["span"]["start_byte"].as_u64().unwrap() as usize;
                    let end = item["span"]["end_byte"].as_u64().unwrap() as usize;
                    &source[start..end]
                })
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(
            role_text("behavior_action"),
            BTreeSet::from(["启动", "启动水"])
        );
        assert_eq!(role_text("behavior_target"), BTreeSet::from(["泵", "水泵"]));
    }

    #[test]
    fn duplicate_conditions_and_rules_merge_semantics_and_provenance() {
        let single = ir(&format!("{PREFIX}当液位低于20%时，系统必须启动水泵。"));
        let merged = ir(&format!(
            "{PREFIX}当液位低于20%时，系统必须启动水泵。
当液位低于20%并且液位低于20%时，系统必须启动水泵。"
        ));
        assert_eq!(
            single["header"]["semantic_hash"],
            merged["header"]["semantic_hash"]
        );
        assert_eq!(merged["rules"].as_array().unwrap().len(), 1);
        assert_eq!(merged["rules"][0]["condition"]["kind"], "comparison");
        let rule_id = merged["rules"][0]["id"].as_str().unwrap();
        let source_entry = merged["source_map"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["semantic_id"] == rule_id)
            .unwrap();
        assert_eq!(source_entry["provenance"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn declaration_forward_references_are_order_independent() {
        let declarations = [
            "“流量”是小数类型的可观测属性，单位为“升每分钟”。",
            "“启动”是“水泵”支持的动作。",
            "“泵”是实体“水泵”的别名。",
            "“水泵”的状态包括“待机”和“运行”，初始状态是“待机”。",
            "“启动命令”是事件。",
            "“水泵”是执行器。",
            "“升每分钟”是单位。",
        ];
        let profile = "本规范采用 APLS 简体中文语言版本 0.1。\n";
        let behavior = "\n当系统收到启动命令时，泵从待机状态进入运行状态。";
        let forward = format!("{profile}{}{behavior}", declarations.join("\n"));
        let reverse = format!(
            "{profile}{}{behavior}",
            declarations
                .iter()
                .rev()
                .copied()
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert_eq!(
            ir(&forward)["header"]["semantic_hash"],
            ir(&reverse)["header"]["semantic_hash"]
        );
    }

    #[test]
    fn transition_reachability_and_conflicts_fail_closed() {
        let prefix = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“启动命令”是事件。
“水泵”的状态包括“待机”、“运行”和“急停”，初始状态是“待机”。
";
        let unreachable = format!("{prefix}当系统收到启动命令时，水泵从待机状态进入运行状态。");
        match compile(unreachable.as_bytes(), "main.apls", CompileThrough::Check) {
            CompileOutcome::Rejected(diagnostics) => {
                let diagnostic = diagnostics.iter().find(|d| d.code == "APLS-E1402").unwrap();
                let primary = diagnostic.primary_source_span.as_ref().unwrap();
                assert_eq!(&unreachable[primary.start_byte..primary.end_byte], "“急停”");
                assert_eq!(diagnostic.related_source_spans.len(), 1);
                let related = &diagnostic.related_source_spans[0];
                assert_eq!(
                    &unreachable[related.start_byte..related.end_byte],
                    "当系统收到启动命令时，水泵从待机状态进入运行状态。"
                );
            }
            other => panic!("{other:?}"),
        }

        let conflict = format!(
            "{prefix}当系统收到启动命令时，水泵从待机状态进入运行状态。
当系统收到启动命令时，水泵从待机状态进入急停状态。"
        );
        match compile(conflict.as_bytes(), "main.apls", CompileThrough::Check) {
            CompileOutcome::Rejected(diagnostics) => {
                let diagnostic = diagnostics.iter().find(|d| d.code == "APLS-E1403").unwrap();
                let primary = diagnostic.primary_source_span.as_ref().unwrap();
                assert_eq!(
                    &conflict[primary.start_byte..primary.end_byte],
                    "当系统收到启动命令时，水泵从待机状态进入运行状态。"
                );
                assert_eq!(diagnostic.related_source_spans.len(), 1);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn stable_diagnostics_use_deadline_and_transition_contract_spans() {
        let source = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“启动命令”是事件。
“水泵”的状态包括“待机”和“运行”，初始状态是“待机”。
验收要求：当系统收到启动命令时，水泵处于运行状态必须在0秒内成立。";
        let diagnostics = match compile(source.as_bytes(), "main.apls", CompileThrough::Check) {
            CompileOutcome::Rejected(diagnostics) => diagnostics,
            other => panic!("{other:?}"),
        };
        let diagnostic = diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "APLS-E1307")
            .unwrap();
        let primary = diagnostic.primary_source_span.as_ref().unwrap();
        assert_eq!(&source[primary.start_byte..primary.end_byte], "0秒");

        let first = crate::diagnostic::envelope_bytes("rejected", &diagnostics).unwrap();
        let second_diagnostics =
            match compile(source.as_bytes(), "main.apls", CompileThrough::Check) {
                CompileOutcome::Rejected(diagnostics) => diagnostics,
                other => panic!("{other:?}"),
            };
        let second = crate::diagnostic::envelope_bytes("rejected", &second_diagnostics).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn inequivalent_final_frames_are_ambiguous() {
        let source = "本规范采用 APLS 简体中文语言版本 0.1。
“泵”是执行器。
“水泵”是执行器。
“有效”是布尔类型的可观测属性。
“启动水”是“泵”支持的动作。
“启动”是“水泵”支持的动作。
当有效等于真时，系统必须启动水泵。";
        match compile(source.as_bytes(), "main.apls", CompileThrough::Check) {
            CompileOutcome::Rejected(diagnostics) => {
                assert_eq!(diagnostics[0].code, "APLS-E1310");
                let primary = diagnostics[0].primary_source_span.as_ref().unwrap();
                assert_eq!(&source[primary.start_byte..primary.end_byte], "启动水泵");
                assert_eq!(
                    diagnostics[0].payload.as_ref().unwrap()["witnesses"]
                        .as_array()
                        .unwrap()
                        .len(),
                    2
                );
                let envelope = crate::diagnostic::envelope_bytes("rejected", &diagnostics)
                    .expect("ambiguity envelope must match the public schema");
                let text = String::from_utf8(envelope).unwrap();
                assert!(!text.contains("token_stream"));
                assert!(!text.contains("parse_tree"));
                assert!(!text.contains("canonical_frame"));
                let repeated = match compile(source.as_bytes(), "main.apls", CompileThrough::Check)
                {
                    CompileOutcome::Rejected(diagnostics) => {
                        crate::diagnostic::envelope_bytes("rejected", &diagnostics).unwrap()
                    }
                    other => panic!("{other:?}"),
                };
                assert_eq!(text.into_bytes(), repeated);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn type_mismatch_is_not_reported_as_ambiguity() {
        let source = format!("{PREFIX}当液位等于真时，系统必须启动水泵。");
        match compile(source.as_bytes(), "main.apls", CompileThrough::Check) {
            CompileOutcome::Rejected(diagnostics) => assert_eq!(diagnostics[0].code, "APLS-E1401"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn literal_spacing_uses_the_actual_grammar_production() {
        let source = "20  %";
        let specialized = RawLiteral {
            kind: RawLiteralKind::Percentage("20".into()),
            span: ByteSpan::new(0, source.len()),
        };
        let mut unit = symbol("unit:%", "%", SymbolKind::Unit, None, None);
        unit.span = ByteSpan::new(4, 5);
        let declared_unit = RawLiteral {
            kind: RawLiteralKind::Quantity {
                value: "20".into(),
                unit,
            },
            span: ByteSpan::new(0, source.len()),
        };

        assert!(!literal_spacing_is_valid(&specialized, source));
        assert!(literal_spacing_is_valid(&declared_unit, source));
        assert!(grammar_ws1("\r\n"));
        assert!(!grammar_ws1("\r"));
    }

    #[test]
    fn production_pipeline_counts_only_fully_bound_candidates() {
        fn stage_counts(document: &str) -> ([usize; 3], String) {
            let (source, sentences) = validate_and_split(document.as_bytes(), "main.apls").unwrap();
            let mut ledger = Ledger::default();
            validate_profile(source, &sentences[0], &mut ledger, "main.apls").unwrap();
            let mut records = Vec::new();
            let mut declarations = BTreeSet::new();
            for sentence in sentences.iter().skip(1) {
                let mut candidates = BTreeSet::new();
                for stream in
                    lex_and_enumerate(sentence, 1, &[], &mut ledger, "main.apls", source).unwrap()
                {
                    if let Ok(parsed) = crate::apls_grammar::DeclarationEntryParser::new()
                        .parse(stream.into_iter().map(Ok))
                    {
                        candidates.insert(parsed.value);
                    }
                }
                if let Some(declaration) = candidates.into_iter().next() {
                    declarations.insert(sentence.index);
                    records.push(DeclRecord {
                        declaration,
                        span: sentence.span,
                    });
                }
            }
            let graph = build_graph(&records, "main.apls", source).unwrap();
            let sentence = sentences
                .iter()
                .skip(1)
                .find(|sentence| !declarations.contains(&sentence.index))
                .unwrap();
            let candidate = lex_and_enumerate(
                sentence,
                2,
                &graph.symbols,
                &mut ledger,
                "main.apls",
                source,
            )
            .unwrap()
            .into_iter()
            .find_map(|stream| {
                crate::apls_grammar::NormativeEntryParser::new()
                    .parse(stream.into_iter().map(Ok))
                    .ok()
            })
            .unwrap()
            .value;
            let result = process_candidate(
                candidate,
                sentence.index,
                sentence.span,
                &graph,
                "main.apls",
                source,
                &mut ledger,
            );
            let counts = [
                ledger.value_for_test(Resource::BoundFrames),
                ledger.value_for_test(Resource::TypedFrames),
                ledger.value_for_test(Resource::CanonicalFrames),
            ];
            let error = match result {
                Ok(_) => panic!("candidate unexpectedly completed every stage"),
                Err(error) => error,
            };
            (counts, error.code)
        }

        let binding_failure = "本规范采用 APLS 简体中文语言版本 0.1。
“设备甲”是设备。
“设备乙”是设备。
“启动”是“设备甲”支持的动作。
“设备乙”的状态包括“待机”和“故障”，初始状态是“待机”。
当设备甲处于故障状态时，系统必须启动设备甲。";
        assert_eq!(
            stage_counts(binding_failure),
            ([0, 0, 0], "APLS-E1207".into())
        );

        let type_failure = format!("{PREFIX}当液位等于真时，系统必须启动水泵。");
        assert_eq!(
            stage_counts(&type_failure),
            ([1, 0, 0], "APLS-E1401".into())
        );
    }

    #[test]
    fn zero_candidate_paths_report_specific_term_category_and_owner_errors() {
        for (sentence, expected) in [
            ("当“未声明属性”等于真时，系统必须启动水泵。", "APLS-E1201"),
            ("当“启动”等于真时，系统必须启动水泵。", "APLS-E1204"),
        ] {
            match compile(
                format!("{PREFIX}{sentence}").as_bytes(),
                "main.apls",
                CompileThrough::Check,
            ) {
                CompileOutcome::Rejected(diagnostics) => {
                    assert!(diagnostics.iter().any(|d| d.code == expected));
                    assert!(!diagnostics.iter().any(|d| d.code == "APLS-E1310"));
                }
                other => panic!("{sentence}: {other:?}"),
            }
        }

        let owner_mismatch = "本规范采用 APLS 简体中文语言版本 0.1。
“设备甲”是设备。
“设备乙”是设备。
“启动”是“设备甲”支持的动作。
“设备乙”的状态包括“待机”和“故障”，初始状态是“待机”。
当设备甲处于故障状态时，系统必须启动设备甲。";
        match compile(
            owner_mismatch.as_bytes(),
            "main.apls",
            CompileThrough::Check,
        ) {
            CompileOutcome::Rejected(diagnostics) => {
                assert!(diagnostics.iter().any(|d| d.code == "APLS-E1207"));
                assert!(!diagnostics.iter().any(|d| d.code == "APLS-E1310"));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn rejects_free_language_without_guessing() {
        let outcome = compile(
            "本规范采用 APLS 简体中文语言版本 0.1。\n温度高的时候适当降低一点速度。".as_bytes(),
            "main.apls",
            CompileThrough::Check,
        );
        match outcome {
            CompileOutcome::Rejected(diagnostics) => {
                let codes: BTreeSet<_> = diagnostics.iter().map(|d| d.code.as_str()).collect();
                assert!(codes.contains("APLS-E1301"));
                assert!(codes.contains("APLS-E1303"));
                assert!(!codes.contains("APLS-E1101"));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn rejects_non_nfc_source() {
        let source = "本规范采用 APLS 简体中文语言版本 0.1。\n说明：『e\u{301}』。";
        match compile(source.as_bytes(), "main.apls", CompileThrough::Parse) {
            CompileOutcome::Rejected(diagnostics) => assert_eq!(diagnostics[0].code, "APLS-E1004"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn syntax_node_overflow_identifies_the_exact_first_excess_node() {
        let document = format!("{PREFIX}当液位低于20%并且液位低于30%时，系统必须启动水泵。");
        let (source, sentences) = validate_and_split(document.as_bytes(), "main.apls").unwrap();
        let mut normal_ledger = Ledger::default();
        validate_profile(source, &sentences[0], &mut normal_ledger, "main.apls").unwrap();
        let mut records = Vec::new();
        let mut declaration_indexes = BTreeSet::new();
        for sentence in sentences.iter().skip(1) {
            for stream in
                lex_and_enumerate(sentence, 1, &[], &mut normal_ledger, "main.apls", source)
                    .unwrap()
            {
                if let Ok(parsed) = crate::apls_grammar::DeclarationEntryParser::new()
                    .parse(stream.into_iter().map(Ok))
                {
                    declaration_indexes.insert(sentence.index);
                    records.push(DeclRecord {
                        declaration: parsed.value,
                        span: sentence.span,
                    });
                }
            }
        }
        let graph = build_graph(&records, "main.apls", source).unwrap();
        let sentence = sentences
            .iter()
            .skip(1)
            .find(|sentence| !declaration_indexes.contains(&sentence.index))
            .unwrap();
        let candidate = lex_and_enumerate(
            sentence,
            2,
            &graph.symbols,
            &mut normal_ledger,
            "main.apls",
            source,
        )
        .unwrap()
        .into_iter()
        .find_map(|stream| {
            crate::apls_grammar::NormativeEntryParser::new()
                .parse(stream.into_iter().map(Ok))
                .ok()
        })
        .unwrap();
        let spans = normative_syntax_node_spans(&candidate.value, sentence.span, source);
        assert_eq!(spans.len(), 7);
        let value_at = source.find("30%").unwrap();
        let expected = ByteSpan::new(value_at, value_at + "30%".len());

        let mut overflow_ledger = Ledger::default();
        overflow_ledger.set_for_test(Resource::SyntaxNodes, 999_995);
        let mut error = None;
        for span in spans {
            if let Err(current) = overflow_ledger.add(
                Resource::SyntaxNodes,
                1,
                Some(sentence.index),
                Some(span),
                "main.apls",
                source,
            ) {
                error = Some(current);
                break;
            }
        }
        let error = error.expect("the second typed value must be the first excess node");
        let primary = error.primary_source_span.unwrap();
        assert_eq!(
            (primary.start_byte, primary.end_byte),
            (expected.start_byte, expected.end_byte)
        );
        assert_eq!(
            error.payload.unwrap()["resource"],
            "parse_candidate_syntax_nodes"
        );
    }

    #[test]
    fn public_diagnostic_limit_is_applied_after_total_ordering() {
        let diagnostics = (0..=crate::limits::MAX_DIAGNOSTICS)
            .rev()
            .map(|index| {
                let mut diagnostic = Diagnostic::source(
                    "APLS-E1101",
                    &format!("diagnostic-{index:04}"),
                    "main.apls",
                    "x",
                    ByteSpan::new(0, 1),
                );
                diagnostic.payload = Some(json!({"index": index}));
                diagnostic
            })
            .collect();
        let final_diagnostics = finalize_diagnostics(diagnostics);
        assert_eq!(final_diagnostics.len(), crate::limits::MAX_DIAGNOSTICS);
        let limit = final_diagnostics.last().unwrap();
        assert_eq!(limit.code, "APLS-T0007");
        assert_eq!(
            limit.payload.as_ref().unwrap()["resource"],
            "public_diagnostics"
        );
        assert_eq!(limit.payload.as_ref().unwrap()["observed"], 1001);
    }

    #[test]
    fn unicode_nfc_family_and_source_map_use_original_byte_boundaries() {
        for text in ["é", "à\u{0315}", "가", "🌱", "\u{0301}"] {
            let source = format!("本规范采用 APLS 简体中文语言版本 0.1。\r\n说明：『{text}』。");
            let value = ir(&source);
            for entry in value["source_map"].as_array().unwrap() {
                for provenance in entry["provenance"].as_array().unwrap() {
                    let sentence = &provenance["sentence_span"];
                    let start = sentence["start_byte"].as_u64().unwrap() as usize;
                    let end = sentence["end_byte"].as_u64().unwrap() as usize;
                    assert!(source.is_char_boundary(start));
                    assert!(source.is_char_boundary(end));
                    assert_eq!(&source[end - "。".len()..end], "。");
                }
            }
        }
        for text in ["e\u{301}", "a\u{315}\u{300}", "\u{1100}\u{1161}"] {
            let source = format!("本规范采用 APLS 简体中文语言版本 0.1。\n说明：『{text}』。");
            match compile(source.as_bytes(), "main.apls", CompileThrough::Parse) {
                CompileOutcome::Rejected(diagnostics) => {
                    assert_eq!(diagnostics[0].code, "APLS-E1004");
                    let span = diagnostics[0].primary_source_span.as_ref().unwrap();
                    assert!(source.is_char_boundary(span.start_byte));
                    assert!(source.is_char_boundary(span.end_byte));
                    assert_eq!(&source[span.start_byte..span.end_byte], text);
                }
                other => panic!("{text:?}: {other:?}"),
            }
        }
    }
}
