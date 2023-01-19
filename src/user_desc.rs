use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDesc {
    pub uid: i64,
    pub email: String,
    pub profile_picture: Option<String>,
    pub user_groups: UserGroups,
}

impl UserDesc {
    /// Creates a new UserDesc. (profile_picture is set to None)
    pub fn new(uid: i64, email: String, user_groups: UserGroups) -> Self {
        UserDesc {
            uid,
            email,
            profile_picture: None,
            user_groups,
        }
    }

    /// Returns the username of the user according to the email address.
    /// example: "edouard.foobar@insa-rouen.fr -> "edouard.foobar"
    pub fn get_username(&self) -> String {
        self.email.split("@").next().unwrap().to_string()
    }
}