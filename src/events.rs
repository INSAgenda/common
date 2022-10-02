use crate::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    Td,
    Cm,
    Tp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub summary: String,
    pub kind: Option<EventKind>,
    pub number: Option<u8>,
    pub teachers: Vec<String>,
    pub group: GroupFilter,
    pub location: Option<Location>,
    pub start_unixtime: u64,
    pub end_unixtime: u64,
}
