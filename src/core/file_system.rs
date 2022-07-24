use std::env;
use std::fs;
use std::path::PathBuf;
use format_bytes::format_bytes;
use std::io::Write;

use super::traits::Byte;

pub struct NyxFileSystem {
    root_dir: PathBuf,
    is_repo: bool,
}

impl NyxFileSystem {
    fn nyx_dir() -> String {
        String::from(".nyx")
    }

    fn objects_dir() -> String {
        String::from("objects")
    }

    fn head_file() -> String {
        String::from("HEAD")
    }

    fn index_file() -> String {
        String::from("index")
    }

    pub fn new() -> Self {
        let mut path = env::current_dir().unwrap();
        let mut root_dir = "";

        let is_repo = loop {
            let exists = path.join(NyxFileSystem::nyx_dir()).exists();
            if exists {
                root_dir = path.to_str().unwrap();
                break true;
            }
            let success = path.pop();
            if !success {
                break false;
            }
        };

        Self {
            root_dir: PathBuf::from(root_dir),
            is_repo,
        }
    }

    pub fn is_repository(&self) -> bool {
        self.is_repo
    }

    pub fn get_root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn get_objects_dir_path(&self) -> PathBuf {
        self.get_root_dir()
            .join(NyxFileSystem::nyx_dir())
            .join(NyxFileSystem::objects_dir())
    }

    pub fn get_object_path(&self, dir_name: &str, file_name: &str) -> PathBuf {
        self.get_objects_dir_path().join(dir_name).join(file_name)
    }

    pub fn get_object_dir_path(&self, dir_name: &str) -> PathBuf {
        self.get_objects_dir_path().join(dir_name)
    }

    pub fn get_head_path(&self) -> PathBuf {
        self.root_dir
            .join(NyxFileSystem::nyx_dir())
            .join(NyxFileSystem::head_file())
    }

    pub fn get_index_path(&self) -> PathBuf {
        self.root_dir
            .join(NyxFileSystem::nyx_dir())
            .join(NyxFileSystem::index_file())
    }
    
    pub fn write_contents<T: Byte>(&self, content: &Vec<T>, path: &str) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        let entries_bytes: Vec<Vec<u8>> = content.iter()
            .map(|entry| format_bytes!(b"{}\n", entry.as_bytes()))
            .collect();
        let entries_bytes = entries_bytes.concat();
        file.write_all(&entries_bytes).unwrap();
    }
    
    fn get_ignored_files(&self) -> Vec<String> {
        let nyx_ignore_content = fs::read_to_string(self.root_dir.join(".nyxignore"));
        let mut ignored: Vec<String> = Vec::new();
        if let Ok(lines) = nyx_ignore_content {
            for line in lines.trim().split("\n").filter(|e| !e.is_empty()) {
                ignored.push(line.to_string());
            }
        }
        ignored.push(String::from(".nyx"));
        ignored
    }
    
    pub fn is_ignored(&self, path: &PathBuf) -> bool {
        let ignored = self.get_ignored_files();
        for ignore in ignored {
            if path.ends_with(ignore) {
                return true;
            }
        }
        return false;
    }
}
