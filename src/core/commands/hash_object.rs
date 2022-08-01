use crate::core::{errors::NyxError, shared::get_object_hash};

pub fn hash_object(path: &str) -> Result<String, NyxError> {
    let object_hash = get_object_hash(path);
    println!("{object_hash}");
    Ok(object_hash)
}
