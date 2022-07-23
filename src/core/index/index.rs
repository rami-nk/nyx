use std::path::PathBuf;
use std::{fs, vec};

use crate::core::errors::NyxError;
use crate::core::object_type::NyxObjectType;
use crate::core::tree::tree::Tree;
use crate::{generate_object, FILE_SYSTEM};

use super::super::traits::Byte;
use super::entry::IndexEntry;
use super::file_state::NyxFileState;

pub struct Index {
    path: PathBuf,
    entries: Vec<IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        let path = FILE_SYSTEM.get_index_path();
        let mut entries = Vec::new();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap();
            let raw_entries: Vec<String> = content
                .split('\n')
                .map(|val| String::from(val))
                .filter(|val| !val.is_empty())
                .collect();
            for entry in raw_entries {
                let splits: Vec<&str> = entry.split_whitespace().collect();

                // TODO: Exception handling
                let file_state: NyxFileState = NyxFileState::from_u8(splits[2].parse().unwrap());
                entries.push(IndexEntry {
                    hash: splits[0].to_string(),
                    path: splits[1].to_string(),
                    state: file_state,
                });
            }
        }
        Self { path, entries }
    }

    pub fn add(&mut self, hash: &str, path: &str) -> Result<(), NyxError> {
        if self.contains_hash(&hash) {
            return Ok(());
        }

        self.entries.drain_filter(|entry| entry.path == path);

        self.entries.push(IndexEntry {
            hash: hash.to_string(),
            path: path.to_string(),
            state: NyxFileState::Staged,
        });

        FILE_SYSTEM.write_contents(&self.entries, &self.path.to_str().unwrap());
        Ok(())
    }

    pub fn write_tree(&mut self) -> Tree {
        self.entries.sort_by(|e1, e2| e1.path.cmp(&e2.path));
        // TODO: Check for errors befor writing
        self.mark_as_committed_and_write();
        let tree = Index::write_tree_recursiv(&mut self.entries);
        tree
    }

    fn mark_as_committed_and_write(&mut self) {
        for mut entry in &mut self.entries {
            entry.state = NyxFileState::Committed;
        }
        FILE_SYSTEM.write_contents(&self.entries, &self.path.to_str().unwrap());
    }

    fn write_tree_recursiv(index: &mut Vec<IndexEntry>) -> Tree {
        let mut tree = Tree::new();

        let mut idx = 0;
        while idx < index.len() {
            if index[idx].has_dir() {
                let i_x = index[idx].path.find("/").unwrap();
                let mut dir = index[idx].path.clone();
                dir.replace_range(i_x.., "");

                let prefix = format!("{}/", dir);
                index[idx].path = index[idx].path.replacen(&prefix, "", 1);

                let mut same_dir_entries = vec![index[idx].clone()];

                for j in idx + 1..index.len() {
                    let entry = &index[j];
                    if entry.path.starts_with(&prefix) {
                        let mut entry = entry.clone();
                        entry.path = entry.path.replacen(&prefix, "", 1);
                        same_dir_entries.push(entry);
                        idx += 1;
                    } else {
                        break;
                    }
                }

                let mut new_tree = Index::write_tree_recursiv(&mut same_dir_entries);
                new_tree.path = dir;

                tree.add_tree(new_tree);
            } else {
                tree.add_blob(&index[idx].hash, &index[idx].path);
            }
            idx += 1;
        }

        let hash = generate_object(&tree.entries.as_bytes()[..], NyxObjectType::Tree);
        tree.set_hash(&hash);

        tree
    }

    fn contains_hash(&self, hash: &str) -> bool {
        self.entries.iter().any(|entry| entry.hash == hash)
    }

    pub fn get_status(&self, hash: &str, path: &str) -> NyxFileState {
        match self.entries.iter().find(|e| e.hash == hash) {
            Some(entry) => entry.state.clone(),
            None => match self.entries.iter().find(|e| e.path == path) {
                Some(_) => NyxFileState::Modified,
                None => NyxFileState::Unstaged,
            },
        }
    }
}
