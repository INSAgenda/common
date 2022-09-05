use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdRecord {
    pub title: String,
    pub id: String,
    pub start_ts: u64,
    pub end_ts: u64,
    pub target: Option<GroupDescriptor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
}
