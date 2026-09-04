use crate::cnl_ast::ByteSpan;
use crate::diagnostic::Diagnostic;
use serde_json::json;
use std::collections::BTreeMap;

pub const MAX_LATTICE_EDGES: usize = 1_000_000;
pub const MAX_STREAMS_PER_SENTENCE: usize = 4_096;
pub const MAX_TOKEN_OCCURRENCES: usize = 1_000_000;
pub const MAX_SYNTAX_NODES: usize = 1_000_000;
pub const MAX_FRAME_CANDIDATES: usize = 1_000_000;

#[derive(Clone, Copy)]
pub enum Resource {
    LatticeEdges,
    Streams,
    TokenOccurrences,
    SyntaxNodes,
    BoundFrames,
    TypedFrames,
    CanonicalFrames,
}

impl Resource {
    const fn name(self) -> &'static str {
        match self {
            Self::LatticeEdges => "candidate_lattice_edges",
            Self::Streams => "complete_token_streams",
            Self::TokenOccurrences => "candidate_token_occurrences",
            Self::SyntaxNodes => "parse_candidate_syntax_nodes",
            Self::BoundFrames => "bound_frame_candidates",
            Self::TypedFrames => "typed_frame_candidates",
            Self::CanonicalFrames => "canonical_frame_candidates",
        }
    }
    const fn stage(self) -> &'static str {
        match self {
            Self::LatticeEdges => "lattice",
            Self::Streams | Self::TokenOccurrences => "enumerate",
            Self::SyntaxNodes => "parse",
            Self::BoundFrames => "bind",
            Self::TypedFrames => "type",
            Self::CanonicalFrames => "normalize",
        }
    }
    const fn limit(self) -> usize {
        match self {
            Self::Streams => MAX_STREAMS_PER_SENTENCE,
            Self::LatticeEdges => MAX_LATTICE_EDGES,
            Self::TokenOccurrences => MAX_TOKEN_OCCURRENCES,
            Self::SyntaxNodes => MAX_SYNTAX_NODES,
            Self::BoundFrames | Self::TypedFrames | Self::CanonicalFrames => MAX_FRAME_CANDIDATES,
        }
    }
}

#[derive(Default)]
pub struct Ledger {
    streams_per_sentence: BTreeMap<usize, usize>,
    lattice_edges: usize,
    token_occurrences: usize,
    syntax_nodes: usize,
    bound_frames: usize,
    typed_frames: usize,
    canonical_frames: usize,
}

impl Ledger {
    pub fn add(
        &mut self,
        resource: Resource,
        count: usize,
        sentence_index: Option<usize>,
        span: Option<ByteSpan>,
        path: &str,
        source: &str,
    ) -> Result<(), Diagnostic> {
        let value = match resource {
            Resource::LatticeEdges => &mut self.lattice_edges,
            Resource::TokenOccurrences => &mut self.token_occurrences,
            Resource::SyntaxNodes => &mut self.syntax_nodes,
            Resource::BoundFrames => &mut self.bound_frames,
            Resource::TypedFrames => &mut self.typed_frames,
            Resource::CanonicalFrames => &mut self.canonical_frames,
            Resource::Streams => unreachable!(),
        };
        if value.saturating_add(count) > resource.limit() {
            return Err(resource_diagnostic(
                resource,
                sentence_index,
                span,
                path,
                source,
            ));
        }
        *value += count;
        Ok(())
    }
    pub fn add_stream(
        &mut self,
        sentence_index: usize,
        span: ByteSpan,
        path: &str,
        source: &str,
    ) -> Result<(), Diagnostic> {
        let value = self.streams_per_sentence.entry(sentence_index).or_default();
        if *value == MAX_STREAMS_PER_SENTENCE {
            return Err(resource_diagnostic(
                Resource::Streams,
                Some(sentence_index),
                Some(span),
                path,
                source,
            ));
        }
        *value += 1;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn set_for_test(&mut self, resource: Resource, value: usize) {
        match resource {
            Resource::LatticeEdges => self.lattice_edges = value,
            Resource::TokenOccurrences => self.token_occurrences = value,
            Resource::SyntaxNodes => self.syntax_nodes = value,
            Resource::BoundFrames => self.bound_frames = value,
            Resource::TypedFrames => self.typed_frames = value,
            Resource::CanonicalFrames => self.canonical_frames = value,
            Resource::Streams => panic!("use sentence-specific stream setup"),
        }
    }

    #[cfg(test)]
    pub(crate) fn value_for_test(&self, resource: Resource) -> usize {
        match resource {
            Resource::LatticeEdges => self.lattice_edges,
            Resource::TokenOccurrences => self.token_occurrences,
            Resource::SyntaxNodes => self.syntax_nodes,
            Resource::BoundFrames => self.bound_frames,
            Resource::TypedFrames => self.typed_frames,
            Resource::CanonicalFrames => self.canonical_frames,
            Resource::Streams => panic!("use sentence-specific stream inspection"),
        }
    }
}

pub fn source_bytes_limit(observed: usize) -> Diagnostic {
    standalone_resource_diagnostic(
        "single_source_bytes",
        crate::limits::MAX_SOURCE_BYTES,
        observed,
        "source",
        "load",
    )
}

pub fn canonical_ir_limit(observed: usize) -> Diagnostic {
    standalone_resource_diagnostic(
        "canonical_ir_bytes",
        crate::limits::MAX_CANONICAL_IR_BYTES,
        observed,
        "artifact",
        "emit",
    )
}

pub fn expression_depth_limit(
    observed: usize,
    sentence_index: usize,
    span: ByteSpan,
    path: &str,
    source: &str,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::source(
        "APLS-T0007",
        "compiler resource limit exceeded",
        path,
        source,
        span,
    );
    diagnostic.normative_rule_reference = "DEC-011 / DES-APLS-CNL-RESOURCE-001".into();
    diagnostic.payload = Some(json!({
        "kind": "resource_limit",
        "limit": crate::limits::MAX_EXPRESSION_DEPTH,
        "observed": observed,
        "resource": "expression_depth",
        "scope": "sentence",
        "sentence_index": sentence_index,
        "stage": "parse"
    }));
    diagnostic
}

pub fn public_diagnostics_limit() -> Diagnostic {
    standalone_resource_diagnostic(
        "public_diagnostics",
        crate::limits::MAX_DIAGNOSTICS,
        crate::limits::MAX_DIAGNOSTICS + 1,
        "document",
        "diagnose",
    )
}

fn standalone_resource_diagnostic(
    resource: &str,
    limit: usize,
    observed: usize,
    scope: &str,
    stage: &str,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::tool("APLS-T0007", "compiler resource limit exceeded");
    diagnostic.normative_rule_reference = "DES-APLS-CNL-RESOURCE-001".into();
    diagnostic.payload = Some(json!({
        "kind": "resource_limit", "limit": limit, "observed": observed,
        "resource": resource, "scope": scope, "sentence_index": null, "stage": stage
    }));
    diagnostic
}

fn resource_diagnostic(
    resource: Resource,
    sentence_index: Option<usize>,
    span: Option<ByteSpan>,
    path: &str,
    source: &str,
) -> Diagnostic {
    let mut diagnostic = span.map_or_else(
        || Diagnostic::tool("APLS-T0007", "compiler resource limit exceeded"),
        |s| {
            Diagnostic::source(
                "APLS-T0007",
                "compiler resource limit exceeded",
                path,
                source,
                s,
            )
        },
    );
    diagnostic.normative_rule_reference = "DES-APLS-CNL-RESOURCE-001".into();
    diagnostic.payload = Some(
        json!({"kind":"resource_limit","limit":resource.limit(),"observed":resource.limit()+1,
        "resource":resource.name(),"scope":if matches!(resource, Resource::Streams){"sentence"}else{"document"},
        "sentence_index":sentence_index,"stage":resource.stage()}),
    );
    diagnostic
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defensive_counters_fail_on_first_item_past_limit() {
        for resource in [
            Resource::SyntaxNodes,
            Resource::BoundFrames,
            Resource::TypedFrames,
            Resource::CanonicalFrames,
        ] {
            let mut ledger = Ledger::default();
            ledger.set_for_test(resource, resource.limit() - 1);
            assert!(
                ledger
                    .add(
                        resource,
                        1,
                        Some(1),
                        Some(ByteSpan::new(0, 3)),
                        "x.apls",
                        "。",
                    )
                    .is_ok()
            );
            let error = ledger
                .add(
                    resource,
                    1,
                    Some(1),
                    Some(ByteSpan::new(0, 3)),
                    "x.apls",
                    "。",
                )
                .unwrap_err();
            assert_eq!(error.code, "APLS-T0007");
            let payload = error.payload.unwrap();
            assert_eq!(payload["resource"], resource.name());
            assert_eq!(payload["limit"], resource.limit());
            assert_eq!(payload["observed"], resource.limit() + 1);
            assert_eq!(payload["stage"], resource.stage());
        }
    }
}
