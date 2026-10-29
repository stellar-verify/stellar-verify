use clap::Parser;
use serde_json::json;
use std::fs;
use std::process;
use stellar_verify::cli::args::{Cli, Commands};
use stellar_verify::verification::{verify_inclusion_proof, VerificationStatus};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::VerifyInclusion {
            tx_file,
            ledger_file,
        } => {
            // 1. Read raw binary XDR payload
            let tx_bytes = match fs::read(tx_file) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!(
                        "{}",
                        json!({ "status": "error", "message": format!("Failed to read tx_file: {}", e) })
                    );
                    process::exit(1);
                }
            };

            // 2. Read and decode hex ledger hash
            let ledger_hex = match fs::read_to_string(ledger_file) {
                Ok(s) => s.trim().to_string(),
                Err(e) => {
                    eprintln!(
                        "{}",
                        json!({ "status": "error", "message": format!("Failed to read ledger_file: {}", e) })
                    );
                    process::exit(1);
                }
            };

            let expected_hash = match hex::decode(&ledger_hex) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!(
                        "{}",
                        json!({ "status": "error", "message": format!("Invalid hex in ledger_file: {}", e) })
                    );
                    process::exit(1);
                }
            };

            // 3. Execute domain logic
            let result = verify_inclusion_proof(&tx_bytes, &expected_hash);

            // 4. Print deterministic JSON output
            match result {
                VerificationStatus::Verified => {
                    println!(
                        "{}",
                        json!({
                            "status": "verified",
                            "message": "Transaction inclusion proven successfully."
                        })
                    );
                    process::exit(0);
                }
                VerificationStatus::Failed(err) => {
                    eprintln!(
                        "{}",
                        json!({
                            "status": "failed",
                            "reason": format!("{:?}", err)
                        })
                    );
                    process::exit(1);
                }
            }
        }
    }
}
