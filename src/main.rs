use std::path::Path;
use std::fs;

fn main() {
    init().unwrap();
}

fn init() -> Result<(), NyxError> {
    let dir = Path::new(".nyx");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;

    Ok(())
}

#[derive(Debug)]
enum NyxError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for NyxError {
   fn from(err: std::io::Error) -> Self {
      NyxError::IOError(err) 
   } 
}