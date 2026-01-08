mod verifier;
mod diagnostics;
mod architect;

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
    /// Visualize the recursive hierarchy of an Aztec proof tree
    Tree,
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
            let failures = diagnostics::inspect_proof_constraints(proof);
            diagnostics::report_diagnostics(failures);
        }
        Commands::Tree => {
            println!("{}", "Generating Recursive Proof Visualization...".magenta());
            let tree = architect::generate_mock_tree();
            architect::visualize_tree(&tree, 0);
        }
    }
}
