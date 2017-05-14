use test::res::module;

#[test]
pub fn load() {
    unsafe { module::all_module() }.unwrap();
}

#[test]
pub fn verify_names() {
    let module = unsafe { module::all_module() }.unwrap();
    let (module_mapper, _) = module.destructure();

    assert_eq!(module_mapper.get_raw_map().len(), 1);
    for (module_name, module_id) in module_mapper.get_raw_map() {
        let mapped_module_id = module_mapper.get(module_name).unwrap();
        assert_eq!(*module_id, mapped_module_id);
    }
}
