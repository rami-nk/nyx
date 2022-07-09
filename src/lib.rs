use clap::{Parser, Subcommand};
use core::panic;
use sha1::{Digest, Sha1};
use std::{io::{Write, Read}, path::{Path, PathBuf}};
use std::{fmt, fs, str};
use std::fs::OpenOptions;
use format_bytes::format_bytes;

mod errors;
use errors::NyxError;

// TODO: Encapsulate command matching logic and check if repo alredy setup
pub fn run(cli: NyxCli) -> Result<(), NyxError> {
    match &cli.command {
        Some(command) => match command {
            NyxCommand::Init => {
                println!("Initializing nyx repo...");
                init().unwrap();
            }
            NyxCommand::HashObject { path } => { hash_object(path)?; },
            NyxCommand::CatFile { hash } => cat_file(hash)?,
            NyxCommand::Add { file_path } => add(file_path)?,
            NyxCommand::LsFile => ls_file(),
            _ => (),
        },
        None => println!("Unknown command!"),
    };
    Ok(())
}

pub fn init() -> Result<(), NyxError> {
    let dir = Path::new(".nyx");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;

    Ok(())
}

pub fn hash_object(path: &str) -> Result<String, NyxError> {
    // TODO: Should be callable from all dirs within the repo
    if !Path::new(".nyx").join("objects").exists() {
        // TODO: logging concept
        panic!("Not in a nyx repository");
    }

    let mut content = fs::read(PathBuf::from(path))?;

    content = append_object_header(&content, NyxObjectType::Blob);
    let sha1 = calculate_sha1(&content);

    let object_dir = &sha1[..2];
    let object_file = &sha1[2..];

    let object_dir_path: PathBuf = [".nyx", "objects", &object_dir].iter().collect();

    if !object_dir_path.exists() {
        fs::create_dir(&object_dir_path)?;
    }

    let mut file = fs::File::create(object_dir_path.join(&object_file))?;

    file.write(&content)?;

    println!("{sha1}");

    Ok(sha1)
}

fn cat_file(hash: &str) -> Result<(), NyxError> {
    // TODO: In every directory callable
    let path: PathBuf = [".nyx", "objects", &hash[..2], &hash[2..]].iter().collect();
    let content = fs::read(path)?;

    // Remove header
    let index = &content.iter().position(|x| *x == 0).unwrap();
    let content = &content[*index..];

    let content = str::from_utf8(&content)?;

    println!("{}", content); 
    Ok(())
}

fn add(file_path: &str) -> Result<(), NyxError> {
    // TODO: Check if same file with same hash and same file with different hash
    let index_path = [".nyx", "index"].iter().collect::<PathBuf>();

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .read(true)
    .open(index_path)?;
    
    let sha1 = hash_object(&file_path)?;
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    if content.contains(&sha1) {
        return Ok(());
    }

    let content = format!("{} {}\n", &sha1, &file_path);
    file.write(content.as_bytes())?;

    Ok(())
}

fn ls_file() {
    let path = [".nyx", "index"].iter().collect::<PathBuf>();   
    let content = fs::read_to_string(path).unwrap();
    println!("{content}");
}

fn calculate_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

fn append_object_header(content: &[u8], object_type: NyxObjectType) -> Vec<u8> {
    let object_type_bytes = object_type.to_string().to_lowercase().as_bytes().to_vec();
    let content_len_bytes = content.len().to_string().as_bytes().to_vec();
    format_bytes!(b"{} {}\0{}", object_type_bytes, content_len_bytes, content)
}

#[derive(Debug)]
pub enum NyxObjectType {
    Commit,
    Tree,
    Blob,
}

impl fmt::Display for NyxObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// #############################################
// ################ CLAP ARGPARSE ##############
// #############################################

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct NyxCli {
    #[clap(subcommand)]
    pub command: Option<NyxCommand>,
}

#[derive(Subcommand)]
pub enum NyxCommand {
    /// Creates an empty nyx repository 
    Init,
    /// Adds one or many files to staging area
    Add {
        #[clap(value_parser)]
        file_path: String,
    },
    Commit,
    // ####### LOW-LEVEL COMMANDS #######
    /// Compute object ID and creates a blob object from a file 
    HashObject {
        #[clap(value_parser)]
        path: String,
    },
    /// Provide content for repository object
    CatFile {
        #[clap(value_parser)]
        hash: String,
    },
    LsFile,
}
