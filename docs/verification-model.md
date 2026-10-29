# Verification Model and Trust Boundaries

This document defines the cryptographic boundaries and trust assumptions for the Stellar Verify toolkit. The primary principle of this project is that **verification claims must not exceed the evidence.**

## Terminology

To avoid conflating network responses with cryptographic proof, this project enforces the following distinctions:

*   **Retrieval:** A service (like Horizon or an RPC endpoint) returned a record or file. This is network I/O, not verification.
*   **Parsing:** The supplied input conforms to the expected XDR encoding and schema sufficiently to be decoded into structured types.
*   **Integrity check:** A specified recomputation or consistency rule matches (e.g., a recomputed SHA-256 hash matches a provided digest).
*   **Commitment verification:** A supplied value matches a documented cryptographic commitment to the exact data and serialization used.
*   **Inclusion evidence:** Evidence establishes that an item belongs to a committed collection under the relevant data structure and verification rules (e.g., a Merkle proof).
*   **Consensus/finality assurance:** Establishing network agreement on a ledger. **Stellar Verify does not independently verify network consensus.** 

## MVP Workflow: [To Be Finalized Post-Audit]

*Note: The exact workflow will be finalized after the completion of the existing-tool audit (Issue #1). A candidate workflow is validating a compact transaction inclusion proof.*

If the inclusion proof workflow is selected, the tool will:
1. Parse the base64-encoded XDR transaction envelope.
2. Recompute the transaction hash commitment.
3. Verify the Merkle path connecting the transaction hash to the provided Transaction Set Hash.

## Trust Assumptions

Stellar Verify operates entirely offline. It does not make independent network calls to verify current network state. Therefore, any result is strictly conditional on the user's trust in the provided anchors.

1.  **Trust in the Anchor:** If verifying an inclusion proof, the user must independently trust the `Transaction Set Hash` or `Ledger Header` provided as the root of trust.
2.  **No Finality Claims:** A successful hash-linking check means the math is consistent; it does not mean the broader Stellar network achieved consensus on that ledger.