use serialization::OsSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct PlatformSerde {
    os: OsSerde,
    path: String,
}

impl PlatformSerde {
    pub fn destructure(self) -> (OsSerde, String) {
        (self.os, self.path)
    }
}
