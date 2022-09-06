use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdContentType {
    Text,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdRecord {
    pub title: String,
    pub id: String,
    pub start_ts: u64,
    pub end_ts: u64,
    pub target: Option<GroupDescriptor>,
    pub ty: AdContentType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
