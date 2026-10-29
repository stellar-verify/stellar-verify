use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Stellar Verify: A toolkit for examining Stellar transaction evidence", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validates a transaction inclusion proof (Mock MVP workflow)
    VerifyInclusion {
        /// Path to the base64-encoded XDR transaction envelope
        #[arg(short, long, value_name = "FILE")]
        tx_file: PathBuf,

        /// Path to the trusted ledger header XDR
        #[arg(short, long, value_name = "FILE")]
        ledger_file: PathBuf,
    },
}
