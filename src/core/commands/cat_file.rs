use crate::core::{shared::read_object_data, errors::NyxError};

pub fn cat_file(hash: &str) -> Result<(), NyxError> {
    let content = read_object_data(hash)?;
    println!("{}", content);
    Ok(())
}
