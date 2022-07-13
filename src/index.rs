use format_bytes::format_bytes;
use std::{fs, vec};
use std::io::Write;
use std::path::PathBuf;

use crate::{append_object_header, calculate_sha1};
use crate::errors::NyxError;
use crate::object_type::NyxObjectType;
use crate::tree::Tree;

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
                entries.push(IndexEntry {
                    hash: splits[0].to_string(),
                    path: splits[1].to_string(),
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
        });

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .unwrap();

        let entries_bytes: Vec<Vec<u8>> = self
            .entries
            .iter()
            .map(|entry| format_bytes!(b"{}\n", entry.as_bytes()))
            .collect();
        let entries_bytes = entries_bytes.concat();
        file.write_all(&entries_bytes).unwrap();
        Ok(())
    }
    
    pub fn write_tree(&mut self) -> Tree {
        self.entries.sort_by(|e1, e2| e1.path.cmp(&e2.path));
        let tree = Index::_write_tree(&mut self.entries);
        tree
    }
    
    fn _write_tree(index: &mut Vec<IndexEntry>) -> Tree {
        let mut tree = Tree::new();
        
        let mut idx = 0;
        while idx < index.len() {
            if index[idx].has_dir() {

                let i_x = index[idx].path.find("/").unwrap();
                let mut dir = index[idx].path.clone();
                dir.replace_range(i_x.., "");

                let prefix = format!("{}/", dir);
                // TODO: very bad!!
                index[idx].path = index[idx].path.replace(&prefix, "");
                
                let mut same_dir_entries = vec![index[idx].clone()];
                
                for j in idx+1..index.len() {
                    let entry = &index[j];
                    if entry.path.contains(&prefix) {
                        let mut entry = entry.clone();
                        entry.path = entry.path.replace(&prefix, "");
                        same_dir_entries.push(entry);
                        idx += 1;
                    } else {
                        break;
                    }
                }

                let mut new_tree = Index::_write_tree(&mut same_dir_entries);
                new_tree.path = dir.clone();

                let new_tree_hash = new_tree.hash.clone();
                
                if let Some(t) = tree.with_path(&dir) {
                    let a = t.entries.clone();
                    for entry in a {
                        new_tree.entries.push(entry);
                    }
                }

                if let Some(t) = tree.with_path(&dir) {
                    tree.remove_tree(&t.hash.clone());
                }

                tree.add_tree_aber_wirklich_diesmal(new_tree);

                tree.add_tree(&new_tree_hash, &dir);
            } else {
                tree.add_blob(&index[idx].hash, &index[idx].path);
            }
            idx += 1;
        }

        // TODO: Hash calculation not correct
        let bytes: Vec<Vec<u8>> = tree.entries.iter().map(|e| e.as_bytes()).collect();
        let content = append_object_header(&bytes.concat()[..], NyxObjectType::Tree);
        let hash = calculate_sha1(&content);
        tree.set_hash(&hash);
        
        tree
    }

    fn contains_hash(&self, hash: &str) -> bool {
        self.entries.iter().any(|entry| entry.hash == hash)
    }
    
    pub fn contains_file(&self, path: &str) -> bool {
        self.entries.iter().any(|entry| entry.path == path)
    }
  
    pub fn get_status(&self, hash: &str, path: &str) -> NyxFileState {
        if self.contains_hash(hash) {
            return NyxFileState::Staged;
        }
        if self.contains_file(path) {
            return NyxFileState::Modified;
        }
        NyxFileState::Unstaged
    }
}

pub enum NyxFileState {
    Unstaged,
    Staged,
    Modified,
}

#[derive(Debug, Clone)]
struct IndexEntry {
    hash: String,
    path: String,
}

impl IndexEntry {
    fn as_bytes(&self) -> Vec<u8> {
        format_bytes!(b"{} {}", self.hash.as_bytes(), self.path.as_bytes())
    }
    
    fn has_dir(&self) -> bool {
        self.path.contains("/")
    }
}
