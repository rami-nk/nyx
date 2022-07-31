use std::fs;

use crate::{core::errors::NyxError, FILE_SYSTEM};

pub fn init() -> Result<(), NyxError> {
    fs::create_dir_all(FILE_SYSTEM.get_objects_dir_path())?;
    fs::create_dir_all(FILE_SYSTEM.get_refs_dir_path())?;
    Ok(())
}
