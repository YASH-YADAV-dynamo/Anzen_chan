use clap::{Parser, Subcommand};
use rpassword::read_password;
use solana_sdk::signer::keypair::Keypair;
use solana_keypair_manager::{read_keypair_from_file, write_keypair_to_file, Pubkey};

/// CLI for managing Solana keypairs securely
#[derive(Parser)]
#[command(name = "Keypair Manager")]
#[command(about = "A tool to manage Solana keypairs securely", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair and store it securely
    Generate {
        /// File path to store the encrypted keypair
        #[arg(short, long)]
        file: String,
    },
    /// Load and decrypt a keypair
    Load {
        /// File path of the encrypted keypair
        #[arg(short, long)]
        file: String,
    },
    /// Export the public key from a stored keypair
    PublicKey {
        /// File path of the encrypted keypair
        #[arg(short, long)]
        file: String,
    },
}

fn prompt_password() -> String {
    println!("Enter a password:");
    read_password().expect("Failed to read password")
}

fn print_banner() {
    println!(r#"
     _        _         _                 
    /_\  _ __| |_ _  _ | |__  ___ __ _ ___
   / _ \| '_ \  _| || || '_ \/ -_) _` (_-<
  /_/ \_\ .__/\__|\_, ||_.__/\___\__,_/__/
        |_|      |__/    ~ Anzen_chan ~   
    "#);
}

fn main() {
    print_banner();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { file } => {
            let keypair = Keypair::new();
            let password = prompt_password();

            write_keypair_to_file(&file, &keypair, &password);
            println!("Keypair generated and stored securely.");
        }
        Commands::Load { file } => {
            let password = prompt_password();

            let keypair = read_keypair_from_file(&file, &password);
            println!("Keypair loaded successfully. Public Key: {}", keypair.pubkey());
        }
        Commands::PublicKey { file } => {
            let password = prompt_password();

            let keypair = read_keypair_from_file(&file, &password);
            println!("Public Key: {}", keypair.pubkey());
        }
    }
}
