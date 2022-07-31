use super::entry::TreeEntry;
use crate::{NyxObjectType, read_object_data};

#[derive(Debug)]
pub struct Tree {
    pub hash: String,
    pub entries: Vec<TreeEntry>,
    pub trees: Vec<Tree>,
    pub path: String,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            hash: String::new(),
            entries: Vec::new(),
            trees: Vec::new(),
            path: String::new(),
        }
    }
    
    pub fn from_hash(hash: &str) -> Self {
        Tree::from_hash_recursive(hash, ".")
    }
    
    fn from_hash_recursive(hash: &str, dir_name: &str) -> Tree {
        let mut tree = Tree::new();
        tree.set_hash(hash);
        tree.set_path(dir_name);

        let content = read_object_data(hash).unwrap();
        
        for line in content.lines() {
            if line.is_empty() {
                continue;
            }
            let line: Vec<&str> = line.split_whitespace().collect();
            
            if line[0].contains("blob") {
                tree.add_blob(line[1], line[2]);
            } else if line[0].contains("tree") {
                let referenced_tree = Tree::from_hash_recursive(line[1], line[2]);
                tree.add_tree(referenced_tree);
            }
        }
        tree
    }

    fn add_entry(&mut self, hash: &str, name: &str, entry_type: NyxObjectType) {
        self.entries.push(TreeEntry {
            entry_type: entry_type,
            hash: hash.to_string(),
            path: name.to_string(),
        });
    }

    pub fn add_blob(&mut self, hash: &str, name: &str) {
        self.add_entry(hash, name, NyxObjectType::Blob);
    }

    pub fn add_tree(&mut self, tree: Tree) {
        self.add_entry(&tree.hash, &tree.path, NyxObjectType::Tree);
        self.trees.push(tree);
    }

    pub fn set_hash(&mut self, hash: &str) {
        self.hash = hash.to_string();
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }
    
    pub fn get_tree_by_hash(&self, hash: &str) -> Option<&Tree> {
        self.trees.iter().find(|t| t.hash == hash)
    }
}
