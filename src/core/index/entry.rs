use format_bytes::format_bytes;

use crate::core::traits::Byte;

use super::file_state::NyxFileState;

#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub hash: String,
    pub path: String,
    pub state: NyxFileState,
}

impl Byte for IndexEntry {
    fn as_bytes(&self) -> Vec<u8> {
        let state = self.state as u8;
        format_bytes!(
            b"{} {} {}",
            self.hash.as_bytes(),
            self.path.as_bytes(),
            state
        )
    }
}

impl IndexEntry {
    pub fn has_dir(&self) -> bool {
        self.path.contains("/")
    }
}
