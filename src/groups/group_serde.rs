use crate::prelude::*;
use serde::{Serializer, Deserializer, de::Visitor};

struct StringVisitor;
impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a formatted string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(value.to_string())
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(value)
    }
}

impl GroupFilter {
    pub fn format_to_string(&self) -> String {
        match self {
            GroupFilter::Is { id, value } => format!("{}={}", id, value),
            GroupFilter::All(filters) => {
                let mut s = String::new();
                s.push('(');
                for filter in filters {
                    s.push_str(&filter.format_to_string());
                    s.push_str(" AND ");
                }
                if s.len() > 1 {
                    s.truncate(s.len() - 5);
                }
                s.push(')');
                s
            },
            GroupFilter::Any(filters) => {
                let mut s = String::new();
                s.push('(');
                for filter in filters {
                    s.push_str(&filter.format_to_string());
                    s.push_str(" OR ");
                }
                if s.len() > 1 {
                    s.truncate(s.len() - 4);
                }
                s.push(')');
                s
            },
        }
    }
}

impl UserGroups {
    pub fn format_to_string(&self) -> String {
        self.groups.iter().cloned().collect::<Vec<_>>().join("+")
    }

    pub fn read_from_string(s: &str) -> Result<UserGroups, String> {
        let mut groups = HashSet::new();
        for part in s.trim().split('+') {
            if part.is_empty() {
                continue;
            }
            groups.insert(part.to_string());
        }
        Ok(UserGroups { groups })
    }
}

impl Serialize for GroupFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let formatted = self.format_to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for GroupFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut formatted = deserializer.deserialize_string(StringVisitor)?;
        formatted = format!("({formatted})");
        read_whole_as_filter(&formatted).map_err(|(s, e)| {
            serde::de::Error::custom(format!("Error while parsing group filter {formatted:?} at char {}: {e}", formatted.len() - s.len()))
        })
    }
}

impl Serialize for UserGroups {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let formatted = self.format_to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for UserGroups {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let formatted = deserializer.deserialize_string(StringVisitor)?;
        UserGroups::read_from_string(&formatted).map_err(|e| {
            serde::de::Error::custom(format!("Error while parsing group descriptor {formatted:?}: {e}"))
        })
    }
}