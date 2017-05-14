extern crate libflo_error;

use libflo_error::Result;
use std::any::Any;

#[no_mangle]
pub fn hello_world() -> Result<()> {
    println!("hello_world");
    Ok(())
}

#[no_mangle]
pub fn construct(_: &Any) -> Result<()> {
    println!("construct");
    assert!(true);
    Ok(())
}
