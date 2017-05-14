use { load, FuncMapper };
use error::*;
use libflo_module::Payload as ModulePayload;
use test::res::module;

pub unsafe fn all_dll() -> Result<(FuncMapper, ModulePayload)> {
    let all_module = try!(module::all_module());
    let (module_mapper, path_resolver) = all_module.clone().destructure();
    let all_dll = try!(load::load(&module_mapper, &path_resolver));
    Ok((all_dll, all_module))
}
