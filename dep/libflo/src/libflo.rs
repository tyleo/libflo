use { ExtEventMapper, ExtFuncMapper, string };
use error::*;
use libflo_event::EventMapper;
use libflo_func::{ FuncMapper, Input };
use libflo_module::{ load, ModuleMapper, PathResolver };
use std::io::Write;
use std::path::PathBuf;
use std::ops::{ Deref, DerefMut };
use std::sync::{ Arc, RwLock };

pub struct Libflo {
    event_mapper: EventMapper,
    default_err: Option<RwLock<Box<Write + Send + Sync>>>,
    default_send: Option<RwLock<Box<Write + Send + Sync>>>,
    func_mapper: FuncMapper,
    module_mapper: ModuleMapper,
    is_running: RwLock<bool>,
    path_resolver: PathResolver,
}

impl Libflo {
    pub fn new(
        event_mapper: EventMapper,
        func_mapper: FuncMapper,
        module_mapper: ModuleMapper,
        path_resolver: PathResolver,
        default_err: Option<Box<Write + Send + Sync>>,
        default_send: Option<Box<Write + Send + Sync>>,
    ) -> Self {
        Libflo {
            event_mapper: event_mapper,
            default_err: default_err.map(|value| RwLock::new(value)),
            default_send: default_send.map(|value| RwLock::new(value)),
            func_mapper: func_mapper,
            module_mapper: module_mapper,
            is_running: RwLock::new(true),
            path_resolver: path_resolver,
        }
    }

    pub unsafe fn construct(&self, arg: &Arc<Libflo>) -> Result<()> {
        let event_mapper = self.get_event_mapper();
        let construct_event = event_mapper.get_by_module_name(string::module(), string::construct_event())?;
        construct_event.call(Input::Any(arg))?;
        self.post_construct()
    }

    pub unsafe fn err(&self, arg: &str) -> Result<()> {
        let event_mapper = self.get_event_mapper();
        let err_event = event_mapper.get_by_module_name(string::module(), string::err_event())?;
        err_event.call(Input::String(arg))?;
        self.err_internal(arg)
    }

    pub unsafe fn errln(&self, arg: &str) -> Result<()> {
        let arg = format!("{}\n", arg);
        self.err(&arg)
    }

    unsafe fn err_internal(&self, arg: &str) -> Result<()> {
        if let Some(ref err) = self.default_err {
            err.write()
                .map_err(|err| Error::from(ErrorKind::DefaultErrLockError(err.to_string())))?
                .write_all(arg.as_bytes())?;
        }
        Ok(())
    }

    pub fn get_event_mapper(&self) -> ExtEventMapper {
        ExtEventMapper::new(&self.event_mapper, &self.module_mapper)
    }

    pub fn get_func_mapper(&self) -> ExtFuncMapper {
        ExtFuncMapper::new(&self.func_mapper, &self.module_mapper)
    }

    pub fn get_module_mapper(&self) -> &ModuleMapper {
        &self.module_mapper
    }

    pub fn get_path_resolver(&self) -> &PathResolver {
        &self.path_resolver
    }

    pub fn is_running(&self) -> Result<bool> {
        Ok(
            self.is_running
                .read()
                .map_err(|err| Error::from(ErrorKind::IsRunningLockError(err.to_string())))?
                .deref()
                .clone()
        )
    }

    unsafe fn post_construct(&self) -> Result<()> {
        let event_mapper = self.get_event_mapper();
        let post_construct_event = event_mapper.get_by_module_name(string::module(), string::post_construct_event())?;
        post_construct_event.call(Input::None)?;
        Ok(())
    }

    pub unsafe fn quit(&self) -> Result<()> {
        let event_mapper = self.get_event_mapper();
        let quit_event = event_mapper.get_by_module_name(string::module(), string::quit_event())?;
        quit_event.call(Input::None)?;
        self.quit_internal()
    }

    pub unsafe fn receive(&self, arg: &str) -> Result<()> {
        let event_mapper = self.get_event_mapper();
        let receive_event = event_mapper.get_by_module_name(string::module(), string::receive_event())?;
        receive_event.call(Input::String(arg))?;
        Ok(())
    }

    pub unsafe fn send(&self, arg: &str) -> Result<()> {
        let event_mapper = self.get_event_mapper();
        let send_event = event_mapper.get_by_module_name(string::module(), string::send_event())?;
        send_event.call(Input::String(arg))?;
        self.send_internal(arg)
    }

    pub unsafe fn sendln(&self, arg: &str) -> Result<()> {
        let arg = format!("{}\n", arg);
        self.send(&arg)
    }

    fn send_internal(&self, arg: &str) -> Result<()> {
        if let Some(ref send) = self.default_send {
            send.write()
                .map_err(|err| Error::from(ErrorKind::DefaultSendLockError(err.to_string())))?
                .write_all(arg.as_bytes())?;
        }
        Ok(())
    }

    pub unsafe fn load(root_path: PathBuf, exe_path: Option<PathBuf>, search_paths: Option<Vec<PathBuf>>, default_err: Option<Box<Write + Send + Sync>>, default_send: Option<Box<Write + Send + Sync>>) -> Result<Libflo> {
        let module_payload = load(root_path, exe_path, search_paths)?;
        let (module_mapper, path_resolver) = module_payload.destructure();
        let func_mapper = FuncMapper::load(&module_mapper, &path_resolver)?;
        let event_mapper = EventMapper::load(&func_mapper, &module_mapper, &path_resolver)?;
        Ok(Self::new(event_mapper, func_mapper, module_mapper, path_resolver, default_err, default_send))
    }

    fn quit_internal(&self) -> Result<()> {
        *self.is_running
            .write()
            .map_err(|err| Error::from(ErrorKind::IsRunningLockError(err.to_string())))?
            .deref_mut() = false;
        Ok(())
    }
}
