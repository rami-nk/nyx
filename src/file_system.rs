use std::path::PathBuf;
use std::env;

pub struct NyxFileSystem {
    root_dir: PathBuf,
    is_repo: bool,
}

impl NyxFileSystem {
    pub fn new() -> Self {
        let mut path = env::current_dir().unwrap();
        let mut root_dir = "";
        
        let is_repo = loop {
            let exists = path.join(".nyx").exists();
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
            is_repo
        }
    }

    pub fn is_repository(&self) -> bool {
        self.is_repo
    }
    
    pub fn get_root_dir(&self) -> &PathBuf {
        &self.root_dir
    }
    
    pub fn get_objects_path(&self, dir_name: &str, file_name: &str) -> PathBuf {
        self.get_root_dir().join(".nyx").join("objects")
                                        .join(dir_name)
                                        .join(file_name)
    }
}