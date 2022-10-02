use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupDesc {
    pub id: String,
    pub name: (String, String), // (french name, english name)
    pub help: (String, String), // (french help message, english help message)
    pub values: Vec<(String, (String, String))>, // (value, (french label, english label))

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_if: Option<GroupFilter>,
}
