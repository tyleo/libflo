#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ParameterSerde {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "string")]
    String
}
