pub mod get_libflo;

use std::sync::Arc;

#[test]
fn load() {
    unsafe {
        get_libflo::get_libflo()
    };
}

#[test]
fn test_0() {
    unsafe {
        let libflo = Arc::new(get_libflo::get_libflo());
        libflo.construct(&libflo).unwrap();
        libflo.receive("{ \"type\": \"libflo_action_test_0 test\", \"value\": 0}").unwrap();
    }
}
