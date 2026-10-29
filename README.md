# Stellar Verify

A developer-focused toolkit for examining and verifying Stellar transaction-related evidence.

Stellar Verify aims to help developers understand which properties of Stellar transaction and ledger evidence can be checked locally, what evidence those checks require, and what conclusions the results support.

## Project status
**Early research and development.** The first supported verification workflow is being defined.

## Goals
- Make a specific Stellar verification workflow reproducible.
- Reuse existing Stellar protocol libraries and authoritative specifications.
- Explain verification results, assumptions, and limitations.
- Provide automated tests and clear contribution instructions.

## Verification boundaries
Retrieving a transaction record does not, by itself, establish cryptographic proof of inclusion or independently verify consensus finality. Stellar Verify will document exactly what each supported check establishes and will not claim stronger guarantees than its implementation provides.

## Contributing
Contributions are welcome once the initial scope is established. Please read [CONTRIBUTING.md](CONTRIBUTING.md), review open issues, and discuss substantial changes before starting implementation.

## Disclaimer
Stellar Verify is an independent open-source project and is not an official Stellar Development Foundation product unless explicitly stated otherwise.