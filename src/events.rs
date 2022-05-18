use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", content = "subject")]
pub enum EventKind {
    Td(String),
    Cm(String),
    Tp(String),
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    #[serde(flatten)]
    pub kind: EventKind,
    pub groups: Vec<EventGroup>,
    pub teachers: Vec<String>,
    pub location: Option<crate::location::Location>,
    pub start_unixtime: u64,
    pub end_unixtime: u64,
}
