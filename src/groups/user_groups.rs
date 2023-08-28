use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserGroups {
    pub(crate) groups: HashSet<String>,
}

impl UserGroups {
    pub fn new_with_groups(groups: Vec<String>) -> UserGroups {
        UserGroups { groups: groups.into_iter().collect() }
    }

    pub fn insert(&mut self, group: String) {
        self.groups.insert(group);
    }

    pub fn remove(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn matches(&self, filter: &GroupFilter) -> bool {
        self.matches_with_name(filter, None)
    }

    pub fn matches_with_name(&self, filter: &GroupFilter, name: Option<&str>) -> bool {
        match filter {
            GroupFilter::Is { id, value } => {
                if let Some(name) = name {
                    if id == "name-after" {
                        return name >= value;
                    }
                    if id == "name-before" {
                        return name <= value;
                    }
                }
                if let Some(v) = self.groups.get(id) {
                    return v == value;
                }
                false
            },
            GroupFilter::All(filters) => {
                for filter in filters {
                    if !self.matches_with_name(filter, name) {
                        return false;
                    }
                }
                true
            },
            GroupFilter::Any(filters) => {
                for filter in filters {
                    if self.matches_with_name(filter, name) {
                        return true;
                    }
                }
                false
            },
        }
    }

    pub fn groups(&self) -> &HashSet<String> {
        &self.groups
    }
}

impl Default for UserGroups {
    fn default() -> Self {
        UserGroups {
            groups: [
                "iti3", "stpi2-precedent" , "stpi22-p9-td-01" , "ad-etudiants", "h-22-ang-cult-stpi-gg-01", "h-22-ang-cult-stpi-pg-02", "etudiants", "h-22-com-stpi-08", "stpi21-tp-a1", "stpi22-i4-td-01", "stpi21-all-td-a-j-k", "stpi22-i3-td-01"
            ].iter().map(|g| g.to_string()).collect(),
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
