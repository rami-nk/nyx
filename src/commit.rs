use std::path::PathBuf;
use std::fs;

use crate::object_type::NyxObjectType;
use crate::tree::Tree;
use crate::generate_object;

#[derive(Debug)]
pub struct Commit {
    tree: Tree,
    parent_hash: String,
    hash: String,
    message: String,
}

impl Commit {
    pub fn new(tree: Tree) -> Self {
        let head_path: PathBuf = [".nyx", "HEAD"].iter().collect();
        let mut parent_hash = String::new();
        if head_path.exists() {
            parent_hash = fs::read_to_string(head_path).unwrap();
        }

        Self { 
            tree,
            parent_hash,
            hash: String::new(),
            message: String::new(),
        }
    }
    
    fn get_content(&self) -> String {
        let mut content = format!("tree {}", self.tree.hash);
        if !self.parent_hash.is_empty() {
            content = format!("{}\nparent {}", content, self.parent_hash);
        }
        if !self.message.is_empty() {
            content = format!("{}\n\n\n{}", content, self.message);
        }
        content
    }
    
    pub fn write(&mut self) {
        self.hash = generate_object(self.get_content().as_bytes(), NyxObjectType::Commit);

        let head_path: PathBuf = [".nyx", "HEAD"].iter().collect();
        fs::write(head_path, &self.hash).unwrap();
    }
    
    pub fn get_hash(&self) -> &str {
        &self.hash
    }
}
