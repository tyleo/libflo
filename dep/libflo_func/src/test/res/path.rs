use std::env;
use std::path::PathBuf;

pub fn all_root_path() -> PathBuf {
    let mut current_exe_path = current_exe_path().unwrap();
    let basic_test_root_path = "test/res/module";
    current_exe_path.push(basic_test_root_path);
    current_exe_path
}

pub fn current_exe_path() -> Option<PathBuf> {
    Some(env::current_dir().unwrap())
}

pub fn empty_search_paths() -> Option<Vec<PathBuf>> {
    Some(Vec::new())
}
