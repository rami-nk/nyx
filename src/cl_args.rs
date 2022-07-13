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
        files: Vec<String>,
    },
    /// Record changes to the repository
    Commit {
        #[clap(short, value_parser)]
        message: String
    },
    Status,
    Log,
    // ####### LOW-LEVEL COMMANDS #######
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
    LsFile,
}
