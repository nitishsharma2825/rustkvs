use clap::{Parser, Subcommand};
use kvs::{KvStore, MyError, Result};
use std::{env::current_dir, process::exit};

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
    let mut store = KvStore::open(current_dir()?.to_path_buf()).unwrap();
    match &args.command {
        Some(Commands::Get { key }) => {
            match store.get(key.clone()) {
                Ok(Some(value)) => {
                    println!("{value}");
                    Ok(())
                },
                Ok(None) => {
                    println!("Key not found");
                    Ok(())
                },
                Err(e) => {
                    println!("Key not found");
                    Ok(())
                }
            }
        }
        Some(Commands::Set { key, value }) => {
            let value = store.set(key.clone(), value.clone()).unwrap();
            Ok(())
        }
        Some(Commands::Rm { key }) => {
            match store.remove(key.clone()) {
                Ok(()) => {
                    Ok(())
                },
                Err(e) => {
                    println!("Key not found");
                    exit(1)
                }
            }
        }
        None => panic!(),
    }
}