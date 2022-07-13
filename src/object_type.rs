use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub enum NyxObjectType {
    Commit,
    Tree,
    Blob,
}

impl fmt::Display for NyxObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
