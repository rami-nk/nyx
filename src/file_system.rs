use std::env;

pub struct NyxFileSystem {
    root_dir: String,
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
            root_dir: root_dir.to_string(),
            is_repo
        }
    }

    pub fn is_repository(&self) -> bool {
        self.is_repo
    }
    
    pub fn get_root_dir(&self) -> &str {
        &self.root_dir
    }
}