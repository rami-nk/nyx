use std::io::Write;
use std::path::PathBuf;
use std::fs;
use format_bytes::format_bytes;

use crate::errors::NyxError;

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
           let raw_entries: Vec<String> = content.split('\n')
                                .map(|val| String::from(val))
                                .filter(|val| !val.is_empty())
                                .collect();
           for entry in raw_entries {
               let splits: Vec<&str> = entry.split_whitespace().collect();
               entries.push(IndexEntry { hash: splits[0].to_string(), path: splits[1].to_string()});
           }
        }
        Self {
            path,
            entries,
        }
    }
    
    pub fn add(&mut self, hash: &str, path: &str) -> Result<(), NyxError> {
        if self.contains_hash(&hash) {
            return Ok(());
        }

        self.entries.drain_filter(|entry| entry.path == path);

        println!("{:?}", self.entries);
        let index_entry = IndexEntry { hash: hash.to_string(), path: path.to_string()};
        self.entries.push(index_entry);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path).unwrap();

        println!("{:?}", self.entries);
        let entries_bytes: Vec<Vec<u8>> = self.entries.iter()
                            .map(|entry| format_bytes!(b"{}\n", entry.as_bytes()))
                            .collect();
        let entries_bytes = entries_bytes.concat();
        file.write_all(&entries_bytes).unwrap();
        Ok(())
    }
    
    fn contains_hash(&self, hash: &str) -> bool {
        self.entries.iter().any(|entry| entry.hash == hash)
    }
}

#[derive(Debug)]
struct IndexEntry {
    hash: String,
    path: String,
}

impl IndexEntry {
    fn as_bytes(&self) -> Vec<u8> {
       format_bytes!(b"{} {}", self.hash.as_bytes(), self.path.as_bytes())
    }
}

