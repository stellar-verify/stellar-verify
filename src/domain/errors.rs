
use serde::Serialize;
use thiserror::Error;

/// Represents the strict outcome states mandated by the PRD.
#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    /// The specified check completed and its stated condition held.
    Passed,
    /// The specified check completed and its condition did not hold.
    Failed,
    /// Required evidence was missing or insufficient.
    Incomplete,
    /// The input or operation is outside the implemented support boundary.
    Unsupported,
}

/// Represents operational or parsing errors that prevent a valid result.
#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Malformed input: {0}")]
    MalformedInput(String),
    
    #[error("Missing evidence: {0}")]
    MissingEvidence(String),
    
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u32),
    
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}