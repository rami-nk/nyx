use super::entry::TreeEntry;
use crate::NyxObjectType;

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
}
