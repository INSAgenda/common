use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendRequestIncoming {
    pub from: i64,
    pub date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FriendRequestOutgoing{
    pub to: i64,
    pub date: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendsLists {
    pub friends_list: Vec<i64>,
    pub friend_requests_outgoing: Vec<FriendRequestOutgoing>,
    pub friend_requests_incoming: Vec<FriendRequestIncoming>,
    pub friend_requests_declined: Vec<i64>,
}