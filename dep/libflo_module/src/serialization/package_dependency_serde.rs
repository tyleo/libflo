#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct PackageDependencySerde {
    name: String,
    version: String,
}
