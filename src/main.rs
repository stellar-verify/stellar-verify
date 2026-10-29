use clap::Parser;
use stellar_verify::cli::args::{Cli, Commands};
use stellar_verify::infrastructure::fs::read_xdr_file;
use std::process;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::VerifyInclusion { tx_file, ledger_file } => {
            println!("Reading transaction evidence from: {}", tx_file.display());
            
            let tx_xdr = match read_xdr_file(tx_file) {
                Ok(xdr) => xdr,
                Err(e) => {
                    eprintln!("Error reading transaction file: {}", e);
                    process::exit(1);
                }
            };

            println!("Successfully loaded XDR ({} bytes). Verification logic pending audit completion.", tx_xdr.len());
        }
    }
}