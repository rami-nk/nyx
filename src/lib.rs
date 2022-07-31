#![feature(drain_filter, fs_try_exists)]
use format_bytes::format_bytes;
use lazy_static::{__Deref, lazy_static};
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use std::{env, fs, str, process};

pub mod core;

use crate::core::cl_args::NyxCli;
use crate::core::cl_args::NyxCommand;
use crate::core::commit::*;
use crate::core::display_strings::DisplayStrings;
use crate::core::errors::NyxError;
use crate::core::file_system::NyxFileSystem;
use crate::core::index::file_state::NyxFileState;
use crate::core::index::index::*;
use crate::core::object_type::NyxObjectType;
use crate::core::tree::tree::Tree;

lazy_static! {
    static ref FILE_SYSTEM: NyxFileSystem = NyxFileSystem::new();
}

pub fn run(cli: NyxCli) -> Result<(), NyxError> {
    if !FILE_SYSTEM.is_repository() {
        match &cli.command {
            Some(command) => match command {
                NyxCommand::Init => {
                    if let Ok(_) = init() {
                        let nyx_dir = env::current_dir().unwrap().join(".nyx");
                        println!("Initialized empty nyx repository in {:?}.", nyx_dir);
                        return Ok(());
                    }
                }
                _ => {
                    eprintln!("Not a nyx repository (or any of the parent directories)");
                    std::process::exit(1);
                }
            },
            _ => (),
        }
    }
    
    match &cli.command {
        Some(command) => match command {
            NyxCommand::HashObject { path } => _ = hash_object(path)?,
            NyxCommand::CatFile { hash } => _ = cat_file(hash)?,
            NyxCommand::Add { paths } => add(paths.deref().to_vec())?,
            NyxCommand::LsFile => ls_file(),
            NyxCommand::Commit { message } => commit(message),
            NyxCommand::Status => status(),
            NyxCommand::Log => log(),
            NyxCommand::Checkout { hash } => checkout(hash),
            NyxCommand::Init => {
                eprintln!("Repository already initialized");
                std::process::exit(1);
            }
        },
        None => println!("Command not known! Type nyx --help for help"),
    };
    Ok(())
}

pub fn checkout(hash: &str) {
    let mut hash = hash.to_string();
    let mut is_master = false;
    if hash.eq("master") {
        hash =  fs::read_to_string(FILE_SYSTEM.get_refs_dir_path().join("master")).unwrap();
        is_master = true;
    }

    // TODO: Move error handling to Commit::from_hash ctor
    if let Err(err) = read_object_data(&hash) {
        eprint!("{:?}", err);
        process::exit(1);
    }
    
    let commit = Commit::from_hash(&hash).unwrap();
    let tree = Tree::from_hash(commit.tree_hash());
    
    // 1. Write hash in HEAD
    fs::write(FILE_SYSTEM.get_head_path(), hash).unwrap();

    // 2. Remove all not ignored files
    // TODO: remove_dir_all not correct if .nyxignore contains specific file in dir
    for entry in fs::read_dir(FILE_SYSTEM.get_root_dir()).unwrap() {
        let entry = entry.unwrap();
        if !FILE_SYSTEM.is_ignored(&entry.path()) {
            if entry.path().is_dir() {
                fs::remove_dir_all(entry.path()).unwrap();
            } else {
                fs::remove_file(entry.path()).unwrap();
            }
        }
    }

    // 3. Resotre working tree recursively
    restore_working_tree(&tree, FILE_SYSTEM.get_root_dir().to_str().unwrap());
    
    if !is_master {
        println!("\
You are in 'detached HEAD' state.

    Undo this operation with:

        nyx checkout master
        
HEAD is now at {} {} commit",
        &commit.get_hash()[0..8], commit.message());
        return;
    }
    println!("HEAD is now at master");
}

fn restore_working_tree(tree: &Tree, path: &str) {
    for entry in &tree.entries {
        match entry.entry_type {
            NyxObjectType::Blob => {
                if !PathBuf::from(path).exists() {
                    fs::create_dir_all(path).unwrap();
                }
                let path = PathBuf::from(path).join(&entry.path);
                let content = read_object_data(&entry.hash).unwrap();
                fs::write(path, content).unwrap();
            },
            NyxObjectType::Tree => {
                let path = PathBuf::from(path).join(&entry.path);
                let path = path.to_str().unwrap();
                let tree = tree.get_tree_by_hash(&entry.hash).unwrap();
                restore_working_tree(tree, path);
            },
            _ => (),
        }
    }
}

pub fn init() -> Result<(), NyxError> {
    fs::create_dir_all(FILE_SYSTEM.get_objects_dir_path())?;
    fs::create_dir_all(FILE_SYSTEM.get_refs_dir_path())?;
    Ok(())
}

fn get_object_hash(path: &str) -> String {
    let content = fs::read(path).unwrap();
    let object_hash = generate_object(&content, NyxObjectType::Blob);

    return object_hash;
}

pub fn hash_object(path: &str) -> Result<String, NyxError> {
    let object_hash = get_object_hash(path);
    println!("{object_hash}");
    Ok(object_hash)
}

fn cat_file(hash: &str) -> Result<(), NyxError> {
    let content = read_object_data(hash)?;
    println!("{}", content);
    Ok(())
}

fn read_object_data(hash: &str) -> Result<String, NyxError> {
    let path = FILE_SYSTEM.get_object_path(&hash[..2], &hash[2..]);
    let content = fs::read(path)?;
    let index = &content.iter().position(|x| *x == 0).unwrap();
    let content = &content[*index+1..];

    let content = str::from_utf8(&content)?;

    Ok(content.to_string())
}

fn add(paths: Vec<String>) -> Result<(), NyxError> {
    let mut index = Index::new();

    for path in paths {
        add_recursive(&path, &mut index);
    }
    Ok(())
}

fn add_recursive(path: &str, index: &mut Index) {
    let path = PathBuf::from(path);
    if path.is_dir() {
        if FILE_SYSTEM.is_ignored(&path) {
            return;
        }
        for p in fs::read_dir(&path).unwrap() {
            let relative_path = path.join(p.unwrap().file_name());
            add_recursive(relative_path.to_str().unwrap(), index);
        }
    } else {
        let path = path.to_str().unwrap();
        let sha1 = get_object_hash(path);
        index.add(&sha1, path).unwrap();
    }
}

fn ls_file() {
    let path = FILE_SYSTEM.get_index_path();
    let content = fs::read_to_string(path).unwrap();
    println!("{content}");
}

fn commit(message: &str) {
    // TODO: Check for ustaged changes
    let mut index = Index::new();
    let tree = index.write_tree();
    let mut commit = Commit::new(&tree.hash, message);
    commit.write();
    println!("{}", commit.get_hash());
}

fn log() {
    let mut commit = Commit::from_head();

    while let Some(c) = &commit {
        println!("{}\n", c);
        commit = Commit::from_hash(&c.get_parent_hash());
    }
}

fn status() {
    // TODO: Error: Empty file is not displayed as untracked
    let root_dir = FILE_SYSTEM.get_root_dir();
    let index = Index::new();
    let mut unstaged = DisplayStrings::new(4, "red");
    let mut modified = DisplayStrings::new(4, "red");
    let mut staged = DisplayStrings::new(4, "green");
    _status(
        &root_dir,
        &root_dir,
        &index,
        &mut unstaged,
        &mut modified,
        &mut staged,
    );

    if staged.is_empty() && modified.is_empty() && unstaged.is_empty() {
        println!("Nothing to commit, working tree clean");
    }
    staged.try_print_with_prefix("Changes to be committed:");
    modified.try_print_with_prefix("Files not staged for commit:");
    unstaged.try_print_with_prefix("Untracked files:");
}

fn _status(
    fixed_root: &PathBuf,
    root: &PathBuf,
    index: &Index,
    unstaged: &mut DisplayStrings,
    modified: &mut DisplayStrings,
    staged: &mut DisplayStrings,
) {
    for path in fs::read_dir(root).unwrap() {
        let path = &path.unwrap().path();
        if FILE_SYSTEM.is_ignored(&path) {
            continue;
        }
        if path.is_dir() {
            _status(
                fixed_root,
                &root.join(path),
                index,
                unstaged,
                modified,
                staged,
            );
        } else {
            let content = fs::read_to_string(path).unwrap();
            let content = append_object_header(content.as_bytes(), NyxObjectType::Blob);
            let hash = calculate_sha1(&content);
            let path_str = path.strip_prefix(fixed_root).unwrap().to_str().unwrap();
            match index.get_status(&hash, path_str) {
                NyxFileState::Staged => staged.push(path_str),
                NyxFileState::Modified => modified.push(path_str),
                NyxFileState::Unstaged => unstaged.push(path_str),
                _ => (),
            }
        }
    }
}

// TODO: split in creation and writing
fn generate_object(content: &[u8], object_type: NyxObjectType) -> String {
    let content = append_object_header(content, object_type);
    let hash = calculate_sha1(&content);

    let object_dir_path = FILE_SYSTEM.get_object_dir_path(&hash[..2]);

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
