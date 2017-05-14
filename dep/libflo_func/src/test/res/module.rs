use error::*;
use libflo_module::{ load, Payload };
use test::res::path;

pub unsafe fn all_module() -> Result<Payload> {
    let basic_test_path = path::all_root_path();
    let current_exe_dir = path::current_exe_path();
    let search_paths = path::empty_search_paths();
    let result = try!(load(basic_test_path, current_exe_dir, search_paths));
    Ok(result)
}
