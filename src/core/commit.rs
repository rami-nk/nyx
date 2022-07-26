use colored::Colorize;
use std::fmt::Display;
use std::fs;

use crate::FILE_SYSTEM;

use super::object_type::NyxObjectType;
use super::shared::{generate_object, read_object_data};

#[derive(Debug)]
pub struct Commit {
    tree_hash: String,
    parent_hash: String,
    hash: String,
    message: String,
}

impl Commit {
    pub fn new(tree_hash: &str, message: &str) -> Self {
        let parent_hash = Commit::read_current_commit_hash();

        Self {
            tree_hash: tree_hash.to_string(),
            parent_hash,
            hash: String::new(),
            message: message.to_string(),
        }
    }

    pub fn from_head() -> Option<Self> {
        Commit::from_hash(&Commit::read_current_commit_hash())
    }

    pub fn from_hash(hash: &str) -> Option<Self> {
        if hash.is_empty() {
            return None;
        }
        // TODO: Implement general read object to struct method (maybe in NyxFileSystem)
        let content = read_object_data(&hash).unwrap();
        let content: Vec<&str> = content.split("\n").filter(|e| !e.is_empty()).collect();

        let message: String;
        let tree_hash: &str;
        let mut parent_hash = "";

        if content.len() == 2 {
            tree_hash = (content[0].split_whitespace().collect::<Vec<&str>>())[1];
            message = content[1].to_string();
        } else if content.len() == 3 {
            tree_hash = (content[0].split_whitespace().collect::<Vec<&str>>())[1];
            parent_hash = (content[1].split_whitespace().collect::<Vec<&str>>())[1];
            message = content[2].to_string();
        } else {
            return None;
        }

        Some(Self {
            tree_hash: tree_hash.to_string(),
            parent_hash: parent_hash.to_string(),
            hash: hash.to_string(),
            message,
        })
    }

    pub fn get_content(&self) -> String {
        let mut content = format!("tree {}\n", self.tree_hash);
        if !self.parent_hash.is_empty() {
            content = format!("{}parent {}\n", content, self.parent_hash);
        }
        if !self.message.is_empty() {
            content = format!("{}{}", content, self.message);
        }
        content
    }

    pub fn write(&mut self) {
        self.hash = generate_object(self.get_content().as_bytes(), NyxObjectType::Commit);

        // TODO: Move master to FILE_SYSTEM
        let master_path = FILE_SYSTEM.get_refs_dir_path().join("master");

        // Create master with current hash in refs dir
        fs::write(master_path, &self.hash).unwrap();

        let head_path = FILE_SYSTEM.get_head_path();

        // Create HEAD with ref to master
        let head_content = format!("ref: refs/master");
        fs::write(head_path, head_content).unwrap();
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_parent_hash(&self) -> &str {
        &self.parent_hash
    }

    fn read_current_commit_hash() -> String {
        let head_path = FILE_SYSTEM.get_head_path();
        if head_path.exists() {
            let head_content = fs::read_to_string(&head_path).unwrap();
            if head_content.contains("ref: ") {
                fs::read_to_string(FILE_SYSTEM.get_refs_dir_path().join("master")).unwrap()
            } else {
                fs::read_to_string(&head_path).unwrap()
            }
        } else {
            String::new()
        }
    }

    pub fn tree_hash(&self) -> &str {
        self.tree_hash.as_ref()
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!(
            "{} {}\n\n    {}",
            "commit".yellow(),
            self.hash.as_str().yellow(),
            self.message
        );
        write!(f, "{}", output)
    }
}
