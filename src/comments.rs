use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Comment {
    /// Random number identifying the comment.
    pub cid: u64,
    /// Id of the parent comment, if any.
    pub parent: Option<u64>,
    /// Author of the comment.
    pub author: UserDesc,
    /// Content of the comment.
    /// Markdown isn't supported but will be eventually.
    pub content: String,
    /// Timestamp of the comment creation.
    pub creation_ts: i64,
    /// Equal to `creation_ts` if the comment has never been edited.
    pub last_edited_ts: i64,
    /// Number of upvotes minus number of downvotes.
    pub score: i64,
    /// The vote of the current user.
    /// -1, 0 or 1.
    pub vote: i8,
}
