use crate::{OrderedDiagnostics, VerifiedArtifact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompileThrough {
    Parse,
    Check,
    Emit,
}

#[derive(Debug)]
pub enum CompileSuccess {
    Parsed,
    Checked,
    Verified(VerifiedArtifact),
}

#[derive(Debug)]
pub enum CompileOutcome {
    Accepted(CompileSuccess),
    Rejected(OrderedDiagnostics),
    ToolFailure(OrderedDiagnostics),
    InternalFailure(OrderedDiagnostics),
}

impl CompileOutcome {
    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        match self {
            Self::Accepted(_) => 0,
            Self::Rejected(_) => 1,
            Self::ToolFailure(_) => 2,
            Self::InternalFailure(_) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CompileOutcome;
    use crate::VerifiedArtifact;

    #[test]
    fn outcomes_map_to_the_approved_exit_codes() {
        assert_eq!(
            CompileOutcome::Accepted(super::CompileSuccess::Verified(VerifiedArtifact::new(
                Vec::new(),
            )))
            .exit_code(),
            0
        );
        assert_eq!(CompileOutcome::Rejected(Vec::new()).exit_code(), 1);
        assert_eq!(CompileOutcome::ToolFailure(Vec::new()).exit_code(), 2);
        assert_eq!(CompileOutcome::InternalFailure(Vec::new()).exit_code(), 3);
    }
}
