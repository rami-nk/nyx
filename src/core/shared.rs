use std::fs;
use format_bytes::format_bytes;
use sha1::{Sha1, Digest};

use crate::FILE_SYSTEM;

use super::{object_type::NyxObjectType, errors::NyxError};

// TODO: split in creation and writing
pub fn generate_object(content: &[u8], object_type: NyxObjectType) -> String {
    let content = append_object_header(content, object_type);
    let hash = calculate_sha1(&content);

    let object_dir_path = FILE_SYSTEM.get_object_dir_path(&hash[..2]);

    if !object_dir_path.exists() {
        fs::create_dir(&object_dir_path).unwrap();
    }

    fs::write(object_dir_path.join(&hash[2..]), &content).unwrap();
    hash
}

pub fn calculate_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

pub fn append_object_header(content: &[u8], object_type: NyxObjectType) -> Vec<u8> {
    let object_type_bytes = object_type.to_string().to_lowercase().as_bytes().to_vec();
    let content_len_bytes = content.len().to_string().as_bytes().to_vec();
    format_bytes!(b"{} {}\0{}", object_type_bytes, content_len_bytes, content)
}

pub fn read_object_data(hash: &str) -> Result<String, NyxError> {
    let path = FILE_SYSTEM.get_object_path(&hash[..2], &hash[2..]);
    let content = fs::read(path)?;
    let index = &content.iter().position(|x| *x == 0).unwrap();
    let content = &content[*index+1..];

    let content = std::str::from_utf8(&content)?;

    Ok(content.to_string())
}
