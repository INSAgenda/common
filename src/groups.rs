use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct GroupDescriptor {
    pub department: Department,
    pub language: Language,
    pub class: Class,
    pub tp_group: i32,
}

impl GroupDescriptor {
    pub fn new(department: Department, class: Class, language: Language, tp_group: u8) -> Result<Self, &'static str> {
        if tp_group != 1 && tp_group != 2 {
            return Err("tp_group must be 1 or 2");
        }
        Ok(Self {
            department,
            language,
            class,
            tp_group: tp_group as i32,
        })
    }
}

impl std::default::Default for GroupDescriptor {
    fn default() -> Self {
        Self {
            department: Department::Stpi1,
            language: Language::All,
            class: Class::E,
            tp_group: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventGroup {
    Everyone,
    S1,
    S2,
    Sib,

    Department(Department),
    Class(Class),
    TpGroup(u8),
    Language(Language),

    And(Vec<EventGroup>),
    Or(Vec<EventGroup>),
}

impl EventGroup {
    pub fn matches(&self, g: &GroupDescriptor, u_email: &str) -> bool {
        match self {
            EventGroup::Everyone => true,
            EventGroup::S1 => [Class::A, Class::B, Class::C, Class::D].contains(&g.class),
            EventGroup::S2 => [Class::E, Class::F, Class::G, Class::H].contains(&g.class),
            EventGroup::Sib => [Class::I, Class::J, Class::K].contains(&g.class),
            EventGroup::Department(p) => p == &g.department,
            EventGroup::Class(class) => class == &g.class,
            EventGroup::TpGroup(tp_group) => *tp_group as i32 == g.tp_group,
            EventGroup::Language(lang) => lang == &g.language,
            EventGroup::And(groups) => {
                for group in groups {
                    if !group.matches(g, u_email) {
                        return false;
                    }
                }
                true
            },
            EventGroup::Or(groups) => {
                for group in groups {
                    if group.matches(g, u_email) {
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
    pub fn class(department: Department, class: Class) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Class(class)])
    }

    pub fn class_and_tp_group(department: Department, class: Class, tp_group: u8) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Class(class), EventGroup::TpGroup(tp_group)])
    }

    pub fn class_and_language(department: Department, class: Class, language: Language) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Class(class), EventGroup::Language(language)])
    }

    pub fn classes(department: Department, classes: Vec<Class>) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Or(classes.into_iter().map(EventGroup::Class).collect())])
    }
}
