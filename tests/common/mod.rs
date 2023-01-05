use std::fs;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

pub fn load_test_code<P: AsRef<Path>>(file_name: P) -> String {
    let mut path = PathBuf::from("./tests/common");
    path.push(file_name);

    fs::read_to_string(path).unwrap()
}

fn load_dir_entry(entry: DirEntry) -> Option<(String, String)> {
    let path = entry.file_name().to_str().map(ToOwned::to_owned)?;

    let src = fs::read_to_string(entry.path()).ok()?;

    Some((path, src))
}

/// Load a group of tests
///
/// # Arguments
///
/// * `folder_name`: Name of the folder the tests are in
///
/// returns: Vec<(String, String)> a vec of pairs of file name and it's contents
pub fn load_test_group<P: AsRef<Path>>(folder_name: P) -> Vec<(String, String)> {
    let mut path = PathBuf::from("./tests/common");
    path.push(folder_name);

    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(load_dir_entry)
        .collect()
}
