//! Unique name binding for the approved APLS 0.1 reference forms.

#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::ast::*;
use crate::index::{
    DeclarationIndex, IndexedProgram, NameErrorCode, NameFailure, NameFinding, SymbolId,
    SymbolKind, index, sort_findings,
};
use crate::lexer::ByteSpan;
use crate::parser::{ParsedBundleFailure, parse_bundle};
use crate::source::{LoadedBundle, LogicalPath};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BoundReference {
    pub logical_path: LogicalPath,
    pub span: ByteSpan,
    pub target: SymbolId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BoundProgram {
    pub indexed: IndexedProgram,
    pub references: Vec<BoundReference>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum BindBundleFailure {
    Parse(ParsedBundleFailure),
    Name(NameFailure),
}

pub(crate) fn bind_bundle(bundle: &LoadedBundle) -> Result<BoundProgram, BindBundleFailure> {
    let parsed = parse_bundle(bundle).map_err(BindBundleFailure::Parse)?;
    let indexed = index(parsed).map_err(BindBundleFailure::Name)?;
    bind(indexed).map_err(BindBundleFailure::Name)
}

pub(crate) fn bind(indexed: IndexedProgram) -> Result<BoundProgram, NameFailure> {
    let (references, mut findings) = {
        let mut resolver = Resolver {
            declarations: &indexed.declarations,
            references: Vec::new(),
            findings: Vec::new(),
        };
        for file in &indexed.parsed.files {
            resolver.resolve_file(&file.logical_path, &file.program);
        }
        (resolver.references, resolver.findings)
    };
    sort_findings(&mut findings);
    if findings.is_empty() {
        Ok(BoundProgram {
            indexed,
            references,
        })
    } else {
        Err(NameFailure { findings })
    }
}

#[derive(Clone, Copy)]
enum Expected {
    Exact(SymbolKind),
    Type,
    SafetyTarget,
}

impl Expected {
    fn accepts(self, kind: SymbolKind) -> bool {
        match self {
            Self::Exact(expected) => kind == expected,
            Self::Type => matches!(
                kind,
                SymbolKind::Type | SymbolKind::Enum | SymbolKind::Record
            ),
            Self::SafetyTarget => matches!(
                kind,
                SymbolKind::Operation
                    | SymbolKind::Event
                    | SymbolKind::Channel
                    | SymbolKind::Execution
                    | SymbolKind::StateMachine
                    | SymbolKind::State
                    | SymbolKind::Rule
            ),
        }
    }
}

struct Resolver<'a> {
    declarations: &'a DeclarationIndex,
    references: Vec<BoundReference>,
    findings: Vec<NameFinding>,
}

impl Resolver<'_> {
    fn resolve_file(&mut self, path: &LogicalPath, program: &ParsedProgram) {
        for member in &program.unit.spec.members {
            self.resolve_member(path, member);
        }
    }

    fn resolve_member(&mut self, path: &LogicalPath, member: &SpecMember) {
        match member {
            SpecMember::Intent(_)
            | SpecMember::Enum(_)
            | SpecMember::Domain(_)
            | SpecMember::Component(_)
            | SpecMember::Transport(_)
            | SpecMember::Decision(_)
            | SpecMember::Open(_)
            | SpecMember::Unknown(_) => {}
            SpecMember::Dimension(value) => {
                self.resolve_identifier(path, &value.base_unit, Expected::Exact(SymbolKind::Unit));
            }
            SpecMember::Unit(value) => {
                self.resolve_name(
                    path,
                    &value.dimension,
                    Expected::Exact(SymbolKind::Dimension),
                );
            }
            SpecMember::Type(value) => self.resolve_type(path, &value.target),
            SpecMember::Record(value) => {
                for field in &value.fields {
                    self.resolve_type(path, &field.type_ref);
                    if let Some(default) = &field.default {
                        self.resolve_const(path, default);
                    }
                }
            }
            SpecMember::Operation(value) => {
                for parameter in &value.parameters {
                    self.resolve_type(path, &parameter.type_ref);
                }
                if let Some(result) = &value.returns {
                    self.resolve_type(path, result);
                }
            }
            SpecMember::Event(value) => {
                if let Some(payload) = &value.payload {
                    self.resolve_type(path, payload);
                }
                if let Some(source) = &value.source {
                    self.resolve_name(path, source, Expected::Exact(SymbolKind::Execution));
                }
            }
            SpecMember::Execution(value) => {
                self.resolve_name(path, &value.domain, Expected::Exact(SymbolKind::Domain));
                if let Some(owner) = &value.owner {
                    self.resolve_name(path, owner, Expected::Exact(SymbolKind::Component));
                }
                match &value.trigger {
                    TriggerExpr::Call(name) => {
                        self.resolve_name(path, name, Expected::Exact(SymbolKind::Operation));
                    }
                    TriggerExpr::Event(name) => {
                        self.resolve_name(path, name, Expected::Exact(SymbolKind::Event));
                    }
                    TriggerExpr::Message(name) => {
                        self.resolve_name(path, name, Expected::Exact(SymbolKind::Channel));
                    }
                    TriggerExpr::Timer(expression) => {
                        self.resolve_expression(path, expression, None)
                    }
                    TriggerExpr::Interrupt(name) => {
                        self.resolve_name(path, name, Expected::Exact(SymbolKind::Execution));
                    }
                    TriggerExpr::StateChange(name) => {
                        self.resolve_name(path, name, Expected::Exact(SymbolKind::State));
                    }
                }
                if let ScheduleExpr::Periodic(arguments) = &value.schedule {
                    self.resolve_arguments(path, arguments, None);
                }
            }
            SpecMember::Channel(value) => {
                self.resolve_name(path, &value.from, Expected::Exact(SymbolKind::Execution));
                self.resolve_name(path, &value.to, Expected::Exact(SymbolKind::Execution));
                self.resolve_type(path, &value.payload);
                self.resolve_name(
                    path,
                    &value.transport,
                    Expected::Exact(SymbolKind::Transport),
                );
                if let Some(timeout) = &value.timeout {
                    self.resolve_expression(path, timeout, None);
                }
            }
            SpecMember::StateMachine(value) => self.resolve_state_machine(path, value),
            SpecMember::Rule(value) => {
                self.resolve_expression(path, &value.when, None);
                if let Some(duration) = &value.hold_for {
                    self.resolve_expression(path, duration, None);
                }
                self.resolve_actions(path, &value.then, None);
            }
            SpecMember::Constraint(value) => self.resolve_expression(path, &value.assertion, None),
            SpecMember::Safety(value) => {
                self.resolve_expression(path, &value.when, None);
                for target in &value.prohibited {
                    self.resolve_name(path, target, Expected::SafetyTarget);
                }
                self.resolve_actions(path, &value.then, None);
            }
            SpecMember::Acceptance(value) => self.resolve_acceptance(path, value),
        }
    }

    fn resolve_state_machine(&mut self, path: &LogicalPath, machine: &StateMachineDecl) {
        let Some(machine_id) = self.unique_top(path, &machine.name.value) else {
            return;
        };
        self.resolve_child_identifier(path, machine_id, &machine.initial, SymbolKind::State);
        for transition in &machine.transitions {
            self.resolve_child_identifier(path, machine_id, &transition.from, SymbolKind::State);
            self.resolve_name(path, &transition.on, Expected::Exact(SymbolKind::Event));
            if let Some(guard) = &transition.guard {
                self.resolve_expression(path, guard, None);
            }
            self.resolve_child_identifier(path, machine_id, &transition.to, SymbolKind::State);
            if let Some(actions) = &transition.then {
                self.resolve_actions(path, actions, None);
            }
        }
    }

    fn resolve_acceptance(&mut self, path: &LogicalPath, acceptance: &AcceptanceDecl) {
        let given_owner = self.unique_top(path, &acceptance.name.value);
        for given in &acceptance.given {
            self.resolve_type(path, &given.type_ref);
            self.resolve_const(path, &given.value);
        }
        self.resolve_actions(path, &acceptance.when, given_owner);
        self.resolve_expression(path, &acceptance.expect, given_owner);
        if let Some(duration) = &acceptance.within {
            self.resolve_expression(path, duration, given_owner);
        }
    }

    fn resolve_type(&mut self, path: &LogicalPath, type_ref: &TypeRef) {
        match &type_ref.value {
            TypeRefKind::Primitive(_) => {}
            TypeRefKind::Named(name) => {
                self.resolve_name(path, name, Expected::Type);
            }
            TypeRefKind::List(inner) | TypeRefKind::Optional(inner) => {
                self.resolve_type(path, inner)
            }
            TypeRefKind::Quantity { dimension, unit } => {
                self.resolve_name(path, dimension, Expected::Exact(SymbolKind::Dimension));
                self.resolve_name(path, unit, Expected::Exact(SymbolKind::Unit));
            }
        }
    }

    fn resolve_const(&mut self, path: &LogicalPath, value: &ConstExpr) {
        match &value.value {
            ConstExprKind::Literal(value) => self.resolve_literal(path, value),
            ConstExprKind::List(values) => {
                for value in values {
                    self.resolve_literal(path, value);
                }
            }
        }
    }

    fn resolve_literal(&mut self, path: &LogicalPath, value: &Literal) {
        if let LiteralKind::Numeric(number) = &value.value {
            if let Some(unit) = &number.unit {
                self.resolve_name(path, unit, Expected::Exact(SymbolKind::Unit));
            }
        }
    }

    fn resolve_expression(
        &mut self,
        path: &LogicalPath,
        expression: &Expression,
        given_owner: Option<SymbolId>,
    ) {
        match &expression.value {
            ExpressionKind::Literal(value) => self.resolve_literal(path, value),
            ExpressionKind::Name(name) if name.value.len() == 1 => {
                if let Some(owner) = given_owner {
                    self.resolve_child(
                        path,
                        owner,
                        &name.value[0],
                        name.span,
                        Expected::Exact(SymbolKind::Given),
                    );
                } else {
                    self.undefined(path, name.span);
                }
            }
            ExpressionKind::Name(name) => {
                self.resolve_name(path, name, Expected::Exact(SymbolKind::EnumValue));
            }
            ExpressionKind::Call { target, arguments } => {
                self.resolve_name(path, target, Expected::Exact(SymbolKind::Operation));
                self.resolve_arguments(path, arguments, given_owner);
            }
            ExpressionKind::Unary { operand, .. } => {
                self.resolve_expression(path, operand, given_owner)
            }
            ExpressionKind::Binary { left, right, .. } => {
                self.resolve_expression(path, left, given_owner);
                self.resolve_expression(path, right, given_owner);
            }
        }
    }

    fn resolve_actions(
        &mut self,
        path: &LogicalPath,
        actions: &[Action],
        given_owner: Option<SymbolId>,
    ) {
        for action in actions {
            match &action.value {
                ActionKind::Emit { target, arguments } => {
                    self.resolve_name(path, target, Expected::Exact(SymbolKind::Event));
                    self.resolve_arguments(path, arguments, given_owner);
                }
                ActionKind::Invoke { target, arguments } => {
                    self.resolve_name(path, target, Expected::Exact(SymbolKind::Operation));
                    self.resolve_arguments(path, arguments, given_owner);
                }
                ActionKind::Transition { machine, state } => {
                    let machine_id =
                        self.resolve_name(path, machine, Expected::Exact(SymbolKind::StateMachine));
                    let state_id = if state.value.len() == 1 {
                        machine_id.and_then(|owner| {
                            self.resolve_child(
                                path,
                                owner,
                                &state.value[0],
                                state.span,
                                Expected::Exact(SymbolKind::State),
                            )
                        })
                    } else {
                        self.resolve_name(path, state, Expected::Exact(SymbolKind::State))
                    };
                    if let (Some(machine_id), Some(state_id)) = (machine_id, state_id) {
                        if self.declarations.symbols[state_id.0].parent != Some(machine_id) {
                            self.wrong_category(path, state.span, state_id);
                        }
                    }
                }
            }
        }
    }

    fn resolve_arguments(
        &mut self,
        path: &LogicalPath,
        arguments: &[NamedArg],
        given_owner: Option<SymbolId>,
    ) {
        let mut names = BTreeMap::<&str, ByteSpan>::new();
        for argument in arguments {
            if let Some(previous) = names.insert(&argument.name.value, argument.name.span) {
                self.findings.push(NameFinding {
                    code: NameErrorCode::DuplicateDefinition,
                    logical_path: path.clone(),
                    span: argument.name.span,
                    related_spans: vec![(path.clone(), previous)],
                });
            }
            self.resolve_expression(path, &argument.value, given_owner);
        }
    }

    fn resolve_identifier(
        &mut self,
        path: &LogicalPath,
        name: &Identifier,
        expected: Expected,
    ) -> Option<SymbolId> {
        self.resolve_parts(path, std::slice::from_ref(&name.value), name.span, expected)
    }

    fn resolve_name(
        &mut self,
        path: &LogicalPath,
        name: &QualifiedName,
        expected: Expected,
    ) -> Option<SymbolId> {
        self.resolve_parts(path, &name.value, name.span, expected)
    }

    fn resolve_parts(
        &mut self,
        path: &LogicalPath,
        parts: &[String],
        span: ByteSpan,
        expected: Expected,
    ) -> Option<SymbolId> {
        let candidates = self.raw_candidates(path, parts);
        self.finish_candidates(path, span, &candidates, expected)
    }

    fn raw_candidates(&self, path: &LogicalPath, parts: &[String]) -> Vec<SymbolId> {
        let Some(scope) = self.declarations.scopes.get(path) else {
            return Vec::new();
        };
        match parts {
            [name] => scope.top_level.get(name).cloned().unwrap_or_default(),
            [prefix, name] => {
                if let Some(alias) = scope.aliases.get(prefix) {
                    return self
                        .declarations
                        .scopes
                        .get(&alias.target)
                        .and_then(|target| target.top_level.get(name))
                        .cloned()
                        .unwrap_or_default();
                }
                let parents = scope.top_level.get(prefix).cloned().unwrap_or_default();
                if parents.len() != 1 {
                    return parents;
                }
                scope
                    .children
                    .get(&parents[0])
                    .and_then(|children| children.get(name))
                    .cloned()
                    .unwrap_or_default()
            }
            [alias_name, parent_name, child_name] => {
                let Some(alias) = scope.aliases.get(alias_name) else {
                    return Vec::new();
                };
                let Some(target) = self.declarations.scopes.get(&alias.target) else {
                    return Vec::new();
                };
                let parents = target
                    .top_level
                    .get(parent_name)
                    .cloned()
                    .unwrap_or_default();
                if parents.len() != 1 {
                    return parents;
                }
                target
                    .children
                    .get(&parents[0])
                    .and_then(|children| children.get(child_name))
                    .cloned()
                    .unwrap_or_default()
            }
            _ => Vec::new(),
        }
    }

    fn resolve_child_identifier(
        &mut self,
        path: &LogicalPath,
        owner: SymbolId,
        name: &Identifier,
        kind: SymbolKind,
    ) -> Option<SymbolId> {
        self.resolve_child(path, owner, &name.value, name.span, Expected::Exact(kind))
    }

    fn resolve_child(
        &mut self,
        path: &LogicalPath,
        owner: SymbolId,
        name: &str,
        span: ByteSpan,
        expected: Expected,
    ) -> Option<SymbolId> {
        let candidates = self.declarations.scopes[path]
            .children
            .get(&owner)
            .and_then(|children| children.get(name))
            .cloned()
            .unwrap_or_default();
        self.finish_candidates(path, span, &candidates, expected)
    }

    fn finish_candidates(
        &mut self,
        path: &LogicalPath,
        span: ByteSpan,
        candidates: &[SymbolId],
        expected: Expected,
    ) -> Option<SymbolId> {
        let [candidate] = candidates else {
            if candidates.is_empty() {
                self.undefined(path, span);
            } else {
                self.findings.push(NameFinding {
                    code: NameErrorCode::AmbiguousName,
                    logical_path: path.clone(),
                    span,
                    related_spans: candidates
                        .iter()
                        .map(|id| {
                            let symbol = &self.declarations.symbols[id.0];
                            (symbol.logical_path.clone(), symbol.span)
                        })
                        .collect(),
                });
            }
            return None;
        };
        let symbol = &self.declarations.symbols[candidate.0];
        if matches!(symbol.kind, SymbolKind::Open | SymbolKind::Unknown) {
            self.findings.push(NameFinding {
                code: NameErrorCode::OpenOrUnknownDependency,
                logical_path: path.clone(),
                span,
                related_spans: vec![(symbol.logical_path.clone(), symbol.span)],
            });
            return None;
        }
        if !expected.accepts(symbol.kind) {
            self.wrong_category(path, span, *candidate);
            return None;
        }
        self.references.push(BoundReference {
            logical_path: path.clone(),
            span,
            target: *candidate,
        });
        Some(*candidate)
    }

    fn unique_top(&self, path: &LogicalPath, name: &str) -> Option<SymbolId> {
        let ids = self.declarations.scopes[path].top_level.get(name)?;
        (ids.len() == 1).then_some(ids[0])
    }

    fn undefined(&mut self, path: &LogicalPath, span: ByteSpan) {
        self.findings.push(NameFinding {
            code: NameErrorCode::UndefinedName,
            logical_path: path.clone(),
            span,
            related_spans: Vec::new(),
        });
    }

    fn wrong_category(&mut self, path: &LogicalPath, span: ByteSpan, target: SymbolId) {
        let symbol = &self.declarations.symbols[target.0];
        self.findings.push(NameFinding {
            code: NameErrorCode::WrongCategory,
            logical_path: path.clone(),
            span,
            related_spans: vec![(symbol.logical_path.clone(), symbol.span)],
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::index::NameErrorCode;

    use super::*;
    use crate::parser::{ParsedFile, ParsedProgramSet, parse};
    use crate::source::{ImportEdge, LogicalPath};

    fn parsed(sources: &[(&str, &str)], imports: Vec<ImportEdge>) -> ParsedProgramSet {
        ParsedProgramSet {
            files: sources
                .iter()
                .map(|(path, source)| ParsedFile {
                    logical_path: LogicalPath::parse(path).unwrap(),
                    program: parse(source.as_bytes()).unwrap(),
                })
                .collect(),
            imports,
        }
    }

    fn import(source: &str, target: &str, alias: &str) -> ImportEdge {
        ImportEdge {
            source: LogicalPath::parse(source).unwrap(),
            target: LogicalPath::parse(target).unwrap(),
            alias: alias.to_owned(),
            path_span: ByteSpan { start: 0, end: 0 },
            alias_span: ByteSpan { start: 0, end: 0 },
        }
    }

    fn codes(failure: &NameFailure) -> Vec<NameErrorCode> {
        failure
            .findings
            .iter()
            .map(|finding| finding.code)
            .collect()
    }

    #[test]
    fn binds_local_imported_nested_and_acceptance_given_names() {
        let shared = r#"apls "0.1"; spec Shared { version "1";
            type Payload = string;
            enum Mode { Auto, Manual }
            operation start(input: Payload) { kind action; }
        }"#;
        let main = r#"apls "0.1"; import "shared.apls" as Shared; spec Main { version "1";
            domain Host { model os; supports [process]; }
            component Owner { responsibility "own"; }
            event Tick { payload Shared.Payload; }
            execution Loop { kind process; domain Host; owner Owner; trigger call(Shared.start); schedule event_driven; }
            state_machine Life { initial Idle; state Idle; state Done terminal;
                transition Go { from Idle; on Tick; to Done; then { emit Tick(); } }
            }
            acceptance A { given expected: Shared.Payload = "ok";
                when { invoke Shared.start(input: expected); transition Life to Done; }
                expect expected == Shared.Mode.Auto;
            }
        }"#;
        let indexed = index(parsed(
            &[("main.apls", main), ("shared.apls", shared)],
            vec![import("main.apls", "shared.apls", "Shared")],
        ))
        .unwrap();
        let bound = bind(indexed).unwrap();
        assert!(bound.references.len() >= 16);
    }

    #[test]
    fn rejects_duplicate_top_level_nested_and_argument_names_as_e2002() {
        let source = r#"apls "0.1"; spec S { version "1";
            type T = string; enum T { A }
            operation op(x: int, x: int) { kind action; }
            event E { }
            state_machine M { initial X; state X;
                transition X { from X; on E; to X; }
            }
        }"#;
        let failure = index(parsed(&[("s.apls", source)], Vec::new())).unwrap_err();
        assert!(
            codes(&failure)
                .iter()
                .all(|code| *code == NameErrorCode::DuplicateDefinition)
        );
        assert!(failure.findings.len() >= 2);

        let arguments = r#"apls "0.1"; spec S { version "1";
            operation op(x: int) { kind action; }
            rule R { when true; then { invoke op(x: 1, x: 2); } }
        }"#;
        let indexed = index(parsed(&[("s.apls", arguments)], Vec::new())).unwrap();
        assert_eq!(
            codes(&bind(indexed).unwrap_err()),
            vec![NameErrorCode::DuplicateDefinition]
        );

        let main = r#"apls "0.1"; import "shared.apls" as Shared; spec Main { version "1";
            type Shared = string;
        }"#;
        let shared = r#"apls "0.1"; spec SharedSpec { version "1"; }"#;
        let failure = index(parsed(
            &[("main.apls", main), ("shared.apls", shared)],
            vec![import("main.apls", "shared.apls", "Shared")],
        ))
        .unwrap_err();
        assert_eq!(codes(&failure), vec![NameErrorCode::DuplicateDefinition]);
    }

    #[test]
    fn rejects_undefined_and_wrong_category_references() {
        let undefined = r#"apls "0.1"; spec S { version "1"; type T = Missing; }"#;
        let indexed = index(parsed(&[("s.apls", undefined)], Vec::new())).unwrap();
        assert_eq!(
            codes(&bind(indexed).unwrap_err()),
            vec![NameErrorCode::UndefinedName]
        );

        let wrong = r#"apls "0.1"; spec S { version "1";
            component C { responsibility "x"; }
            event E { payload C; }
        }"#;
        let indexed = index(parsed(&[("s.apls", wrong)], Vec::new())).unwrap();
        assert_eq!(
            codes(&bind(indexed).unwrap_err()),
            vec![NameErrorCode::WrongCategory]
        );
    }

    #[test]
    fn rejects_normative_dependency_on_open_or_unknown_as_e4004() {
        let source = r#"apls "0.1"; spec S { version "1";
            open Q { question "which type"; }
            type T = Q;
        }"#;
        let indexed = index(parsed(&[("s.apls", source)], Vec::new())).unwrap();
        assert_eq!(
            codes(&bind(indexed).unwrap_err()),
            vec![NameErrorCode::OpenOrUnknownDependency]
        );
    }

    #[test]
    fn defensive_multiple_candidates_are_e2003_and_never_produce_a_bound_program() {
        let source = r#"apls "0.1"; spec S { version "1";
            type T = string; type U = T;
        }"#;
        let mut indexed = index(parsed(&[("s.apls", source)], Vec::new())).unwrap();
        let path = LogicalPath::parse("s.apls").unwrap();
        let t = indexed.declarations.scopes[&path].top_level["T"][0];
        indexed
            .declarations
            .scopes
            .get_mut(&path)
            .unwrap()
            .top_level
            .get_mut("T")
            .unwrap()
            .push(t);
        assert_eq!(
            codes(&bind(indexed).unwrap_err()),
            vec![NameErrorCode::AmbiguousName]
        );
    }
}
