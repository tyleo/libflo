use { serde_json, string };
use libflo_action_std::{ ACTION_MAPPER, dispatch_fn, DispatchMap, DISPATCH_MAP, impl_construct as impl_action_construct, impl_dispatch, impl_parse, parse_fn, ParseMap, PARSE_MAP };
use libflo_std::{ impl_construct, LIBFLO, Result as FuncResult };
use serde_json::Error as SerdeJsonError;
use serialization::TestSerde;
use std::any::Any;

#[no_mangle]
pub extern fn action_construct(arg: &Any) -> FuncResult<()> {
    impl_action_construct(
        arg,
        |arg| {
            ACTION_MAPPER.set(arg.clone()).unwrap();
            try_init()
        }
    )
}

#[no_mangle]
pub extern fn construct(arg: &Any) -> FuncResult<()> {
    impl_construct(
        arg,
        |arg| {
            LIBFLO.set(arg.clone()).unwrap();
            try_init()
        }
    )
}

#[no_mangle]
pub extern fn dispatch(arg: &Any) -> FuncResult<()> {
    impl_dispatch(
        arg,
        |arg| {
            let dispatch_map = DISPATCH_MAP.read().unwrap();
            let libflo = LIBFLO.read().unwrap();
            let module_mapper = libflo.get_module_mapper();
            let action_mapper = ACTION_MAPPER.read().unwrap();
            dispatch_map.dispatch(arg, module_mapper, &action_mapper)
        }
    )
}

#[no_mangle]
pub extern fn load() -> FuncResult<()> {
    Ok(())
}

#[no_mangle]
pub extern fn parse(arg: &Any) -> FuncResult<Box<Any>> {
    impl_parse(
        arg,
        |arg| {
            let parser = PARSE_MAP.read().unwrap();
            let libflo = LIBFLO.read().unwrap();
            let module_mapper = libflo.get_module_mapper();
            let action_mapper = ACTION_MAPPER.read().unwrap();
            parser.parse(arg, module_mapper, &action_mapper)
        }
    )
}

fn try_init() -> FuncResult<()> {
    if ACTION_MAPPER.is_set().unwrap() && LIBFLO.is_set().unwrap() {
        let action_mapper = ACTION_MAPPER.read().unwrap();
        let libflo = LIBFLO.read().unwrap();
        let module_mapper = libflo.get_module_mapper();

        // Set up parser
        let test_parse = parse_fn(|value| -> Result<TestSerde, SerdeJsonError> { serde_json::from_str::<TestSerde>(value) });

        let parse_map = ParseMap::new(
            module_mapper,
            &action_mapper,
            vec![(string::module(), string::test(), test_parse)]
        ).unwrap();

        PARSE_MAP.set(parse_map).unwrap();

        // Set up dispatch map
        let test_dispatch = dispatch_fn(|value: &TestSerde| -> Result<(), SerdeJsonError> {
            assert!(value.value == 0);
            Ok(())
        });

        let dispatch_map = DispatchMap::new(
            module_mapper,
            &action_mapper,
            vec![(string::module(), string::test(), test_dispatch)]
        ).unwrap();

        DISPATCH_MAP.set(dispatch_map).unwrap();
    }

    Ok(())
}
