use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GroupDescriptor {
    Stpi { department: Department, language: Language, class: Class, tp_group: i32 },
    Iti { department: Department, group: i32, language: Language, lv1_group: i32, lv2_group: i32 },
}

impl GroupDescriptor {
    pub fn new_stpi(department: Department, class: Class, language: Language, tp_group: u8) -> Result<Self, &'static str> {
        let group = GroupDescriptor::Stpi {
            department,
            language,
            class,
            tp_group: tp_group as i32,
        };
        group.validate()
    }

    pub fn new_iti(department: Department, group: u8, language: Language, lv1_group: u8, lv2_group: u8) -> Result<Self, &'static str> {
        let group = GroupDescriptor::Iti {
            department,
            group: group as i32,
            language,
            lv1_group: lv1_group as i32,
            lv2_group: lv2_group as i32,
        };
        group.validate()
    }

    pub fn validate(self) -> Result<Self, &'static str> {
        match &self {
            GroupDescriptor::Stpi { tp_group, .. } => {
                if *tp_group != 1 && *tp_group != 2 {
                    return Err("tp_group must be 1 or 2");
                }
            }
            GroupDescriptor::Iti { group, lv1_group, lv2_group, .. } => {
                if !(1i32..=4).contains(group) {
                    return Err("group must be 1, 2, 3 or 4");
                }
                if !(1i32..=4).contains(lv1_group) || !(1i32..=4).contains(lv2_group) {
                    return Err("language_group must be 1, 2, 3 or 4");
                }
            }
        }
        Ok(self)
    }
}

impl std::default::Default for GroupDescriptor {
    fn default() -> Self {
        GroupDescriptor::Stpi {
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

    Department(Department),
    Class(Class),
    Group(u8),
    TpGroup(u8),
    Language(Language),
    Lv1Group(u8),
    Lv2Group(u8),

    And(Vec<EventGroup>),
    Or(Vec<EventGroup>),
}

impl EventGroup {
    pub fn matches(&self, g: &GroupDescriptor, u_email: &str) -> bool {
        match self {
            EventGroup::Everyone => true,
            EventGroup::Department(d) => match g {
                GroupDescriptor::Stpi { department, .. } => department == d,
                GroupDescriptor::Iti { department, .. } => department == d,
            },
            EventGroup::Class(class) => match g {
                GroupDescriptor::Stpi { class: c, .. } => c == class,
                GroupDescriptor::Iti { .. } => false,
            },
            EventGroup::Group(group) => match g {
                GroupDescriptor::Stpi { .. } => false,
                GroupDescriptor::Iti { group: g, .. } => *g == *group as i32,
            },
            EventGroup::TpGroup(tp_group) => match g {
                GroupDescriptor::Stpi { tp_group: t, .. } => *t == *tp_group as i32,
                GroupDescriptor::Iti { .. } => false,
            },
            EventGroup::Language(lang) => match g {
                GroupDescriptor::Stpi { language: l, .. } => l == lang,
                GroupDescriptor::Iti { language: l, .. } => l == lang,
            },
            EventGroup::Lv1Group(lv1_group) => match g {
                GroupDescriptor::Stpi { .. } => false,
                GroupDescriptor::Iti { lv1_group: l, .. } => *l == *lv1_group as i32,
            },
            EventGroup::Lv2Group(lv2_group) => match g {
                GroupDescriptor::Stpi { .. } => false,
                GroupDescriptor::Iti { lv2_group: l, .. } => *l == *lv2_group as i32,
            },
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

    pub fn group(department: Department, group: u8) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Group(group)])
    }

    pub fn class_and_tp_group(department: Department, class: Class, tp_group: u8) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Class(class), EventGroup::TpGroup(tp_group)])
    }

    pub fn class_and_language(department: Department, class: Class, language: Language) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Class(class), EventGroup::Language(language)])
    }

    pub fn grouped_language(department: Department, language: Language, language_group: u8) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Language(language), EventGroup::Lv2Group(language_group)])
    }

    pub fn classes(department: Department, classes: Vec<Class>) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Or(classes.into_iter().map(EventGroup::Class).collect())])
    }

    pub fn groups(department: Department, groups: Vec<u8>) -> Self {
        EventGroup::And(vec![EventGroup::Department(department), EventGroup::Or(groups.into_iter().map(EventGroup::Group).collect())])
    }
}
