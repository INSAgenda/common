use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendRequestIncoming {
    pub from: (UserDesc, UserGroups),
    pub date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendRequestOutgoing{
    pub to: (UserDesc, UserGroups),
    pub date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendsLists {
    pub friends_list: Vec<(UserDesc, UserGroups)>,
    pub friend_requests_outgoing: Vec<FriendRequestOutgoing>,
    pub friend_requests_incoming: Vec<FriendRequestIncoming>,
    pub friend_requests_declined: Vec<(UserDesc, UserGroups)>,
}