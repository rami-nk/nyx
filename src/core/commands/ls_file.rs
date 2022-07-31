use std::fs;

use crate::FILE_SYSTEM;

pub fn ls_file() {
    let path = FILE_SYSTEM.get_index_path();
    let content = fs::read_to_string(path).unwrap();
    println!("{content}");
}
