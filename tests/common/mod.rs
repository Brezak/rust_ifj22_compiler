use std::fs;
use std::path::Path;

pub fn load_test_code<P: AsRef<Path>>(file_name: P) -> String {
    fs::read_to_string(file_name).unwrap()
}