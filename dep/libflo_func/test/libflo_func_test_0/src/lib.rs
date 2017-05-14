extern crate libflo_error;

use libflo_error::*;

#[no_mangle]
pub fn hello_world() -> Result<()> {
    println!("hello_world");
    Ok(())
}
