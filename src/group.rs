use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub help: String,
    pub values: Vec<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_if: Option<GroupFilter>,
}
