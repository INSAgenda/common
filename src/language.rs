use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Language {
    All,
    Esp,
    AllDeb,
    EspDeb,
    /// Français langue étrangère
    Fle,
}

impl FromStr for Language{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ALL" => Ok(Language::All),
            "ALL-DEB" => Ok(Language::AllDeb),
            "ESP" => Ok(Language::Esp),
            "ESP-DEB" => Ok(Language::EspDeb),
            "FLE" => Ok(Language::Fle),
            _ => Err(format!("Impossible to parse: {} into a Language enum.", s))
        }
    }
}

impl From<&Language> for &'static str {
    fn from(obj: &Language) -> Self {
        match obj {
            Language::All => "ALL",
            Language::Esp => "ESP",
            Language::AllDeb => "ALL-DEB",
            Language::EspDeb => "ESP-DEB",
            Language::Fle => "FLE",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl Language {
    pub fn list() -> &'static [Language] {
        &[Language::AllDeb, Language::EspDeb, Language::All, Language::Esp, Language::Fle]
    }
}
