use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupDescriptor {
    pub promotion: Promotion,
    pub lang: Language,
    pub class: Class,
    pub class_half: i32,
}

impl GroupDescriptor {
    pub fn new(promotion: Promotion, class: Class, lang: Language, class_half: u8) -> Result<Self, &'static str> {
        if class_half != 1 && class_half != 2 {
            return Err("class_division must be 1 or 2");
        }
        Ok(Self {
            promotion,
            lang,
            class,
            class_half: class_half as i32,
        })
    }
}

impl std::default::Default for GroupDescriptor {
    fn default() -> Self {
        Self {
            promotion: Promotion::Stpi1,
            lang: Language::All,
            class: Class::E,
            class_half: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventGroup {
    Everyone,
    Section1,
    Section2,
    Esp,
    All,
    Sib,
    A,
    A1,
    A2,
    AllA,
    AllDebA,
    EspA,
    EspDebA,
    B,
    B1,
    B2,
    AllB,
    AllDebB,
    EspB,
    EspDebB,
    C,
    C1,
    C2,
    AllC,
    AllDebC,
    EspC,
    EspDebC,
    D,
    D1,
    D2,
    AllD,
    AllDebD,
    EspD,
    EspDebD,
    E,
    E1,
    E2,
    AllE,
    AllDebE,
    EspE,
    EspDebE,
    F,
    F1,
    F2,
    AllF,
    AllDebF,
    EspF,
    EspDebF,
    G,
    G1,
    G2,
    AllG,
    AllDebG,
    EspG,
    EspDebG,
    H,
    H1,
    H2,
    AllH,
    AllDebH,
    EspH,
    EspDebH,
    I,
    I1,
    I2,
    AllI,
    AllDebI,
    EspI,
    EspDebI,
    J,
    J1,
    J2,
    AllJ,
    AllDebJ,
    EspJ,
    EspDebJ,
    K,
    K1,
    K2,
    AllK,
    AllDebK,
    EspK,
    EspDebK,
    // Special groups
    FleJK,
    FleIJ,
    JK,
    JK1,
    JK2,
    IJ,
    IJ1,
    IJ2,
    Range { groups: Vec<EventGroup>, from: String, to: String },
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl EventGroup {
    pub fn matches(&self, name: &str, class: Class, subgroup: u8, language: Language) -> bool {
        match self {
            EventGroup::Everyone => true,
            EventGroup::Section1 => [Class::A,Class::B,Class::C,Class::D].contains(&class),
            EventGroup::Section2 => [Class::E,Class::F,Class::G,Class::H].contains(&class),
            EventGroup::Sib => [Class::I,Class::J,Class::K].contains(&class),
            EventGroup::All => language == Language::All,
            EventGroup::Esp => language == Language::Esp,
            EventGroup::A => class == Class::A,
            EventGroup::A1 => class == Class::A && subgroup == 1,
            EventGroup::A2 => class == Class::A && subgroup == 2,
            EventGroup::AllA => class == Class::A && language == Language::All,
            EventGroup::AllDebA => class == Class::A && language == Language::AllDeb,
            EventGroup::EspA => class == Class::A && language == Language::Esp,
            EventGroup::EspDebA => class == Class::A && language == Language::EspDeb,
            EventGroup::B => class == Class::B,
            EventGroup::B1 => class == Class::B && subgroup == 1,
            EventGroup::B2 => class == Class::B && subgroup == 2,
            EventGroup::AllB => class == Class::B && language == Language::All,
            EventGroup::AllDebB => class == Class::B && language == Language::AllDeb,
            EventGroup::EspB => class == Class::B && language == Language::Esp,
            EventGroup::EspDebB => class == Class::B && language == Language::EspDeb,
            EventGroup::C => class == Class::C,
            EventGroup::C1 => class == Class::C && subgroup == 1,
            EventGroup::C2 => class == Class::C && subgroup == 2,
            EventGroup::AllC => class == Class::C && language == Language::All,
            EventGroup::AllDebC => class == Class::C && language == Language::AllDeb,
            EventGroup::EspC => class == Class::C && language == Language::Esp,
            EventGroup::EspDebC => class == Class::C && language == Language::EspDeb,
            EventGroup::D => class == Class::D,
            EventGroup::D1 => class == Class::D && subgroup == 1,
            EventGroup::D2 => class == Class::D && subgroup == 2,
            EventGroup::AllD => class == Class::D && language == Language::All,
            EventGroup::AllDebD => class == Class::D && language == Language::AllDeb,
            EventGroup::EspD => class == Class::D && language == Language::Esp,
            EventGroup::EspDebD => class == Class::D && language == Language::EspDeb,
            EventGroup::E => class == Class::E,
            EventGroup::E1 => class == Class::E && subgroup == 1,
            EventGroup::E2 => class == Class::E && subgroup == 2,
            EventGroup::AllE => class == Class::E && language == Language::All,
            EventGroup::AllDebE => class == Class::E && language == Language::AllDeb,
            EventGroup::EspE => class == Class::E && language == Language::Esp,
            EventGroup::EspDebE => class == Class::E && language == Language::EspDeb,
            EventGroup::F => class == Class::F,
            EventGroup::F1 => class == Class::F && subgroup == 1,
            EventGroup::F2 => class == Class::F && subgroup == 2,
            EventGroup::AllF => class == Class::F && language == Language::All,
            EventGroup::AllDebF => class == Class::F && language == Language::AllDeb,
            EventGroup::EspF => class == Class::F && language == Language::Esp,
            EventGroup::EspDebF => class == Class::F && language == Language::EspDeb,
            EventGroup::G => class == Class::G,
            EventGroup::G1 => class == Class::G && subgroup == 1,
            EventGroup::G2 => class == Class::G && subgroup == 2,
            EventGroup::AllG => class == Class::G && language == Language::All,
            EventGroup::AllDebG => class == Class::G && language == Language::AllDeb,
            EventGroup::EspG => class == Class::G && language == Language::Esp,
            EventGroup::EspDebG => class == Class::G && language == Language::EspDeb,
            EventGroup::H => class == Class::H,
            EventGroup::H1 => class == Class::H && subgroup == 1,
            EventGroup::H2 => class == Class::H && subgroup == 2,
            EventGroup::AllH => class == Class::H && language == Language::All,
            EventGroup::AllDebH => class == Class::H && language == Language::AllDeb,
            EventGroup::EspH => class == Class::H && language == Language::Esp,
            EventGroup::EspDebH => class == Class::H && language == Language::EspDeb,
            EventGroup::I => class == Class::I,
            EventGroup::I1 => class == Class::I && subgroup == 1,
            EventGroup::I2 => class == Class::I && subgroup == 2,
            EventGroup::AllI => class == Class::I && language == Language::All,
            EventGroup::AllDebI => class == Class::I && language == Language::AllDeb,
            EventGroup::EspI => class == Class::I && language == Language::Esp,
            EventGroup::EspDebI => class == Class::I && language == Language::EspDeb,
            EventGroup::J => class == Class::J,
            EventGroup::J1 => class == Class::J && subgroup == 1,
            EventGroup::J2 => class == Class::J && subgroup == 2,
            EventGroup::AllJ => class == Class::J && language == Language::All,
            EventGroup::AllDebJ => class == Class::J && language == Language::AllDeb,
            EventGroup::EspJ => class == Class::J && language == Language::Esp,
            EventGroup::EspDebJ => class == Class::J && language == Language::EspDeb,
            EventGroup::K => class == Class::K,
            EventGroup::K1 => class == Class::K && subgroup == 1,
            EventGroup::K2 => class == Class::K && subgroup == 2,
            EventGroup::AllK => class == Class::K && language == Language::All,
            EventGroup::AllDebK => class == Class::K && language == Language::AllDeb,
            EventGroup::EspK => class == Class::K && language == Language::Esp,
            EventGroup::EspDebK => class == Class::K && language == Language::EspDeb,
            EventGroup::IJ => class == Class::I || class == Class::J,
            EventGroup::IJ1 => (class == Class::I || class == Class::J) && subgroup == 1,
            EventGroup::IJ2 => (class == Class::I || class == Class::J) && subgroup == 2,
            EventGroup::FleIJ => (class == Class::I || class == Class::J) && language == Language::Fle,
            EventGroup::JK => class == Class::J || class == Class::K,
            EventGroup::JK1 => (class == Class::J || class == Class::K) && subgroup == 1,
            EventGroup::JK2 => (class == Class::J || class == Class::K) && subgroup == 2,
            EventGroup::FleJK => (class == Class::J || class == Class::K) && language == Language::Fle,
            EventGroup::Range { groups, from, to } => {
                for group in groups {
                    if group.matches(name, class, subgroup, language) {
                        return name.to_lowercase() >= from.to_lowercase() && name.to_lowercase() <= to.to_lowercase();
                    }
                }
                false
            },
        }
    }
}
