use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
    S1,
    S2,
    Sib,

    Promotion(Promotion),
    Class(Class),
    ClassAndTpGroup(Class, u8),
    Lang(Language),

    And(Vec<EventGroup>),
    Or(Vec<EventGroup>),
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

impl Class {
    pub fn list() -> &'static [Class] {
        &[Class::A, Class::B, Class::C, Class::D, Class::E, Class::F, Class::G, Class::H, Class::I, Class::J, Class::K]
    }
}

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

impl EventGroup {
    pub fn matches(&self, u_promotion: Promotion, u_class: Class, u_tp_group: u8, u_lang: Language, u_email: &str) -> bool {
        match self {
            EventGroup::Everyone => true,
            EventGroup::S1 => [Class::A, Class::B, Class::C, Class::D].contains(&u_class),
            EventGroup::S2 => [Class::E, Class::F, Class::G, Class::H].contains(&u_class),
            EventGroup::Sib => [Class::I, Class::J, Class::K].contains(&u_class),
            EventGroup::Promotion(p) => p == &u_promotion,
            EventGroup::Class(class) => class == &u_class,
            EventGroup::ClassAndTpGroup(class, tp_group) => class == &u_class && tp_group == &u_tp_group,
            EventGroup::Lang(lang) => lang == &u_lang,
            EventGroup::And(groups) => {
                for group in groups {
                    if !group.matches(u_promotion, u_class, u_tp_group, u_lang, u_email) {
                        return false;
                    }
                }
                true
            },
            EventGroup::Or(groups) => {
                for group in groups {
                    if group.matches(u_promotion, u_class, u_tp_group, u_lang, u_email) {
                        return true;
                    }
                }
                false
            }
        }
    }
}
