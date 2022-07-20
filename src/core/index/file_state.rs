#[derive(Debug, Clone)]
pub enum NyxFileState {
    Invalid = 0,
    Unstaged = 1,
    Staged = 2,
    Modified = 3,
    Committed = 4,
}

// TODO: Search for safe approach
impl NyxFileState {
    pub fn from_u8(u: u8) -> NyxFileState {
        match u {
            1 => NyxFileState::Unstaged,
            2 => NyxFileState::Staged,
            3 => NyxFileState::Modified,
            4 => NyxFileState::Committed,
            _ => NyxFileState::Invalid,
        }
    }
}