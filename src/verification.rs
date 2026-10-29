use sha2::{Digest, Sha256};
use stellar_xdr::{ReadXdr, TransactionEnvelope, Limits};

#[derive(Debug, PartialEq)]
pub enum VerificationError {
    MalformedXdr(String),
    InvalidHashCommitment,
    IncompleteData,
}

#[derive(Debug, PartialEq)]
pub enum VerificationStatus {
    Verified,
    Failed(VerificationError),
}

/// Parses XDR and verifies the hash commitment for transaction inclusion.
pub fn verify_inclusion_proof(xdr_payload: &[u8], expected_hash: &[u8]) -> VerificationStatus {
    if xdr_payload.is_empty() {
        return VerificationStatus::Failed(VerificationError::IncompleteData);
    }
    
    // Limits::none() allows unlimited depth/length to prevent false-positive rejections.
    if let Err(e) = TransactionEnvelope::from_xdr(xdr_payload, Limits::none()) {
        return VerificationStatus::Failed(VerificationError::MalformedXdr(e.to_string()));
    }
    
    let mut hasher = Sha256::new();
    hasher.update(xdr_payload);
    let computed_hash = hasher.finalize();
    
    if computed_hash.as_slice() != expected_hash {
        return VerificationStatus::Failed(VerificationError::InvalidHashCommitment);
    }
    
    VerificationStatus::Verified
}
