#![forbid(unsafe_code)]

mod cnl_ast;
mod cnl_lexer;
mod cnl_pipeline;
pub mod diagnostic;
pub mod limits;
pub mod pipeline;
mod resource;
mod validate;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all)]
    apls_grammar
);

pub use cnl_pipeline::compile;
pub use diagnostic::{
    Diagnostic, DiagnosticSeverity, OrderedDiagnostics, SourceSpan, envelope_bytes,
};
pub use pipeline::{CompileOutcome, CompileSuccess, CompileThrough};
pub use validate::VerifiedArtifact;
