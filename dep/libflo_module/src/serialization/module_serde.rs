use serialization::DependencySerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ModuleSerde {
    name: Option<String>,
    dependencies: Option<Vec<DependencySerde>>,
    version: Option<String>,
}

impl ModuleSerde {
    pub fn destructure(self) -> (Option<String>, Option<Vec<DependencySerde>>, Option<String>) {
        (
            self.name,
            self.dependencies,
            self.version
        )
    }
}
