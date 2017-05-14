use serialization::LibSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibsSerde {
    lib: Option<LibSerde>,
    libs: Option<Vec<LibSerde>>,
}

impl LibsSerde {
    pub fn destructure(self) -> (Option<LibSerde>, Option<Vec<LibSerde>>) {
        (self.lib, self.libs)
    }
}
