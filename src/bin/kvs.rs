use clap::{Parser, Subcommand};
use kvs::Result;
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// gets the value for the key
    Get {
        /// key for which value is needed
        key: String,
    },
    /// sets the value for the given key
    Set {
        /// key
        key: String,

        /// value
        value: String,
    },
    /// removes the key from the store
    Rm {
        /// key
        key: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Get { key }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(Commands::Set { key, value }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(Commands::Rm { key }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        None => panic!(),
    }
}