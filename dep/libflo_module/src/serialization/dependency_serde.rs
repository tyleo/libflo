use serialization::{ FileDependencySerde, PackageDependencySerde };

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum DependencySerde {
    #[serde(rename = "file")]
    File(FileDependencySerde),
    #[serde(rename = "package")]
    Package(PackageDependencySerde),
}
