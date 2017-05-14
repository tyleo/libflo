#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum OsSerde {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    MacOs,
    #[serde(rename = "windows")]
    Windows,
}

impl OsSerde {
    #[cfg(target_os = "linux")]
    pub fn current() -> Self {
        OsSerde::Linux
    }

    #[cfg(target_os = "macos")]
    pub fn current() -> Self {
        OsSerde::MacOs
    }

    #[cfg(target_os = "windows")]
    pub fn current() -> Self {
        OsSerde::Windows
    }
}
