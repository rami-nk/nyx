use clap::{Parser, Subcommand};
use std::io::Read;
use std::path::Path;
use std::{io, fs, str};

fn main() {
    let cli = NyxCli::parse();
    
    match &cli.command {
        Some(command) => {
            match command {
                NyxCommand::Init => {
                    println!("Initializing nyx repo...");
                    init().unwrap();
                },
                NyxCommand::HashObject { path } => {
                    hash_object(path).unwrap();
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

fn hash_object(path: &String) -> Result<(), NyxError> {
    let mut buffer = Vec::new();
    let mut file = fs::File::open(path)?;
    file.read_to_end(&mut buffer)?;
    let content_str = std::str::from_utf8(&buffer)?.trim();
    println!("You entered the following input: {content_str:?}");
    Ok(())
}

#[derive(Debug)]
enum NyxError {
    IoError(std::io::Error),
    Utf8Error(str::Utf8Error),
}

impl From<io::Error> for NyxError {
   fn from(err: io::Error) -> Self {
      NyxError::IoError(err) 
   } 
}

impl From<str::Utf8Error> for NyxError {
   fn from(err: str::Utf8Error) -> Self {
      NyxError::Utf8Error(err) 
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
    HashObject {
        #[clap(value_parser)]
        path: String,
    },
}