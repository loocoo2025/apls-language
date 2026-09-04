#[derive(Debug)]
pub struct VerifiedArtifact {
    bytes: Vec<u8>,
}

impl VerifiedArtifact {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
