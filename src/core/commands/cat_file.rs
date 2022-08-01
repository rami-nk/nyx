use crate::core::{errors::NyxError, shared::read_object_data};

pub fn cat_file(hash: &str) -> Result<(), NyxError> {
    let content = read_object_data(hash)?;
    println!("{}", content);
    Ok(())
}
