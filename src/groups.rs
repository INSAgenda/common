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
    TpGroup(u8),
    Language(Language),

    And(Vec<EventGroup>),
    Or(Vec<EventGroup>),
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
            EventGroup::TpGroup(tp_group) => tp_group == &u_tp_group,
            EventGroup::Language(lang) => lang == &u_lang,
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

// Helper shortcut functions
impl EventGroup {
    pub fn class(promotion: Promotion, class: Class) -> Self {
        EventGroup::And(vec![EventGroup::Promotion(promotion), EventGroup::Class(class)])
    }

    pub fn class_and_tp_group(promotion: Promotion, class: Class, tp_group: u8) -> Self {
        EventGroup::And(vec![EventGroup::Promotion(promotion), EventGroup::Class(class), EventGroup::TpGroup(tp_group)])
    }

    pub fn class_and_language(promotion: Promotion, class: Class, language: Language) -> Self {
        EventGroup::And(vec![EventGroup::Promotion(promotion), EventGroup::Class(class), EventGroup::Language(language)])
    }

    pub fn classes(promotion: Promotion, classes: Vec<Class>) -> Self {
        EventGroup::And(vec![EventGroup::Promotion(promotion), EventGroup::Or(classes.into_iter().map(EventGroup::Class).collect())])
    }
}
