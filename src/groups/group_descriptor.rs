use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupDescriptor {
    pub(crate) groups: BTreeMap<String, String>,
}

impl GroupDescriptor {
    pub fn new_with_groups(groups: BTreeMap<String, String>) -> GroupDescriptor {
        GroupDescriptor { groups }
    }

    pub fn insert(&mut self, id: String, value: String) {
        self.groups.insert(id, value);
    }

    pub fn matches(&self, filter: &GroupFilter) -> bool {
        match filter {
            GroupFilter::Is { id, value } => {
                if let Some(v) = self.groups.get(id) {
                    return v == value;
                }
                false
            },
            GroupFilter::All(filters) => {
                for filter in filters {
                    if !self.matches(filter) {
                        return false;
                    }
                }
                true
            },
            GroupFilter::Any(filters) => {
                for filter in filters {
                    if self.matches(filter) {
                        return true;
                    }
                }
                false
            },
        }
    }

    pub fn groups(&self) -> &BTreeMap<String, String> {
        &self.groups
    }

    pub fn validate(&self, groups: &[Group]) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        for group in groups {
            if group.required_if.as_ref().map(|ri| self.matches(ri)).unwrap_or(true) && !self.groups.contains_key(&group.id) {
                issues.push(ValidationIssue::MissingRequiredGroup { group: group.id.clone() });
            }
            if let Some(value) = self.groups.get(&group.id) {
                if !group.values.iter().any(|(v,_)| v==value) {
                    issues.push(ValidationIssue::InvalidValue { group: group.id.clone(), value: value.clone() });
                }
            }
        }
        for group in self.groups.keys() {
            if !groups.iter().any(|g| &g.id == group) {
                issues.push(ValidationIssue::UnknownGroup { group: group.clone() });
            }
        }
        
        issues
    }
}

impl Default for GroupDescriptor {
    fn default() -> Self {
        GroupDescriptor {
            groups: [
                (String::from("school"), String::from("insa-rouen")),
                (String::from("insa-rouen:department"), String::from("STPI1")),
                (String::from("insa-rouen:language"), String::from("ESP")),
                (String::from("insa-rouen:stpi:class"), String::from("A")),
                (String::from("insa-rouen:stpi:tp-group"), String::from("1")),
            ].iter().cloned().collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ValidationIssue {
    MissingRequiredGroup { group: String },
    InvalidValue { group: String, value: String },
    UnknownGroup { group: String },
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationIssue::MissingRequiredGroup { group } => write!(f, "Missing required group {}", group),
            ValidationIssue::InvalidValue { group, value } => write!(f, "Invalid value for group {}: {}", group, value),
            ValidationIssue::UnknownGroup { group } => write!(f, "Unknown group {}", group),
        }
    }
}
