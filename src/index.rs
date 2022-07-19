use format_bytes::format_bytes;
use std::{fs, vec};
use std::io::Write;
use std::path::PathBuf;

use crate::errors::NyxError;
use crate::generate_object;
use crate::object_type::NyxObjectType;
use crate::tree::Tree;
use crate::traits::Byte;

pub struct Index {
    path: PathBuf,
    entries: Vec<IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        let path = [".nyx", "index"].iter().collect::<PathBuf>();
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
        
        self.write_content();
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
        self.write_content();
    }
    
    fn write_content(&self) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .unwrap();

        let entries_bytes: Vec<Vec<u8>> = self.entries
            .iter()
            .map(|entry| format_bytes!(b"{}\n", entry.as_bytes()))
            .collect();
        let entries_bytes = entries_bytes.concat();
        file.write_all(&entries_bytes).unwrap();
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
                
                for j in idx+1..index.len() {
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

#[derive(Debug, Clone)]
pub enum NyxFileState {
    Invalid = 0,
    Unstaged = 1,
    Staged = 2,
    Modified = 3,
    Committed = 4,
}

// TODO: Search for safe approach
impl NyxFileState {
    fn from_u8(u: u8) -> NyxFileState {
        match u {
            1 => NyxFileState::Unstaged,
            2 => NyxFileState::Staged,
            3 => NyxFileState::Modified,
            4 => NyxFileState::Committed,
            _ => NyxFileState::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
struct IndexEntry {
    hash: String,
    path: String,
    state: NyxFileState,
}

impl Byte for IndexEntry {
    fn as_bytes(&self) -> Vec<u8> {
        let state = self.state as u8;
        format_bytes!(b"{} {} {}", self.hash.as_bytes(), self.path.as_bytes(), state)
    }
}

impl IndexEntry {
    fn has_dir(&self) -> bool {
        self.path.contains("/")
    }
}
