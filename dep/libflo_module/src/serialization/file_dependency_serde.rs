#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FileDependencySerde {
    path: String,
}

impl FileDependencySerde {
    pub fn get_path(&self) -> &String {
        &self.path
    }
}
