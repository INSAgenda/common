use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    /// The count of api keys
    pub api_key_count: u64,
    /// Last password modification timestamp.
    /// Can be `None` if the user has no password or if the user has never changed his password since the addition of the tracking feature.
    pub last_password_mod: Option<i64>,
    /// The email associated with its verification state
    pub email: (String, bool),
    /// Which group the user is in
    pub group_desc: GroupDescriptor,
    /// Last colors modification timestamp.
    pub last_colors_mod: i64,
    /// has password
    #[serde(default)]
    pub has_password: bool,
}