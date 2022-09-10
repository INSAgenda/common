use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Promotion {
    Stpi1,
    Stpi2,
    // TODO add the rest
}

impl FromStr for Promotion{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "STPI1" => Ok(Promotion::Stpi1),
            "STPI2" => Ok(Promotion::Stpi2),
            _ => Err(format!("Impossible to parse: {} into a Section enum.", s))
        }
    }
}

impl From<&Promotion> for &'static str {
    fn from(obj: &Promotion) -> Self {
        match obj {
            Promotion::Stpi1 => "STPI1",
            Promotion::Stpi2 => "STPI2",
        }
    }
}

impl std::fmt::Display for Promotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl Promotion {
    pub fn list() -> &'static [Promotion] {
        &[Promotion::Stpi1, Promotion::Stpi2]
    }
}
