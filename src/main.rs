mod verifier;

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "aztec-prooflens")]
#[command(about = "Diagnostic Layer for the Aztec Network", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify the validity of a zero-knowledge proof
    Verify {
        /// Path to the proof file
        #[arg(short, long)]
        proof: String,

        /// Path to the verification key file
        #[arg(short, long)]
        vk: String,
    },
    /// Inspect a proof and map constraints to Noir source code
    Inspect {
        /// Path to the proof file
        #[arg(short, long)]
        proof: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Verify { proof, vk } => {
            println!("{}", "Initiating Proof Verification...".cyan());
            if verifier::verify_proof_logic(proof, vk) {
                println!("{}", "Verification Logic Initialized Successfully.".green());
            } else {
                println!("{}", "Verification Logic Failed to Initialize.".red());
            }
        }
        Commands::Inspect { proof } => {
            println!("{}", "Initiating Proof Inspection...".yellow());
            println!("Proof Path: {}", proof);
            // Diagnostic logic will be implemented in the next pillar
        }
    }
}
