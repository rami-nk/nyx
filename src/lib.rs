#![feature(drain_filter)]
use colored::*;
use core::panic;
use format_bytes::format_bytes;
use sha1::{Digest, Sha1};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fs, str, env};

pub mod cl_args;
mod errors;
mod index;
mod object_type;
mod tree;
mod traits;
mod commit;

use cl_args::{NyxCli, NyxCommand};
use errors::NyxError;
use index::{Index, NyxFileState};
use object_type::NyxObjectType;
use commit::Commit;

// TODO: Encapsulate command matching logic and check if repo alredy setup
pub fn run(cli: NyxCli) -> Result<(), NyxError> {
    match &cli.command {
        Some(command) => match command {
            NyxCommand::Init => {
                if let Ok(_) = init() {
                    let nyx_dir = env::current_dir().unwrap().join(".nyx");
                    println!("Inizialized empty nyx repository in {:?}.", nyx_dir);
                }
            }
            NyxCommand::HashObject { path } => _ = hash_object(path)?,
            NyxCommand::CatFile { hash } => cat_file(hash)?,
            NyxCommand::Add { files } => add(files.deref().to_vec())?,
            NyxCommand::LsFile => ls_file(),
            NyxCommand::Commit { message } => commit(message),
            NyxCommand::Status => status(),
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

fn commit(message: &str) {
    // TODO: Check for ustaged changes
    // TODO: Remove all elements form staging area
    let mut index = Index::new();
    let tree = index.write_tree();
    let mut commit = Commit::new(tree, message);
    commit.write();
    println!("{}", commit.get_hash());
}

fn status() {
    // TODO: Error: Empty file is displayed as staged
    let root_dir = env::current_dir().unwrap();
    let index = Index::new();
    _status(&root_dir, &root_dir, &index);
}

fn _status(fixed_root: &PathBuf, root: &PathBuf, index: &Index) {
    for path in fs::read_dir(root).unwrap() {
        let path = &path.unwrap().path();
        // TODO: create .nyxigore file
        if path.ends_with(".nyx") ||
           path.ends_with(".git") ||
           path.ends_with("target") ||
           path.ends_with(".vscode") {
            continue;
        }
        if path.is_dir() {
            _status(fixed_root, &root.join(path), index);
        } else {
            let content = fs::read_to_string(path).unwrap();
            let content = append_object_header(content.as_bytes(), NyxObjectType::Blob);
            let hash = calculate_sha1(&content);
            let path_str = path.strip_prefix(fixed_root).unwrap().to_str().unwrap(); 
            match index.get_status(&hash, path_str) {
                NyxFileState::Staged =>   println!("{}", path_str.green()),
                NyxFileState::Modified => println!("{}", path_str.blue()),
                NyxFileState::Unstaged => println!("{}", path_str.red()),
            }
        }
    }
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
