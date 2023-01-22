use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendRequestIncoming {
    pub from: (UserDesc, UserGroups),
    pub at_ts: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendRequestOutgoing{
    pub to: (UserDesc, UserGroups),
    pub at_ts: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendLists {
    pub friends: Vec<(UserDesc, UserGroups)>,
    pub outgoing: Vec<FriendRequestOutgoing>,
    pub incoming: Vec<FriendRequestIncoming>,
    pub declined: Vec<(UserDesc, UserGroups)>,
}
