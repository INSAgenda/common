use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDesc {
    pub uid: i64,
    pub email: String,
    pub picture: Option<String>,
}

impl UserDesc {
    /// Creates a new UserDesc. (profile_picture is set to None)
    pub fn new(uid: i64, email: String) -> Self {
        UserDesc {
            uid,
            email,
            picture: None,
        }
    }

    /// Returns the username of the user based on the email address.
    /// 
    /// Example: "edouard.foobar@insa-rouen.fr" -> "edouard.foobar"
    pub fn get_username(&self) -> String {
        self.email.split('@').next().unwrap().to_string()
    }

    pub fn get_mastodon_username(&self) -> String {
        self.get_username().replace(".", "_")
    }
}
