use clap::{Parser, Subcommand};
use core::panic;
use sha1::{Digest, Sha1};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{fmt, fs, str};
use flate2::Compression;
use flate2::write::ZlibEncoder; 
use flate2::write::ZlibDecoder; 

mod errors;
use errors::NyxError;

pub fn run(cli: NyxCli) {
    match &cli.command {
        Some(command) => match command {
            NyxCommand::Init => {
                println!("Initializing nyx repo...");
                init().unwrap();
            }
            NyxCommand::HashObject { path } => hash_object(path).unwrap(),
            NyxCommand::CatFile { hash } => cat_file(hash),
            _ => (),
        },
        None => println!("Unknown command!"),
    };
}

pub fn init() -> Result<(), NyxError> {
    let dir = Path::new(".nyx");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;

    Ok(())
}

pub fn hash_object(path: &str) -> Result<(), NyxError> {
    if !Path::new(".nyx").join("objects").exists() {
        panic!("You are not in a nyx repo!");
    }

    let mut buffer = Vec::new();
    let mut file = fs::File::open(path)?;
    file.read_to_end(&mut buffer)?;
    let content_str = std::str::from_utf8(&buffer)?.trim();

    // Todo: Currently only blob types are supported
    let content = concat_content(content_str, NyxObjectType::Blob);
    let sha1 = calculate_sha1(&content);

    let object_dir = &sha1[..2];
    let object_file = &sha1[2..];

    let object_dir_path: PathBuf = [".nyx", "objects", &object_dir].iter().collect();

    if !object_dir_path.exists() {
        fs::create_dir(&object_dir_path)?;
    }

    let mut file = fs::File::create(object_dir_path.join(&object_file))?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content.as_bytes())?;
    let compressed_bytes = encoder.finish()?;

    file.write(&compressed_bytes)?;

    println!("{sha1}");

    Ok(())
}

fn cat_file(hash: &str) {
    // TODO: In every directory callable
    let path: PathBuf = [".nyx", "objects", &hash[..2], &hash[2..]].iter().collect();
    // TODO: Error Handling
    let mut file = fs::File::open(path).unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Could not read content!");

    let mut writer = Vec::new();
    let mut decoder = ZlibDecoder::new(writer);
    decoder.write_all(&content[..]).expect("Could not write compressed bytes!");
    writer = decoder.finish().unwrap();
    
    // Remove header
    let index = writer.iter().position(|x| *x == 0).unwrap();
    writer = (&writer[index..]).to_vec();

    let decompressed = String::from_utf8(writer)
    .expect("Could not convert byte array to utf-8 String!");
    println!("{}", decompressed); 
}

fn calculate_sha1(content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

fn concat_content(content: &str, object_type: NyxObjectType) -> String {
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
    Init,
    Add,
    Commit,
    // ####### LOW-LEVEL COMMANDS #######
    HashObject {
        #[clap(value_parser)]
        path: String,
    },
    CatFile {
        #[clap(value_parser)]
        hash: String,
    },
}
