use std::{path::PathBuf, fs};

use crate::{core::{index::index::Index, errors::NyxError, shared::get_object_hash}, FILE_SYSTEM};

pub fn add(paths: Vec<String>) -> Result<(), NyxError> {
    let mut index = Index::new();

    for path in paths {
        add_recursive(&path, &mut index);
    }
    Ok(())
}

fn add_recursive(path: &str, index: &mut Index) {
    let path = PathBuf::from(path);
    if path.is_dir() {
        if FILE_SYSTEM.is_ignored(&path) {
            return;
        }
        for p in fs::read_dir(&path).unwrap() {
            let relative_path = path.join(p.unwrap().file_name());
            add_recursive(relative_path.to_str().unwrap(), index);
        }
    } else {
        let path = path.to_str().unwrap();
        let sha1 = get_object_hash(path);
        index.add(&sha1, path).unwrap();
    }
}
