use crate::prelude::*;
use serde::{Serializer, Deserializer, de::Visitor};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub help: String,
    pub values: Vec<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_if: Option<GroupFilter>,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupDescriptor {
    groups: BTreeMap<String, String>,
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

    pub fn format_to_string(&self) -> String {
        self.groups.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("+")
    }

    // school=insa-rouen+insa-rouen:department=STPI1+insa-rouen:language=ESP+insa-rouen:stpi:class=A+insa-rouen:stpi:tp-group=1
    // school=insa-rouen+insa-rouen:department=ITI3+insa-rouen:language=ESP+insa-rouen:iti:group=1+insa-rouen:lv1-group=1+insa-rouen:lv2-group=1
    pub fn read_from_string(s: &str) -> Result<GroupDescriptor, String> {
        let mut groups = BTreeMap::new();
        for part in s.trim().split('+') {
            if part.is_empty() {
                continue;
            }
            let mut parts = part.split('=');
            let key = parts.next().ok_or_else(|| format!("invalid group descriptor (missing key): {s}"))?;
            let value = parts.next().ok_or_else(|| format!("invalid group descriptor (missing value): {s}"))?;
            if parts.next().is_some() {
                return Err(format!("invalid group descriptor (too many parts): {s}"));
            }
            if key.is_empty() || key.chars().any(|c| !c.is_ascii_alphanumeric() && c != ':' && c != '_' && c != '-') {
                return Err(format!("invalid group descriptor (invalid key): {key:?}"));
            }
            if value.is_empty() {
                return Err(format!("invalid group descriptor (invalid value): {value:?}"));
            }
            groups.insert(key.to_string(), value.to_string());
        }
        Ok(GroupDescriptor { groups })
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupFilter {
    Is { id: String, value: String },
    All(Vec<GroupFilter>),
    Any(Vec<GroupFilter>),
}

mod parsing {
    use super::*;

    // Trim whitespaces at the beginning of the string
    fn skip_whitespaces(s: &str) -> &str {
        s.trim_start()
    }
    
    // Trim an expected prefix at the beginning of the string
    fn expect<'a, 'b>(s: &'a str, expected: &'b str) -> Result<&'a str, (&'a str, String)> {
        if s.starts_with(expected) {
            Ok(&s[expected.len()..])
        } else {
            Err((s, format!("Expected {expected:?}.")))
        }
    }
    
    // Read an identifier at the beginning of the string
    fn read_identifier(s: &str) -> Result<(&str, String), (&str, String)> {
        let mut i = 0;
        for c in s.chars() {
            if !c.is_ascii_alphanumeric() && c != ':' && c != '_' && c != '-' {
                break;
            }
            i += 1;
        }
        if i == 0 {
            return Err((s, String::from("Expected non-empty identifier.")));
        }
        Ok((&s[i..], s[..i].to_string()))
    }
    
    // Read a simple filter `name=value` at the beginning of the string
    fn read_simple_filter(s: &str) -> Result<(&str, GroupFilter), (&str, String)> {
        let s = skip_whitespaces(s);
        let (s, id) = read_identifier(s)?;
        let s = skip_whitespaces(s);
        let s = expect(s, "=")?;
        let s = skip_whitespaces(s);
        let (s, value) = read_identifier(s)?;
        Ok((s, GroupFilter::Is { id, value }))
    }
    
    // Read a composite filter `(name=value AND name=value)` at the beginning of the string
    fn read_composite_filter(s: &str) -> Result<(&str, GroupFilter), (&str, String)> {
        let s = skip_whitespaces(s);
        let mut s = expect(s, "(")?;
        let mut is_or = false;
        let mut filters = Vec::new();
        loop {
            if s.starts_with(')') {
                break;
            }
            let (ns, filter) = read_filter(s)?;
            filters.push(filter);
            let ns = skip_whitespaces(ns);
            s = ns;
            if s.starts_with(')') {
                break;
            }
            if s.starts_with("OR ") || s.starts_with("or ") {
                if filters.len() == 1 {
                    is_or = true;
                } else if !is_or {
                    return Err((s, String::from("OR filter mixed with AND filter.")));
                }
                s = &s[3..];
                continue;
            }
            if s.starts_with("AND ") || s.starts_with("and ") {
                if filters.len() == 1 {
                    is_or = false;
                } else if is_or {
                    return Err((s, String::from("AND filter mixed with OR filter.")));
                }
                s = &s[4..];
                continue;
            }
            return Err((s, format!("Expected OR, AND or a closing parenthesis at the end of a composite filter. Got {s:?}")));
        }
        let s = expect(s, ")")?;
        
        if filters.len() == 1 {
            return Ok((s, filters.pop().unwrap()));
        }
        match is_or {
            true => Ok((s, GroupFilter::Any(filters))),
            false => Ok((s, GroupFilter::All(filters))),
        }
    }
    
    // Read a filter, either simple or composite, at the beginning of the string
    fn read_filter(s: &str) -> Result<(&str, GroupFilter), (&str, String)> {
        let (s1, e1) = match read_simple_filter(s) {
            Ok((s, filter)) => return Ok((s, filter)),
            Err(e) => e,
        };
        let p1 = s.len() - s1.len();
        let (s2, e2) = match read_composite_filter(s) {
            Ok((s, filter)) => return Ok((s, filter)),
            Err(e) => e,
        };
        let p2 = s.len() - s2.len();
        Err((s, format!("Expected a simple or a composite filter. Both result in reading errors. Simple filter read, at char {p1}: {e1:?}, Composite filter read, at char {p2}: {e2:?}")))
    }

    // Read the whole string as a filter
    pub(super) fn read_whole_as_filter(s: &str) -> Result<GroupFilter, (&str, String)> {
        let (s, filter) = read_filter(s)?;
        let s = skip_whitespaces(s);
        if s.is_empty() {
            Ok(filter)
        } else {
            Err((s, format!("Expected end of string. Got something after the filter {s:?}")))
        }
    }

    #[cfg(test)]
    #[test]
    fn test() {
        read_whole_as_filter("test=value").unwrap();
        read_whole_as_filter("()").unwrap();
        read_whole_as_filter("(test=value AND other=value)").unwrap();
        read_whole_as_filter("(test=value OR other=value)").unwrap();
        read_whole_as_filter("(test=value OR (test=value AND other=value))").unwrap();
        read_whole_as_filter("(test=value   OR ( test=value   AND  other=value))").unwrap();

        read_whole_as_filter("(tést=value OR (test=value AND other=value))").unwrap_err();
        read_whole_as_filter("(test=value OR test=value AND other=value)").unwrap_err();
    }
}

impl<T: ToString, U: ToString> From<(T, U)> for GroupFilter {
    fn from((id, value): (T, U)) -> Self {
        GroupFilter::Is {
            id: id.to_string(),
            value: value.to_string(),
        }
    }
}

impl GroupFilter {
    pub fn is(id: impl ToString, value: impl ToString) -> Self {
        GroupFilter::Is {
            id: id.to_string(),
            value: value.to_string(),
        }
    }

    pub fn both(left: impl Into<Self>, right: impl Into<Self>) -> Self {
        GroupFilter::All(vec![left.into(), right.into()])
    }

    pub fn three(left: impl Into<Self>, middle: impl Into<Self>, right: impl Into<Self>) -> Self {
        GroupFilter::All(vec![left.into(), middle.into(), right.into()])
    }

    pub fn either(left: impl Into<Self>, right: impl Into<Self>) -> Self {
        GroupFilter::Any(vec![left.into(), right.into()])
    }

    pub fn any<T: Into<GroupFilter>>(filters: Vec<T>) -> Self {
        GroupFilter::Any(filters.into_iter().map(|f| f.into()).collect())
    }

    pub fn any_values(id: impl ToString, values: Vec<impl ToString>) -> Self {
        GroupFilter::Any(
            values
                .into_iter()
                .map(|v| GroupFilter::Is {
                    id: id.to_string(),
                    value: v.to_string(),
                })
                .collect(),
        )
    }

    fn format_to_string(&self) -> String {
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
        parsing::read_whole_as_filter(&formatted).map_err(|(s, e)| {
            serde::de::Error::custom(format!("Error while parsing group filter {formatted:?} at char {}: {e}", formatted.len() - s.len()))
        })
    }
}

impl Serialize for GroupDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let formatted = self.format_to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for GroupDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let formatted = deserializer.deserialize_string(StringVisitor)?;
        GroupDescriptor::read_from_string(&formatted).map_err(|e| {
            serde::de::Error::custom(format!("Error while parsing group descriptor {formatted:?}: {e}"))
        })
    }
}
