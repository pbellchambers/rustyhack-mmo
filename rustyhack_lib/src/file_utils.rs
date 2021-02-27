use std::fs::ReadDir;
use std::path::PathBuf;
use std::{env, fs, process};

pub fn current_exe_location() -> PathBuf {
    env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    })
}

pub fn get_all_files_in_location(path: PathBuf) -> ReadDir {
    fs::read_dir(path.as_path()).unwrap_or_else(|err| {
        error!(
            "Problem reading directory {:?}, error: {}",
            path.as_path(),
            err
        );
        process::exit(1);
    })
}
