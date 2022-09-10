use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Class {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
}

impl FromStr for Class{
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Class::A),
            "B" => Ok(Class::B),
            "C" => Ok(Class::C),
            "D" => Ok(Class::D),
            "E" => Ok(Class::E),
            "F" => Ok(Class::F),
            "G" => Ok(Class::G),
            "H" => Ok(Class::H),
            "I" => Ok(Class::I),
            "J" => Ok(Class::J),
            "K" => Ok(Class::K),
            _ => Err(format!("Impossible to parse: {} into a Class enum.", s))
        }
    }

    type Err = String;
}

impl From<&Class> for &'static str {
    fn from(obj: &Class) -> Self {
        match obj {
            Class::A => "A",
            Class::B => "B",
            Class::C => "C",
            Class::D => "D",
            Class::E => "E",
            Class::F => "F",
            Class::G => "G",
            Class::H => "H",
            Class::I => "I",
            Class::J => "J",
            Class::K => "K",
        }
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl Class {
    pub fn list() -> &'static [Class] {
        &[Class::A, Class::B, Class::C, Class::D, Class::E, Class::F, Class::G, Class::H, Class::I, Class::J, Class::K]
    }
}
