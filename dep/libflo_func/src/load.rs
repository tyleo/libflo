use { file_funcs, FuncMapper, string };
use error::*;
use func::{ Func, new_func };
use libflo_module::{ ModuleMapper, PathResolver, self_module_name };
use serialization::{ FuncSerde, LibsSerde, OsSerde, ParameterSerde, PlatformSerde };
use sharedlib::LibArc;
use std::collections::HashMap;
use std::path::PathBuf;

pub unsafe fn load(
    module_mapper: &ModuleMapper,
    path_resolver: &PathResolver,
) -> Result<FuncMapper> {
    let my_id = module_mapper.get(self_module_name())?;
    let func_file_name = string::func_file_name();

    let mut pre_func_list = Vec::new();
    let mut pre_func_map = module_mapper.get_raw_map().into_iter().map(|_| None).collect();

    for (module_name, module_id) in module_mapper.get_raw_map() {
        if let Some(libs_path) = path_resolver.try_find_submodule_file_path(func_file_name, *module_id, my_id)? {
            let libs = file_funcs::libs_from_path(libs_path)?;
            create_funcs(libs, module_name, *module_id, my_id, &path_resolver, &mut pre_func_list, &mut pre_func_map)?;
        };
    }

    let func_mapper = FuncMapper::new(pre_func_map, pre_func_list);
    Ok(func_mapper)
}

unsafe fn create_funcs(
    libs: LibsSerde,
    module_name: &String,
    module_id: usize,
    self_module_id: usize,
    path_resolver: &PathResolver,
    pre_func_list: &mut Vec<Box<Func>>,
    pre_func_map: &mut Vec<Option<HashMap<String, usize>>>,
) -> Result<()> {
    let (lib, libs) = libs.destructure();
    if let Some(lib) = lib {
        if let Some(_) = libs {
            return Err(ErrorKind::LibAndLibsBothDefined.into());
        } else {
            let (functions, platforms) = lib.destructure();
            let path = get_path(path_resolver, module_id, self_module_id, platforms)?;
            let library = LibArc::new(path.clone()).chain_err(|| ErrorKind::DllCouldNotLoad(path, module_id)).chain_err(|| ErrorKind::ModuleInfo(module_name.clone()))?;
            for function in functions {
                create_func(function, &library, module_name, module_id, pre_func_list, pre_func_map).chain_err(|| ErrorKind::ModuleInfo(module_name.clone()))?;
            }
        }
    } else if let Some(libs) = libs {
        for lib in libs {
            let (functions, platforms) = lib.destructure();
            let path = get_path(path_resolver, module_id, self_module_id, platforms)?;
            let library = LibArc::new(path.clone()).chain_err(|| ErrorKind::DllCouldNotLoad(path, module_id)).chain_err(|| ErrorKind::ModuleInfo(module_name.clone()))?;
            for function in functions {
                create_func(function, &library, module_name, module_id, pre_func_list, pre_func_map).chain_err(|| ErrorKind::ModuleInfo(module_name.clone()))?;
            }
        }
    } else {
        return Err(ErrorKind::LibOrLibsNotDefined.into());
    }

    Ok(())
}

unsafe fn create_func(
    func: FuncSerde,
    library: &LibArc,
    module_name: &String,
    module_id: usize,
    pre_func_list: &mut Vec<Box<Func>>,
    pre_func_map: &mut Vec<Option<HashMap<String, usize>>>,
) -> Result<()> {
    let (input, output, symbol, name) = func.destructure();

    let input = input.unwrap_or(ParameterSerde::None);
    let output = output.unwrap_or(ParameterSerde::None);
    let name = name.unwrap_or(symbol.clone());
    let func_id = pre_func_list.len();

    if let Some(&None) = pre_func_map.get(module_id) {
        pre_func_map[module_id] = Some(HashMap::new());
    }

    if let Some(&mut Some(ref mut inner_func_map)) = pre_func_map.get_mut(module_id) {
        if let Some(_) = inner_func_map.insert(name.clone(), func_id) {
            return Err(ErrorKind::FuncLoadNameCollision(name, module_name.clone()).into());
        }
    }

    let func = new_func(library, symbol, input, output)?;
    pre_func_list.push(func);

    Ok(())
}

fn get_path(
    path_resolver: &PathResolver,
    module_id: usize,
    self_module_id: usize,
    platforms: Vec<PlatformSerde>
) -> Result<PathBuf> {
    for platform in platforms {
        let (os, path) = platform.destructure();
        if os == OsSerde::current() {
            let result = path_resolver.find_submodule_file_path(path, module_id, self_module_id)?;
            return Ok(result);
        }
    }

    Err(ErrorKind::PlatformNotSupported(OsSerde::current()).into())
}
