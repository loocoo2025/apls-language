//! Deterministic declaration indexing for the approved APLS 0.1 namespaces.

#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::ast::{Identifier, SpecMember};
use crate::lexer::ByteSpan;
use crate::parser::ParsedProgramSet;
use crate::source::LogicalPath;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NameErrorCode {
    UndefinedName,
    DuplicateDefinition,
    AmbiguousName,
    WrongCategory,
    OpenOrUnknownDependency,
}

impl NameErrorCode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::UndefinedName => "APLS-E2001",
            Self::DuplicateDefinition => "APLS-E2002",
            Self::AmbiguousName => "APLS-E2003",
            Self::WrongCategory => "APLS-E2005",
            Self::OpenOrUnknownDependency => "APLS-E4004",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NameFinding {
    pub code: NameErrorCode,
    pub logical_path: LogicalPath,
    pub span: ByteSpan,
    pub related_spans: Vec<(LogicalPath, ByteSpan)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NameFailure {
    pub findings: Vec<NameFinding>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct SymbolId(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SymbolKind {
    Spec,
    Dimension,
    Unit,
    Type,
    Enum,
    Record,
    EnumValue,
    Field,
    Domain,
    Component,
    Operation,
    Parameter,
    Transport,
    Event,
    Execution,
    Channel,
    StateMachine,
    State,
    Transition,
    Rule,
    Constraint,
    Safety,
    Acceptance,
    Given,
    Decision,
    Open,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Symbol {
    pub id: String,
    pub name: String,
    pub kind: SymbolKind,
    pub logical_path: LogicalPath,
    pub span: ByteSpan,
    pub parent: Option<SymbolId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AliasEntry {
    pub target: LogicalPath,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceScope {
    pub spec_name: String,
    pub spec_symbol: SymbolId,
    pub top_level: BTreeMap<String, Vec<SymbolId>>,
    pub aliases: BTreeMap<String, AliasEntry>,
    pub children: BTreeMap<SymbolId, BTreeMap<String, Vec<SymbolId>>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DeclarationIndex {
    pub symbols: Vec<Symbol>,
    pub scopes: BTreeMap<LogicalPath, SourceScope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IndexedProgram {
    pub parsed: ParsedProgramSet,
    pub declarations: DeclarationIndex,
}

pub(crate) fn index(parsed: ParsedProgramSet) -> Result<IndexedProgram, NameFailure> {
    let mut declarations = DeclarationIndex {
        symbols: Vec::new(),
        scopes: BTreeMap::new(),
    };
    let mut findings = Vec::new();
    let mut specs = BTreeMap::<String, (LogicalPath, ByteSpan)>::new();

    for file in &parsed.files {
        let spec = &file.program.unit.spec;
        if let Some((previous_path, previous_span)) = specs.get(&spec.name.value) {
            duplicate(
                &mut findings,
                &file.logical_path,
                spec.name.span,
                previous_path,
                *previous_span,
            );
        } else {
            specs.insert(
                spec.name.value.clone(),
                (file.logical_path.clone(), spec.name.span),
            );
        }
        let spec_symbol = push_symbol(
            &mut declarations,
            &file.logical_path,
            spec.name.value.clone(),
            spec.name.value.clone(),
            SymbolKind::Spec,
            spec.name.span,
            None,
        );
        declarations.scopes.insert(
            file.logical_path.clone(),
            SourceScope {
                spec_name: spec.name.value.clone(),
                spec_symbol,
                top_level: BTreeMap::new(),
                aliases: BTreeMap::new(),
                children: BTreeMap::new(),
            },
        );
    }

    for import in &parsed.imports {
        let scope = declarations
            .scopes
            .get_mut(&import.source)
            .expect("parsed import source must have a source scope");
        if let Some(previous) = scope.aliases.get(&import.alias) {
            duplicate(
                &mut findings,
                &import.source,
                import.alias_span,
                &import.source,
                previous.span,
            );
        } else {
            scope.aliases.insert(
                import.alias.clone(),
                AliasEntry {
                    target: import.target.clone(),
                    span: import.alias_span,
                },
            );
        }
    }

    for file in &parsed.files {
        let spec_name = file.program.unit.spec.name.value.clone();
        for member in &file.program.unit.spec.members {
            let Some((name, kind)) = top_level_declaration(member) else {
                continue;
            };
            let previous = declarations.scopes[&file.logical_path]
                .top_level
                .get(&name.value)
                .and_then(|ids| ids.first())
                .map(|id| declarations.symbols[id.0].span);
            if let Some(previous_span) = previous {
                duplicate(
                    &mut findings,
                    &file.logical_path,
                    name.span,
                    &file.logical_path,
                    previous_span,
                );
            }
            if let Some(alias) = declarations.scopes[&file.logical_path]
                .aliases
                .get(&name.value)
            {
                duplicate(
                    &mut findings,
                    &file.logical_path,
                    name.span,
                    &file.logical_path,
                    alias.span,
                );
            }

            let symbol_id = push_symbol(
                &mut declarations,
                &file.logical_path,
                format!("{spec_name}.{}", name.value),
                name.value.clone(),
                kind,
                name.span,
                None,
            );
            declarations
                .scopes
                .get_mut(&file.logical_path)
                .expect("scope exists")
                .top_level
                .entry(name.value.clone())
                .or_default()
                .push(symbol_id);
            index_nested(
                &mut declarations,
                &mut findings,
                &file.logical_path,
                &spec_name,
                symbol_id,
                member,
            );
        }
    }

    sort_findings(&mut findings);
    if findings.is_empty() {
        sort_symbols(&mut declarations);
        Ok(IndexedProgram {
            parsed,
            declarations,
        })
    } else {
        Err(NameFailure { findings })
    }
}

fn sort_symbols(declarations: &mut DeclarationIndex) {
    let old_symbols = std::mem::take(&mut declarations.symbols);
    let mut order: Vec<usize> = (0..old_symbols.len()).collect();
    order.sort_by(|left, right| {
        let left = &old_symbols[*left];
        let right = &old_symbols[*right];
        left.logical_path
            .cmp(&right.logical_path)
            .then(left.span.start.cmp(&right.span.start))
            .then(left.span.end.cmp(&right.span.end))
            .then(left.id.cmp(&right.id))
    });

    let mut remap = vec![SymbolId(0); old_symbols.len()];
    for (new_id, old_id) in order.iter().copied().enumerate() {
        remap[old_id] = SymbolId(new_id);
    }
    declarations.symbols = order
        .into_iter()
        .map(|old_id| {
            let mut symbol = old_symbols[old_id].clone();
            symbol.parent = symbol.parent.map(|parent| remap[parent.0]);
            symbol
        })
        .collect();

    for scope in declarations.scopes.values_mut() {
        scope.spec_symbol = remap[scope.spec_symbol.0];
        for ids in scope.top_level.values_mut() {
            remap_and_sort(ids, &remap);
        }
        let old_children = std::mem::take(&mut scope.children);
        for (old_parent, mut names) in old_children {
            for ids in names.values_mut() {
                remap_and_sort(ids, &remap);
            }
            scope.children.insert(remap[old_parent.0], names);
        }
    }
}

fn remap_and_sort(ids: &mut [SymbolId], remap: &[SymbolId]) {
    for id in ids.iter_mut() {
        *id = remap[id.0];
    }
    ids.sort();
}

fn top_level_declaration(member: &SpecMember) -> Option<(&Identifier, SymbolKind)> {
    Some(match member {
        SpecMember::Intent(_) => return None,
        SpecMember::Dimension(value) => (&value.name, SymbolKind::Dimension),
        SpecMember::Unit(value) => (&value.name, SymbolKind::Unit),
        SpecMember::Type(value) => (&value.name, SymbolKind::Type),
        SpecMember::Enum(value) => (&value.name, SymbolKind::Enum),
        SpecMember::Record(value) => (&value.name, SymbolKind::Record),
        SpecMember::Domain(value) => (&value.name, SymbolKind::Domain),
        SpecMember::Component(value) => (&value.name, SymbolKind::Component),
        SpecMember::Operation(value) => (&value.name, SymbolKind::Operation),
        SpecMember::Transport(value) => (&value.name, SymbolKind::Transport),
        SpecMember::Event(value) => (&value.name, SymbolKind::Event),
        SpecMember::Execution(value) => (&value.name, SymbolKind::Execution),
        SpecMember::Channel(value) => (&value.name, SymbolKind::Channel),
        SpecMember::StateMachine(value) => (&value.name, SymbolKind::StateMachine),
        SpecMember::Rule(value) => (&value.name, SymbolKind::Rule),
        SpecMember::Constraint(value) => (&value.name, SymbolKind::Constraint),
        SpecMember::Safety(value) => (&value.name, SymbolKind::Safety),
        SpecMember::Acceptance(value) => (&value.name, SymbolKind::Acceptance),
        SpecMember::Decision(value) => (&value.name, SymbolKind::Decision),
        SpecMember::Open(value) => (&value.name, SymbolKind::Open),
        SpecMember::Unknown(value) => (&value.name, SymbolKind::Unknown),
    })
}

fn index_nested(
    declarations: &mut DeclarationIndex,
    findings: &mut Vec<NameFinding>,
    path: &LogicalPath,
    spec_name: &str,
    parent: SymbolId,
    member: &SpecMember,
) {
    match member {
        SpecMember::Enum(value) => {
            for variant in &value.variants {
                add_child(
                    declarations,
                    findings,
                    path,
                    spec_name,
                    parent,
                    variant,
                    SymbolKind::EnumValue,
                );
            }
        }
        SpecMember::Record(value) => {
            for field in &value.fields {
                add_child(
                    declarations,
                    findings,
                    path,
                    spec_name,
                    parent,
                    &field.name,
                    SymbolKind::Field,
                );
            }
        }
        SpecMember::Operation(value) => {
            for parameter in &value.parameters {
                add_child(
                    declarations,
                    findings,
                    path,
                    spec_name,
                    parent,
                    &parameter.name,
                    SymbolKind::Parameter,
                );
            }
        }
        SpecMember::StateMachine(value) => {
            for state in &value.states {
                add_child(
                    declarations,
                    findings,
                    path,
                    spec_name,
                    parent,
                    &state.name,
                    SymbolKind::State,
                );
            }
            for transition in &value.transitions {
                add_child(
                    declarations,
                    findings,
                    path,
                    spec_name,
                    parent,
                    &transition.name,
                    SymbolKind::Transition,
                );
            }
        }
        SpecMember::Acceptance(value) => {
            for given in &value.given {
                add_child(
                    declarations,
                    findings,
                    path,
                    spec_name,
                    parent,
                    &given.name,
                    SymbolKind::Given,
                );
                let scope = &declarations.scopes[path];
                if let Some(id) = scope
                    .top_level
                    .get(&given.name.value)
                    .and_then(|ids| ids.first())
                {
                    duplicate(
                        findings,
                        path,
                        given.name.span,
                        path,
                        declarations.symbols[id.0].span,
                    );
                }
                if let Some(alias) = scope.aliases.get(&given.name.value) {
                    duplicate(findings, path, given.name.span, path, alias.span);
                }
            }
        }
        _ => {}
    }
}

fn add_child(
    declarations: &mut DeclarationIndex,
    findings: &mut Vec<NameFinding>,
    path: &LogicalPath,
    spec_name: &str,
    parent: SymbolId,
    name: &Identifier,
    kind: SymbolKind,
) {
    let previous = declarations.scopes[path]
        .children
        .get(&parent)
        .and_then(|children| children.get(&name.value))
        .and_then(|ids| ids.first())
        .map(|id| declarations.symbols[id.0].span);
    if let Some(previous_span) = previous {
        duplicate(findings, path, name.span, path, previous_span);
    }
    let parent_name = declarations.symbols[parent.0].name.clone();
    let child_id = push_symbol(
        declarations,
        path,
        format!("{spec_name}.{parent_name}.{}", name.value),
        name.value.clone(),
        kind,
        name.span,
        Some(parent),
    );
    declarations
        .scopes
        .get_mut(path)
        .expect("scope exists")
        .children
        .entry(parent)
        .or_default()
        .entry(name.value.clone())
        .or_default()
        .push(child_id);
}

fn push_symbol(
    declarations: &mut DeclarationIndex,
    path: &LogicalPath,
    id: String,
    name: String,
    kind: SymbolKind,
    span: ByteSpan,
    parent: Option<SymbolId>,
) -> SymbolId {
    let symbol_id = SymbolId(declarations.symbols.len());
    declarations.symbols.push(Symbol {
        id,
        name,
        kind,
        logical_path: path.clone(),
        span,
        parent,
    });
    symbol_id
}

fn duplicate(
    findings: &mut Vec<NameFinding>,
    path: &LogicalPath,
    span: ByteSpan,
    related_path: &LogicalPath,
    related_span: ByteSpan,
) {
    findings.push(NameFinding {
        code: NameErrorCode::DuplicateDefinition,
        logical_path: path.clone(),
        span,
        related_spans: vec![(related_path.clone(), related_span)],
    });
}

pub(crate) fn sort_findings(findings: &mut [NameFinding]) {
    for finding in findings.iter_mut() {
        finding.related_spans.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then(left.1.start.cmp(&right.1.start))
                .then(left.1.end.cmp(&right.1.end))
        });
    }
    findings.sort_by(|left, right| {
        left.logical_path
            .cmp(&right.logical_path)
            .then(left.span.start.cmp(&right.span.start))
            .then(left.span.end.cmp(&right.span.end))
            .then(left.code.as_str().cmp(right.code.as_str()))
    });
}
