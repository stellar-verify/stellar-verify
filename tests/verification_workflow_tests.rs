use std::fs;
use sha2::{Digest, Sha256};

#[path = "../src/verification.rs"]
mod verification;
use verification::{verify_inclusion_proof, VerificationStatus, VerificationError};

fn compute_hash(payload: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    hasher.finalize().to_vec()
}

#[test]
fn test_valid_inclusion_proof() {
    let xdr = fs::read("fixtures/proofs/valid_proof.xdr").expect("Failed to read valid_proof.xdr");
    let expected_hash = compute_hash(&xdr);
    
    let result = verify_inclusion_proof(&xdr, &expected_hash);
    assert_eq!(result, VerificationStatus::Verified);
}

#[test]
fn test_invalid_hash_commitment() {
    let xdr = fs::read("fixtures/proofs/valid_proof.xdr").expect("Failed to read valid_proof.xdr");
    let bad_hash = vec![0xFF; 32];
    
    let result = verify_inclusion_proof(&xdr, &bad_hash);
    assert_eq!(result, VerificationStatus::Failed(VerificationError::InvalidHashCommitment));
}

#[test]
fn test_malformed_xdr() {
    let xdr = vec![0x00, 0xFF, 0x0A]; // Intentionally malformed
    let expected_hash = compute_hash(&xdr);
    
    let result = verify_inclusion_proof(&xdr, &expected_hash);
    assert!(matches!(result, VerificationStatus::Failed(VerificationError::MalformedXdr(_))));
}

#[test]
fn test_incomplete_data() {
    let result = verify_inclusion_proof(&[], &[]);
    assert_eq!(result, VerificationStatus::Failed(VerificationError::IncompleteData));
}
