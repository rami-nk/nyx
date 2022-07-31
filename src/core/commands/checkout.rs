use std::{fs, process, path::PathBuf};

use crate::{FILE_SYSTEM, core::{shared::read_object_data, commit::Commit, tree::tree::Tree, object_type::NyxObjectType}};

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