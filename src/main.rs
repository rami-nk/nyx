use clap::{Parser, Subcommand};
use std::path::Path;
use std::io;
use std::fs;

fn main() {
    let cli = NyxCli::parse();
    
    match &cli.command {
        Some(command) => {
            match command {
                NyxCommand::Init => {
                    println!("Initializing nyx repo...");
                    init().unwrap();
                },
                NyxCommand::HashObject => {
                    println!("Not implemented yet!");
                },
                _ => ()
            }
        },
        None => println!("Unknown command!"),
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

impl From<io::Error> for NyxError {
   fn from(err: io::Error) -> Self {
      NyxError::IOError(err) 
   } 
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct NyxCli {
    #[clap(subcommand)]
    command: Option<NyxCommand>,
}

#[derive(Subcommand)]
enum NyxCommand {
    Init,
    Add,
    Commit,
    // ####### LOW-LEVEL COMMANDS #######
    HashObject,
}