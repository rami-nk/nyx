#![feature(drain_filter, fs_try_exists)]
use lazy_static::{__Deref, lazy_static};
use std::env;

pub mod core;

use crate::core::cl_args::NyxCli;
use crate::core::cl_args::NyxCommand;
use crate::core::commands::add::add;
use crate::core::commands::cat_file::cat_file;
use crate::core::commands::checkout::checkout;
use crate::core::commands::commit::commit;
use crate::core::commands::hash_object::hash_object;
use crate::core::commands::init::init;
use crate::core::commands::log::log;
use crate::core::commands::ls_file::ls_file;
use crate::core::commands::status::status;
use crate::core::errors::NyxError;
use crate::core::file_system::NyxFileSystem;
use crate::core::object_type::NyxObjectType;

lazy_static! {
    static ref FILE_SYSTEM: NyxFileSystem = NyxFileSystem::new();
}

pub fn run(cli: NyxCli) -> Result<(), NyxError> {
    if !FILE_SYSTEM.is_repository() {
        match &cli.command {
            Some(command) => match command {
                NyxCommand::Init => {
                    if let Ok(_) = init() {
                        let nyx_dir = env::current_dir().unwrap().join(".nyx");
                        println!("Initialized empty nyx repository in {:?}.", nyx_dir);
                        return Ok(());
                    }
                }
                _ => {
                    eprintln!("Not a nyx repository (or any of the parent directories)");
                    std::process::exit(1);
                }
            },
            _ => (),
        }
    }

    match &cli.command {
        Some(command) => match command {
            NyxCommand::HashObject { path } => _ = hash_object(path)?,
            NyxCommand::CatFile { hash } => _ = cat_file(hash)?,
            NyxCommand::Add { paths } => add(paths.deref().to_vec())?,
            NyxCommand::LsFile => ls_file(),
            NyxCommand::Commit { message } => commit(message),
            NyxCommand::Status => status(),
            NyxCommand::Log => log(),
            NyxCommand::Checkout { hash } => checkout(hash),
            NyxCommand::Init => {
                eprintln!("Repository already initialized");
                std::process::exit(1);
            }
        },
        None => println!("Command not known! Type nyx --help for help"),
    };
    Ok(())
}
