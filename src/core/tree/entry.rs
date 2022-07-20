use crate::core::{object_type::NyxObjectType, traits::Byte};
use format_bytes::format_bytes;

#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub entry_type: NyxObjectType,
    pub hash: String,
    pub path: String,
}

impl Byte for TreeEntry {
    fn as_bytes(&self) -> Vec<u8> {
        format_bytes!(
            b"{} {} {}",
            self.entry_type.to_string().to_lowercase().as_bytes(),
            self.hash.as_bytes(),
            self.path.as_bytes()
        )
    }
}

impl Byte for Vec<TreeEntry> {
    fn as_bytes(&self) -> Vec<u8> {
        let bytes_vec: Vec<Vec<u8>> = self
            .iter()
            .map(|e| format_bytes!(b"{}\n", e.as_bytes()))
            .collect();
        (&bytes_vec.concat()[..]).to_vec()
    }
}
