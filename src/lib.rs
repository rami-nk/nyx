use clap::{Parser, Subcommand};
use core::panic;
use sha1::{Digest, Sha1};
use std::{io::Write, path::{Path, PathBuf}};
use std::{fmt, fs, str};
use flate2::Compression;
use flate2::write::ZlibEncoder; 
use flate2::write::ZlibDecoder; 
use std::fs::OpenOptions;

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
            NyxCommand::HashObject { path } => hash_object(path)?,
            NyxCommand::CatFile { hash } => cat_file(hash)?,
            NyxCommand::Add { file_path } => add(file_path)?,
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

pub fn hash_object(path: &str) -> Result<(), NyxError> {
    // TODO: Should be callable from all dirs within the repo
    if !Path::new(".nyx").join("objects").exists() {
        // TODO: logging concept
        panic!("Not in a nyx repository");
    }

    let mut content = fs::read_to_string(PathBuf::from(path)).expect("Unable to read file");

    // Todo: Currently only blob types are supported
    content = append_object_header(&content, NyxObjectType::Blob);
    let sha1 = calculate_sha1(&content);

    let object_dir = &sha1[..2];
    let object_file = &sha1[2..];

    let object_dir_path: PathBuf = [".nyx", "objects", &object_dir].iter().collect();

    if !object_dir_path.exists() {
        fs::create_dir(&object_dir_path)?;
    }

    let mut file = fs::File::create(object_dir_path.join(&object_file))?;

    let compressed_bytes = zlib_compress(&content)?;
    //let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    //encoder.write_all(content.as_bytes())?;
    //let compressed_bytes = encoder.finish()?;

    file.write(&compressed_bytes)?;

    println!("{sha1}");

    Ok(())
}

// TODO: Change api of zlib_compress to accept a byte slice
fn zlib_compress(content: &str) -> Result<Vec<u8>, NyxError> {
   let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default()); 
   encoder.write_all(content.as_bytes())?;
   let compressed_bytes = encoder.finish()?;
   Ok(compressed_bytes)
}

fn zlib_decompress(content: &[u8]) -> Result<Vec<u8>, NyxError> {
    let mut writer = Vec::new();
    let mut decoder = ZlibDecoder::new(writer);
    decoder.write_all(content)?;
    writer = decoder.finish()?;
    Ok(writer)
}

fn cat_file(hash: &str) -> Result<(), NyxError> {
    // TODO: In every directory callable
    let path: PathBuf = [".nyx", "objects", &hash[..2], &hash[2..]].iter().collect();

    let content = fs::read(path)?;

    let decompressed_bytes = zlib_decompress(&content)?;
    
    // Remove header
    let index = decompressed_bytes.iter().position(|x| *x == 0).unwrap();
    let decompressed_bytes = (&decompressed_bytes[index..]).to_vec();

    let decompressed = String::from_utf8(decompressed_bytes)?;
    println!("{}", decompressed); 
    Ok(())
}

fn add(file_path: &str) -> Result<(), NyxError> {
    // let mut appender = FileAccessor::Appender(path);
    // appender.append(b"asdfasdf");
    let mut file = OpenOptions::new()
    .append(true)
    .open([".nyx", "index"].iter().collect::<PathBuf>())?;
    
    let mut content = fs::read_to_string(PathBuf::from(file_path))?;
    let sha1 = calculate_sha1(&content);
    let content = format!("{} {}\n", &sha1, &file_path);
//    file.write();

    Ok(())
}

fn calculate_sha1(content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

fn append_object_header(content: &str, object_type: NyxObjectType) -> String {
    format!(
        "{} {}\0{}",
        object_type.to_string().to_lowercase(),
        &content.as_bytes().len().to_string(),
        content
    )
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
}
