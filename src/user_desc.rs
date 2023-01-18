use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDesc {
    pub uid: i64,
    pub email: String,
    pub profile_picture: Option<String>,
}

impl UserDesc {
    /// Creates a new UserDesc. (profile_picture is set to None)
    pub fn new(uid: i64, email: String) -> Self {
        UserDesc {
            uid,
            email,
            profile_picture: None,
        }
    }

    /// Returns the username of the user according to the email address.
    /// example: "edouard.foobar@insa-rouen.fr -> "edouard.foobar"
    pub fn get_username(&self) -> String {
        self.email.split("@").next().unwrap().to_string()
    }
}