use crate::core::commit::Commit;

pub fn log() {
    let mut commit = Commit::from_head();

    while let Some(c) = &commit {
        println!("{}\n", c);
        commit = Commit::from_hash(&c.get_parent_hash());
    }
}
