use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct NyxCli {
    #[clap(subcommand)]
    pub command: Option<NyxCommand>,
}

#[derive(Subcommand)]
pub enum NyxCommand {
    /// Creates an empty nyx repository
    Init,
    /// Adds one or many files to staging area
    Add {
        #[clap(value_parser)]
        paths: Vec<String>,
    },
    /// Record changes to the repository
    Commit {
        #[clap(short, value_parser)]
        message: String,
    },
    /// Display untracked/modified files
    Status,
    /// Log commit history
    Log,
    /// Switch between commits
    Checkout {
        #[clap(value_parser)]
        hash: String,
    },

    // ##################################
    // ####### LOW-LEVEL COMMANDS #######
    // ##################################
    /// Compute object ID and creates a blob object from a file
    HashObject {
        #[clap(value_parser)]
        path: String,
    },
    /// Provide content for repository object
    CatFile {
        #[clap(value_parser)]
        hash: String,
    },
    /// Provide content of index
    LsFile,
}
