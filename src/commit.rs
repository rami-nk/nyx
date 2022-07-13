use std::path::PathBuf;
use std::fs;

use crate::object_type::NyxObjectType;
use crate::{generate_object, cat_file};

#[derive(Debug)]
pub struct Commit {
    tree_hash: String,
    parent_hash: String,
    hash: String,
    message: String,
}

impl Commit {
    pub fn new(tree_hash: &str, message: &str) -> Self {
        let head_path: PathBuf = [".nyx", "HEAD"].iter().collect();
        let mut parent_hash = String::new();
        if head_path.exists() {
            parent_hash = fs::read_to_string(head_path).unwrap();
        }

        Self { 
            tree_hash: tree_hash.to_string(),
            parent_hash,
            hash: String::new(),
            message: message.to_string(),
        }
    }
    
    pub fn from_head() -> Option<Self> {
        let head_path: PathBuf = [".nyx", "HEAD"].iter().collect();
        let mut hash = String::new();
        if head_path.exists() {
            hash = fs::read_to_string(head_path).unwrap();
        }
        
        Commit::from_hash(&hash)
    }
    
    pub fn get_content(&self) -> String {
        let mut content = format!("tree {}", self.tree_hash);
        if !self.parent_hash.is_empty() {
            content = format!("{}\nparent {}", content, self.parent_hash);
        }
        if !self.message.is_empty() {
            content = format!("{}\n\n\n{}", content, self.message);
        }
        content
    }

    pub fn from_hash(hash: &str) -> Option<Self> {
        // TODO: Implement general read object to struct method (maybe in NyxFileSystem)
        let content = cat_file(&hash).unwrap();
        let content: Vec<&str> = content.split("\n").filter(|e| !e.is_empty()).collect();
        
        if content.len() != 3 {
            return None;
        }
        
        let tree_hash = (content[0].split_whitespace().collect::<Vec<&str>>())[1];
        let parent_hash = (content[1].split_whitespace().collect::<Vec<&str>>())[1];
        
        Some(Self { 
            tree_hash: tree_hash.to_string(),
            parent_hash: parent_hash.to_string(),
            hash: hash.to_string(),
            message: content[2].to_string()
        })
    }
    
    pub fn write(&mut self) {
        self.hash = generate_object(self.get_content().as_bytes(), NyxObjectType::Commit);

        let head_path: PathBuf = [".nyx", "HEAD"].iter().collect();
        fs::write(head_path, &self.hash).unwrap();
    }
    
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_parent_hash(&self) -> &str {
        &self.parent_hash
    }
}
