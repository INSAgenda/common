use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Groups {
    pub(crate) groups: HashSet<String>,
}

impl Groups {
    pub fn new_with_groups(groups: Vec<String>) -> Groups {
        Groups { groups: groups.into_iter().collect() }
    }

    pub fn insert(&mut self, group: String) {
        self.groups.insert(group);
    }

    pub fn remove(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn matches(&self, group: String) -> bool {
        self.groups.contains(&group)
    }

    pub fn matches_with_name(&self, group: String, _name: Option<&str>) -> bool {
        self.matches(group)
    }

    pub fn groups(&self) -> &HashSet<String> {
        &self.groups
    }
}

impl Default for Groups {
    fn default() -> Self {
        Groups {
            groups: [
                "iti3", "stpi2-precedent" , "stpi22-p9-td-01" , "ad-etudiants", "h-22-ang-cult-stpi-gg-01", "h-22-ang-cult-stpi-pg-02", "etudiants", "h-22-com-stpi-08", "stpi21-tp-a1", "stpi22-i4-td-01", "stpi21-all-td-a-j-k", "stpi22-i3-td-01"
            ].iter().map(|g| g.to_string()).collect(),
        }
    }
}

impl From<Vec<String>> for Groups {
    fn from(groups: Vec<String>) -> Self {
        Groups { groups: groups.into_iter().collect() }
    }
}

impl Serialize for Groups {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut groups = self.groups.iter().collect::<Vec<_>>();
        groups.sort();
        groups.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Groups {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let groups = Vec::<String>::deserialize(deserializer)?;
        Ok(Groups { groups: groups.into_iter().collect() })
    }
}
