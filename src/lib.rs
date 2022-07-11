#![feature(drain_filter)]
use core::panic;
use format_bytes::format_bytes;
use sha1::{Digest, Sha1};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fs, str};

pub mod cl_args;
mod errors;
mod index;
mod object_type;

use cl_args::{NyxCli, NyxCommand};
use errors::NyxError;
use index::Index;
use object_type::NyxObjectType;

// TODO: Encapsulate command matching logic and check if repo alredy setup
pub fn run(cli: NyxCli) -> Result<(), NyxError> {
    match &cli.command {
        Some(command) => match command {
            NyxCommand::Init => {
                println!("Initializing nyx repo...");
                init().unwrap();
            }
            NyxCommand::HashObject { path } => {
                hash_object(path)?;
            }
            NyxCommand::CatFile { hash } => cat_file(hash)?,
            NyxCommand::Add { files } => add(files.deref().to_vec())?,
            NyxCommand::LsFile => ls_file(),
            NyxCommand::Commit => commit(),
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

    let content = fs::read(PathBuf::from(path))?;

    let object_hash = generate_object(&content, NyxObjectType::Blob);

    println!("{object_hash}");

    Ok(object_hash)
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

fn add(files: Vec<String>) -> Result<(), NyxError> {
    let mut index = Index::new();

    for file in files {
        let sha1 = hash_object(&file)?;
        index.add(&sha1, &file).unwrap();
    }

    Ok(())
}

fn ls_file() {
    let path = [".nyx", "index"].iter().collect::<PathBuf>();
    let content = fs::read_to_string(path).unwrap();
    println!("{content}");
}

fn commit() {
    let index_file = [".nyx", "index"].iter().collect::<PathBuf>();

    if !index_file.exists() {
        panic!("Noting to commit.");
    }

    // Generate Tree Object (currently only a single one)
    // TODO: handle file in directory (build trees)
    let index_content = fs::read_to_string(index_file).unwrap();

    let tree_hash = generate_object(index_content.as_bytes(), NyxObjectType::Tree);

    // TODO: Add parent section referencing last commit
    // Generate Commit Object
    let commit_content = format!("tree {}", tree_hash);
    let commit_hash = generate_object(commit_content.as_bytes(), NyxObjectType::Commit);

    // Write current commit's hash to track head
    fs::write([".nyx", "head"].iter().collect::<PathBuf>(), &commit_hash).unwrap();
}

fn generate_object(content: &[u8], object_type: NyxObjectType) -> String {
    let content = append_object_header(content, object_type);
    let hash = calculate_sha1(&content);

    let object_dir_path: PathBuf = [".nyx", "objects", &hash[..2]].iter().collect();

    if !object_dir_path.exists() {
        fs::create_dir(&object_dir_path).unwrap();
    }

    fs::write(object_dir_path.join(&hash[2..]), &content).unwrap();
    hash
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
