use serde::Serialize;
use serde_json::Value;

use crate::cnl_ast::ByteSpan;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Warning,
    Error,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourceSpan {
    pub logical_path: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl SourceSpan {
    pub fn from_bytes(logical_path: &str, source: &str, span: ByteSpan) -> Self {
        fn position(source: &str, byte: usize) -> (usize, usize) {
            let prefix = &source[..byte.min(source.len())];
            let line = prefix.bytes().filter(|value| *value == b'\n').count() + 1;
            let column = prefix
                .rsplit_once('\n')
                .map_or(prefix, |(_, tail)| tail)
                .chars()
                .count()
                + 1;
            (line, column)
        }
        let (start_line, start_column) = position(source, span.start_byte);
        let (end_line, end_column) = position(source, span.end_byte);
        Self {
            logical_path: logical_path.into(),
            start_byte: span.start_byte,
            end_byte: span.end_byte,
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CandidateSymbol {
    pub display_name: String,
    pub symbol_id: String,
    pub symbol_kind: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FixSuggestion {
    pub kind: String,
    pub message: String,
    pub target_span: Option<SourceSpan>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Diagnostic {
    pub candidate_symbols: Vec<CandidateSymbol>,
    pub code: String,
    pub fix_suggestions: Vec<FixSuggestion>,
    pub message: String,
    pub missing_or_ambiguous_roles: Vec<String>,
    pub normative_rule_reference: String,
    pub payload: Option<Value>,
    pub primary_source_span: Option<SourceSpan>,
    pub related_source_spans: Vec<SourceSpan>,
    pub severity: DiagnosticSeverity,
}

impl Diagnostic {
    pub fn source(code: &str, message: &str, path: &str, source: &str, span: ByteSpan) -> Self {
        Self::new(
            code,
            message,
            Some(SourceSpan::from_bytes(path, source, span)),
        )
    }
    pub fn tool(code: &str, message: &str) -> Self {
        Self::new(code, message, None)
    }
    fn new(code: &str, message: &str, primary_source_span: Option<SourceSpan>) -> Self {
        Self {
            candidate_symbols: Vec::new(),
            code: code.into(),
            fix_suggestions: Vec::new(),
            message: message.into(),
            missing_or_ambiguous_roles: Vec::new(),
            normative_rule_reference: "APLS 0.1 CNL diagnostics".into(),
            payload: None,
            primary_source_span,
            related_source_spans: Vec::new(),
            severity: DiagnosticSeverity::Error,
        }
    }
}

pub type OrderedDiagnostics = Vec<Diagnostic>;

pub fn envelope_bytes(status: &str, diagnostics: &OrderedDiagnostics) -> Result<Vec<u8>, ()> {
    const SCHEMA: &str =
        include_str!("../../../../04_design/diagnostics/apls-cnl-diagnostic-0.1.schema.json");
    let value = serde_json::json!({"diagnostics": diagnostics, "format": "apls-diagnostics", "format_version": "0.1", "status": status});
    let schema: Value = serde_json::from_str(SCHEMA).map_err(|_| ())?;
    let validator = jsonschema::validator_for(&schema).map_err(|_| ())?;
    if !validator.is_valid(&value) {
        return Err(());
    }
    serde_json::to_vec(&value).map_err(|_| ())
}

pub fn normalize_diagnostics(diagnostics: &mut OrderedDiagnostics) {
    for diagnostic in diagnostics.iter_mut() {
        diagnostic.candidate_symbols.sort();
        diagnostic.candidate_symbols.dedup();
        diagnostic.fix_suggestions.sort();
        diagnostic.fix_suggestions.dedup();
        diagnostic
            .missing_or_ambiguous_roles
            .sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        diagnostic.missing_or_ambiguous_roles.dedup();
        diagnostic.related_source_spans.sort();
        diagnostic.related_source_spans.dedup();
    }
    diagnostics.sort_by(|left, right| {
        let lk = left
            .primary_source_span
            .as_ref()
            .map_or((1, "", 0, 0), |s| {
                (0, s.logical_path.as_str(), s.start_byte, s.end_byte)
            });
        let rk = right
            .primary_source_span
            .as_ref()
            .map_or((1, "", 0, 0), |s| {
                (0, s.logical_path.as_str(), s.start_byte, s.end_byte)
            });
        lk.cmp(&rk)
            .then_with(|| left.code.as_bytes().cmp(right.code.as_bytes()))
            .then_with(|| {
                serde_json::to_vec(&left.related_source_spans)
                    .unwrap_or_default()
                    .cmp(&serde_json::to_vec(&right.related_source_spans).unwrap_or_default())
            })
            .then_with(|| {
                serde_json::to_vec(left)
                    .unwrap_or_default()
                    .cmp(&serde_json::to_vec(right).unwrap_or_default())
            })
    });
    diagnostics.dedup_by(|a, b| serde_json::to_vec(a).ok() == serde_json::to_vec(b).ok());
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn total_order_uses_end_code_full_value_and_places_tools_last() {
        let source = "abcd";
        let mut short = Diagnostic::source(
            "APLS-E1301",
            "short-span",
            "main.apls",
            source,
            ByteSpan::new(0, 2),
        );
        short.payload = Some(json!({"value": 2}));
        let mut long = Diagnostic::source(
            "APLS-E1201",
            "long-span",
            "main.apls",
            source,
            ByteSpan::new(0, 3),
        );
        long.payload = Some(json!({"value": 0}));
        let mut same_low = Diagnostic::source(
            "APLS-E1301",
            "same-span",
            "main.apls",
            source,
            ByteSpan::new(0, 2),
        );
        same_low.payload = Some(json!({"value": 1}));
        let tool = Diagnostic::tool("APLS-T0002", "tool-last");
        let mut diagnostics = vec![tool, long, short, same_low];

        normalize_diagnostics(&mut diagnostics);

        assert_eq!(diagnostics[0].payload.as_ref().unwrap()["value"], 1);
        assert_eq!(diagnostics[1].payload.as_ref().unwrap()["value"], 2);
        assert_eq!(diagnostics[2].message, "long-span");
        assert_eq!(diagnostics[3].message, "tool-last");
    }
}
