//! LALRPOP adapter for the unique, complete APLS 0.1 grammar.

#![allow(dead_code)]

use lalrpop_util::ParseError;

use crate::ast::{BinaryOperator, Expression, ExpressionKind, ParsedProgram, spanned};
use crate::lexer::{ByteSpan, LexFinding, TokenKind, lex_with_token_limit};
use crate::limits::MAX_TOKENS;
use crate::source::{ImportEdge, LoadedBundle, LogicalPath};

lalrpop_util::lalrpop_mod!(
    #[allow(unused_imports, clippy::all)]
    apls_grammar
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParseErrorCode {
    UnexpectedToken,
    MissingOrMisorderedStructure,
}

impl ParseErrorCode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::UnexpectedToken => "APLS-E1101",
            Self::MissingOrMisorderedStructure => "APLS-E1103",
        }
    }

    pub(crate) fn message(self) -> &'static str {
        match self {
            Self::UnexpectedToken => "token does not match the APLS 0.1 grammar",
            Self::MissingOrMisorderedStructure => {
                "required structure is missing, repeated, or out of order"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParseFinding {
    pub code: ParseErrorCode,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParseFailure {
    Lex(LexFinding),
    Syntax(ParseFinding),
}

pub(crate) fn parse(source: &[u8]) -> Result<ParsedProgram, ParseFailure> {
    parse_with_token_limit(source, MAX_TOKENS).map(|result| result.program)
}

pub(crate) struct ParsedSource {
    pub program: ParsedProgram,
    pub token_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedFile {
    pub logical_path: LogicalPath,
    pub program: ParsedProgram,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedProgramSet {
    pub files: Vec<ParsedFile>,
    pub imports: Vec<ImportEdge>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedBundleFailure {
    pub logical_path: LogicalPath,
    pub failure: ParseFailure,
}

pub(crate) fn parse_bundle(bundle: &LoadedBundle) -> Result<ParsedProgramSet, ParsedBundleFailure> {
    let mut files = Vec::with_capacity(bundle.sources.len());
    for source in &bundle.sources {
        let program = parse(&source.bytes).map_err(|failure| ParsedBundleFailure {
            logical_path: source.logical_path.clone(),
            failure,
        })?;
        files.push(ParsedFile {
            logical_path: source.logical_path.clone(),
            program,
        });
    }
    Ok(ParsedProgramSet {
        files,
        imports: bundle.imports.clone(),
    })
}

pub(crate) fn parse_with_token_limit(
    source: &[u8],
    token_limit: usize,
) -> Result<ParsedSource, ParseFailure> {
    let lexed = lex_with_token_limit(source, token_limit).map_err(ParseFailure::Lex)?;
    let token_count = lexed.tokens.len();
    let tokens = lexed
        .tokens
        .iter()
        .map(|token| Ok((token.span.start, token.kind, token.span.end)));

    apls_grammar::CompilationUnitParser::new()
        .parse(lexed.source(), tokens)
        .map(|unit| ParsedSource {
            program: ParsedProgram { unit },
            token_count,
        })
        .map_err(normalize_parse_error)
}

fn normalize_parse_error(error: ParseError<usize, TokenKind, LexFinding>) -> ParseFailure {
    match error {
        ParseError::User { error } => ParseFailure::Lex(error),
        ParseError::InvalidToken { location } => {
            syntax_finding(ParseErrorCode::UnexpectedToken, location, location)
        }
        ParseError::UnrecognizedEof { location, .. } => syntax_finding(
            ParseErrorCode::MissingOrMisorderedStructure,
            location,
            location,
        ),
        ParseError::UnrecognizedToken { token, .. } | ParseError::ExtraToken { token } => {
            let code = if matches!(token.1, TokenKind::Keyword(_)) {
                ParseErrorCode::MissingOrMisorderedStructure
            } else {
                ParseErrorCode::UnexpectedToken
            };
            syntax_finding(code, token.0, token.2)
        }
    }
}

fn syntax_finding(code: ParseErrorCode, start: usize, end: usize) -> ParseFailure {
    ParseFailure::Syntax(ParseFinding {
        code,
        span: ByteSpan { start, end },
    })
}

pub(super) fn fold_binary(
    first: Expression,
    operator: BinaryOperator,
    rest: Vec<Expression>,
) -> Expression {
    rest.into_iter().fold(first, |left, right| {
        let start = left.span.start;
        let end = right.span.end;
        spanned(
            start,
            ExpressionKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            end,
        )
    })
}

pub(super) fn fold_binary_pairs(
    first: Expression,
    rest: Vec<(BinaryOperator, Expression)>,
) -> Expression {
    rest.into_iter().fold(first, |left, (operator, right)| {
        let start = left.span.start;
        let end = right.span.end;
        spanned(
            start,
            ExpressionKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            end,
        )
    })
}

pub(super) fn decode_string_literal(raw: &str) -> String {
    serde_json::from_str(raw).expect("the deterministic lexer accepted only valid JSON strings")
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOperator, ExpressionKind, LiteralKind, SpecMember};
    use crate::lexer::LexErrorCode;

    use super::{ParseErrorCode, ParseFailure, parse};

    const COMPLETE_SOURCE: &str = r#"
apls "0.1";
import "shared.apls" as Shared;
spec Demo {
  version "1.0";
  intent "complete grammar";
  dimension Time { base_unit second; rationale "SI"; }
  unit ms : Time { symbol "ms"; scale 0.001; offset -0; rationale "milliseconds"; }
  type Name = string;
  enum Mode { Auto, Manual }
  record Reading { value: quantity<Time, ms> = 1 @ ms; tags: list<string> = ["a", "b"]; }
  domain Host { model os; supports [process, thread]; rationale "host"; }
  component Controller { responsibility "control"; rationale "owner"; }
  operation ready(input: optional<Name>) returns bool { kind pure; rationale "query"; }
  transport Local { kind in_process; rationale "local"; }
  event Tick { payload int; source Shared.Clock; }
  execution Main {
    kind process; domain Host; owner Controller;
    trigger timer(10 @ ms); schedule periodic(period: 10 @ ms);
    responsibility "run";
  }
  channel Samples {
    from Main; to Controller; payload Reading; mode queue; transport Local;
    ordering fifo; delivery at_least_once; capacity 16; timeout 5 @ ms;
  }
  state_machine Lifecycle {
    initial Idle;
    state Idle;
    state Running terminal;
    transition Start { from Idle; on Tick; guard ready(input: "x"); to Running; priority 1; then { emit Tick(); } }
  }
  rule KeepRunning { when ready(input: "x") and true; hold_for 1 @ ms; then { emit Tick(); } rationale "rule"; }
  constraint Bounded { class resource; assert 1 + 2 * 3 == 7; rationale "bound"; }
  safety Stop { when false; prohibit Controller.stop; then { invoke Controller.stop(); } rationale "safe"; }
  acceptance Starts {
    given enabled: bool = true;
    when { transition Lifecycle to Running; }
    expect true; within 5 @ ms;
  }
  decision D1 { status approved; statement "chosen"; rationale "because"; }
  open Q1 { question "why"; }
  unknown U1 { statement "unknown"; }
}
"#;

    #[test]
    fn parses_every_formal_spec_member_through_the_single_parser() {
        let program = parse(COMPLETE_SOURCE.as_bytes()).unwrap();
        let members = &program.unit.spec.members;
        assert_eq!(program.unit.imports.len(), 1);
        assert_eq!(members.len(), 21);
        assert!(matches!(members[0], SpecMember::Intent(_)));
        assert!(matches!(members[1], SpecMember::Dimension(_)));
        assert!(matches!(members[2], SpecMember::Unit(_)));
        assert!(matches!(members[3], SpecMember::Type(_)));
        assert!(matches!(members[4], SpecMember::Enum(_)));
        assert!(matches!(members[5], SpecMember::Record(_)));
        assert!(matches!(members[6], SpecMember::Domain(_)));
        assert!(matches!(members[7], SpecMember::Component(_)));
        assert!(matches!(members[8], SpecMember::Operation(_)));
        assert!(matches!(members[9], SpecMember::Transport(_)));
        assert!(matches!(members[10], SpecMember::Event(_)));
        assert!(matches!(members[11], SpecMember::Execution(_)));
        assert!(matches!(members[12], SpecMember::Channel(_)));
        assert!(matches!(members[13], SpecMember::StateMachine(_)));
        assert!(matches!(members[14], SpecMember::Rule(_)));
        assert!(matches!(members[15], SpecMember::Constraint(_)));
        assert!(matches!(members[16], SpecMember::Safety(_)));
        assert!(matches!(members[17], SpecMember::Acceptance(_)));
        assert!(matches!(members[18], SpecMember::Decision(_)));
        assert!(matches!(members[19], SpecMember::Open(_)));
        assert!(matches!(members[20], SpecMember::Unknown(_)));
    }

    #[test]
    fn builds_the_approved_expression_precedence_tree() {
        let program = parse(
            br#"apls "0.1"; spec P { version "1"; constraint C { class architecture; assert true or false and 1 + 2 * 3 == 7; } }"#,
        )
        .unwrap();
        let SpecMember::Constraint(constraint) = &program.unit.spec.members[0] else {
            panic!("expected constraint")
        };
        let ExpressionKind::Binary {
            operator: BinaryOperator::Or,
            right,
            ..
        } = &constraint.assertion.value
        else {
            panic!("expected or root")
        };
        let ExpressionKind::Binary {
            operator: BinaryOperator::And,
            right,
            ..
        } = &right.value
        else {
            panic!("expected and below or")
        };
        let ExpressionKind::Binary {
            operator: BinaryOperator::Equal,
            left,
            ..
        } = &right.value
        else {
            panic!("expected equality below and")
        };
        let ExpressionKind::Binary {
            operator: BinaryOperator::Add,
            right,
            ..
        } = &left.value
        else {
            panic!("expected addition below equality")
        };
        assert!(matches!(
            right.value,
            ExpressionKind::Binary {
                operator: BinaryOperator::Multiply,
                ..
            }
        ));
    }

    #[test]
    fn decodes_json_string_scalars_when_constructing_the_ast() {
        let program =
            parse(br#"apls "0.1"; spec S { version "1"; intent "line\n\uD83D\uDE80"; }"#).unwrap();
        let SpecMember::Intent(value) = &program.unit.spec.members[0] else {
            panic!("expected intent")
        };
        assert_eq!(value.value, "line\n🚀");
    }

    #[test]
    fn lexical_failure_cannot_construct_a_parsed_program() {
        let failure = parse(b"\xEF\xBB\xBFapls \"0.1\";").unwrap_err();
        assert!(matches!(
            failure,
            ParseFailure::Lex(finding) if finding.code == LexErrorCode::InvalidCharacter
        ));
    }

    #[test]
    fn syntax_failures_use_the_approved_e1101_and_e1103_categories() {
        let unexpected = parse(br#"apls "0.1"; spec S { version "1"; mystery }"#).unwrap_err();
        let missing = parse(br#"apls "0.1"; spec S { version "1";"#).unwrap_err();
        let repeated = parse(br#"apls "0.1"; spec S { version "1"; version "2"; }"#).unwrap_err();

        assert!(matches!(
            unexpected,
            ParseFailure::Syntax(finding)
                if finding.code == ParseErrorCode::UnexpectedToken
                    && finding.code.as_str() == "APLS-E1101"
        ));
        for failure in [missing, repeated] {
            assert!(matches!(
                failure,
                ParseFailure::Syntax(finding)
                    if finding.code == ParseErrorCode::MissingOrMisorderedStructure
                        && finding.code.as_str() == "APLS-E1103"
            ));
        }
    }

    #[test]
    fn reserved_set_keyword_is_not_an_apls_0_1_action() {
        let failure = parse(
            br#"apls "0.1"; spec S { version "1"; rule R { when true; then { set value = true; } } }"#,
        )
        .unwrap_err();
        assert!(matches!(failure, ParseFailure::Syntax(_)));
    }

    #[test]
    fn literals_remain_typed_in_the_surface_ast() {
        let program = parse(
            br#"apls "0.1"; spec S { version "1"; acceptance A { given ok: bool = true; when {} expect false; } }"#,
        )
        .unwrap();
        let SpecMember::Acceptance(acceptance) = &program.unit.spec.members[0] else {
            panic!("expected acceptance")
        };
        let crate::ast::ConstExprKind::Literal(literal) = &acceptance.given[0].value.value else {
            panic!("expected literal")
        };
        assert_eq!(literal.value, LiteralKind::Bool(true));
    }
}
