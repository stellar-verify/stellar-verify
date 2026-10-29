# Stellar Verify (Offline Transaction Inclusion)

`stellar-verify` is a specialized, lightweight CLI tool designed to securely verify Stellar transaction inclusion proofs in completely offline or zero-trust environments. 

By taking a raw XDR transaction payload and a trusted ledger hash, `stellar-verify` deterministically parses the transaction envelope, computes the SHA-256 hash commitment, and proves inclusion without requiring an active Horizon or RPC node connection.

## 🚀 Why this exists
While tools like `stellar-archivist` and `rs-stellar-xdr` excel at retrieving archives and parsing structures, there is a recognized tooling gap (see Stellar Docs #2611) for **pure offline verification**. This tool bridges that gap by isolating the cryptographic hash-matching logic into a standalone, CI-friendly binary that outputs deterministic JSON.

## 🛠️ Prerequisites
- [Rust Toolchain](https://rustup.rs/) (1.75.0 or higher)

## 📦 Installation & Build

Clone the repository and build the release binary:

```bash
git clone [https://github.com/YOUR_ORG/stellar-verify.git](https://github.com/YOUR_ORG/stellar-verify.git)
cd stellar-verify
cargo build --release

💻 Usage

The CLI requires two inputs:

    --tx-file: The raw binary XDR file of the transaction envelope.

    --ledger-file: A text file containing the expected 32-byte hex-encoded SHA-256 hash of the transaction.

Example: Verifying a Valid Proof

We provide reproducible fixtures in the repository for testing.
Bash

cargo run -- verify-inclusion \
  --tx-file fixtures/proofs/valid_proof.xdr \
  --ledger-file fixtures/proofs/valid_ledger_hash.txt

Output (Success):
JSON

{"message":"Transaction inclusion proven successfully.","status":"verified"}

Example: Handling Malformed Data
Bash

cargo run -- verify-inclusion \
  --tx-file fixtures/proofs/malformed_proof.xdr \
  --ledger-file fixtures/proofs/valid_ledger_hash.txt

Output (Failure):
JSON

{"reason":"MalformedXdr(\"xdr value max length exceeded\")","status":"failed"}

🔒 Verification Model & Trust Boundaries

    XDR Parsing: The payload is strictly parsed using the official stellar-xdr v30 crate to prevent malleability attacks.

    Hash Commitment: A SHA-256 digest is computed directly from the validated XDR bytes.

    Comparison: The computed digest is compared against the trusted ledger hash provided by the user.

🤝 Contributing

Please read CONTRIBUTING.md for details on our pull request process, branch naming conventions, and coding standards.
📄 License

This project is licensed under the MIT License.
