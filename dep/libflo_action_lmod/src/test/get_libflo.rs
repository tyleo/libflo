use libflo_std::Libflo;
use std::env;
use std::path::PathBuf;

fn current_exe_path() -> Option<PathBuf> {
    Some(env::current_dir().unwrap())
}

fn empty_search_paths() -> Option<Vec<PathBuf>> {
    Some(Vec::new())
}

fn root_path() -> PathBuf {
    let mut current_exe_path = current_exe_path().unwrap();
    let root_path = "test/res/module";
    current_exe_path.push(root_path);
    current_exe_path
}

pub unsafe fn get_libflo() -> Libflo {
    Libflo::load(root_path(), current_exe_path(), empty_search_paths(), None, None).unwrap()
}
