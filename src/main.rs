use std::path::Path;
use std::fs;

fn main() {
    let cli = NyxCli::parse();
    
    match &cli.command {
        Some(NyxCommands::Init) => {
            println!("Initializing nyx repo...");
            init().unwrap();
        },
        _ => println!("Unknown command!"),
    }
}

fn init() -> Result<(), NyxError> {
    let dir = Path::new(".nyx");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;

    Ok(())
}

#[derive(Debug)]
enum NyxError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for NyxError {
   fn from(err: std::io::Error) -> Self {
      NyxError::IOError(err) 
   } 
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct NyxCli {
    #[clap(subcommand)]
    command: Option<NyxCommands>,
}

#[derive(Subcommand)]
enum NyxCommands {
    Init,
    Add,
    Commit,
}
