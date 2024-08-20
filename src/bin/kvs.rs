use clap::{Parser, Subcommand};
use kvs::KvStore;
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

fn main() {
    let args = Args::parse();
    let mut store = KvStore::new();
    match &args.command {
        Some(Commands::Get { key }) => {
            // let value = match store.get(key.clone()) {
            //     Some(val) => val,
            //     None => String::new()
            // };
            // println!("Value for key {key} is {value}");
            eprintln!("unimplemented");
            exit(1);
        }
        Some(Commands::Set { key, value }) => {
            // store.set(key.to_owned(), value.to_owned())
            eprintln!("unimplemented");
            exit(1);
        }
        Some(Commands::Rm { key }) => {
            // store.remove(key.to_owned())
            eprintln!("unimplemented");
            exit(1);
        }
        None => panic!(),
    }
}
