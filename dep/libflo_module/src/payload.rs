use { ModuleMapper, PathResolver };

#[derive(Clone, Debug)]
pub struct Payload {
    module_mapper: ModuleMapper,
    path_resolver: PathResolver,
}

impl Payload {
    pub fn new(
        module_mapper: ModuleMapper,
        path_resolver: PathResolver,
    ) -> Self {
        Payload {
            module_mapper: module_mapper,
            path_resolver: path_resolver,
        }
    }

    pub fn destructure(self) -> (ModuleMapper, PathResolver) {
        (
            self.module_mapper,
            self.path_resolver,
        )
    }
}
