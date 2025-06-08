use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

#[must_use]
pub fn current_exe_location() -> PathBuf {
    env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {err}");
        process::exit(1);
    })
}

#[must_use]
pub fn get_all_files_in_location(path: &Path) -> ReadDir {
    fs::read_dir(path).unwrap_or_else(|err| {
        error!("Problem reading directory {}, error: {err}", path.display());
        process::exit(1);
    })
}
