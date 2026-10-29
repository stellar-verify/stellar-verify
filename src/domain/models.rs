use super::errors::{VerificationStatus, VerifyError};
use serde::Serialize;

/// The deterministic JSON output structure for the CLI.
#[derive(Serialize, Debug)]
pub struct VerificationResult {
    pub version: String,
    pub operation: String,
    pub status: VerificationStatus,
    pub checks_performed: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<String>,
}

impl VerificationResult {
    pub fn error(operation: &str, err: VerifyError) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            operation: operation.to_string(),
            status: match err {
                VerifyError::UnsupportedVersion(_) => VerificationStatus::Unsupported,
                VerifyError::MissingEvidence(_) => VerificationStatus::Incomplete,
                _ => VerificationStatus::Failed, // Maps standard parsing errors to failed
            },
            checks_performed: vec![],
            diagnostics: Some(err.to_string()),
        }
    }
}
