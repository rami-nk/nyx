use crate::core::{commit::Commit, index::index::Index};

pub fn commit(message: &str) {
    // TODO: Check for ustaged changes
    let mut index = Index::new();
    let tree = index.write_tree();
    let mut commit = Commit::new(&tree.hash, message);
    commit.write();
    println!("{}", commit.get_hash());
}
