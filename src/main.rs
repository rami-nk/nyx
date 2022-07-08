use std::io::{Read, Write};
use std::path::Path;
use clap::{Parser, Subcommand};
use core::panic;
use std::{io, fs, str, fmt};
use sha1::{Sha1, Digest};

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

fn hash_object(path: &str) -> Result<(), NyxError> {
    let objects_path = Path::new(".nyx").join("objects");
    
    if !objects_path.exists() {
        panic!("You are not in a nyx repo!");
    }

    let mut buffer = Vec::new();
    let mut file = fs::File::open(path)?;
    file.read_to_end(&mut buffer)?;
    let content_str = std::str::from_utf8(&buffer)?.trim();

    // Todo: Currently only blob types are supported
    let sha1 = calculate_sha1(content_str, NyxObjectType::Blob);
    
    let object_dir = &sha1[..2];
    let object_file = &sha1[2..];
    
    let object_dir_path = Path::new(".nyx")
                                        .join("objects")
                                        .join(&object_dir);
    
    if !object_dir_path.exists() { fs::create_dir(&object_dir_path)?; }

    let mut file = fs::File::create(object_dir_path.join(&object_file))?;
    file.write(&buffer)?;
    
    println!("{sha1}");

    Ok(())
}

fn calculate_sha1(content: &str, object_type: NyxObjectType) -> String {
    let mut hasher = Sha1::new();
    
    let content = format!("{} {}\0{}",
         object_type.to_string().to_lowercase(),
         &content.as_bytes().len().to_string(),
         content);

    hasher.update(content);
    hex::encode(hasher.finalize())
}

#[derive(Debug)]
enum NyxError {
    IoError(io::Error),
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

#[derive(Debug)]
enum NyxObjectType {
    Commit,
    Tree,
    Blob,
}

impl fmt::Display for NyxObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}