use std::env;

pub struct NyxFileSystem; 

impl NyxFileSystem {
    pub fn is_in_nyx_repository() -> bool {
        let mut path = env::current_dir().unwrap();
        
        loop {
            let exists = path.join(".nyx").exists();
            if exists {
                return true;
            }
            let success = path.pop();
            if !success {
                return false;
            }
        }
    }
}