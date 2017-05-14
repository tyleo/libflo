use func::Input;
use test::res::dll;

#[test]
fn load() {
    unsafe { dll::all_dll() }.unwrap();
}

#[test]
fn hello_world() {
    unsafe {
        let (func_mapper, module) = dll::all_dll().unwrap();
        let (module_mapper, _) = module.destructure();

        let libflo_func_test_0_id = module_mapper.get("libflo_func_test_0").unwrap();
        let hello_world_id = func_mapper.get_id(libflo_func_test_0_id, "hello_world".to_string()).unwrap();

        let hello_world_func = func_mapper.get(hello_world_id).unwrap();

        hello_world_func.call(Input::None).unwrap();
    }
}
