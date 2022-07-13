use format_bytes::format_bytes;
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
    
    pub fn add_tree(&mut self, hash: &str, dir: &str) {
        // check if dir is already in tree and override if so 
        
        self.entries.retain(|e| e.path != dir);
        
        self.entries.push(TreeEntry { 
            entry_type: NyxObjectType::Tree, 
            hash: hash.to_string(),
            path: dir.to_string() });
    }
    
    pub fn add_tree_aber_wirklich_diesmal(&mut self, tree: Tree) {
        self.trees.push(tree);
    }

    pub fn add_blob(&mut self, hash: &str, dir: &str) {
        self.entries.push(TreeEntry { 
            entry_type: NyxObjectType::Blob, 
            hash: hash.to_string(),
            path: dir.to_string() });
    }
    
    pub fn set_hash(&mut self, hash: &str) {
        self.hash = hash.to_string();
    }
}

#[derive(Debug, Clone)]
pub struct TreeEntry {
    entry_type: NyxObjectType,
    hash: String,
    path: String,
}

impl Byte for TreeEntry {
    fn as_bytes(&self) -> Vec<u8> {
        format_bytes!(b"{} {} {}", self.entry_type.to_string().to_lowercase().as_bytes(), self.hash.as_bytes(), self.path.as_bytes())
    }
}

pub trait Byte {
    fn as_bytes(&self) -> Vec<u8>;
}

impl Byte for Vec<TreeEntry> {
    fn as_bytes(&self) -> Vec<u8> {
        let bytes_vec: Vec<Vec<u8>> = self.iter().map(|e| e.as_bytes()).collect();
        (&bytes_vec.concat()[..]).to_vec()
    }
}
