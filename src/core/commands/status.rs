use std::{path::PathBuf, fs};

use crate::{FILE_SYSTEM, core::{index::{index::Index, file_state::NyxFileState}, display_strings::DisplayStrings, shared::{append_object_header, calculate_sha1}, object_type::NyxObjectType}};

pub fn status() {
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