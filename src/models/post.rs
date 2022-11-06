use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::avatar::Avatar;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub avatar: Avatar,
    pub content: String,
    pub date_published: String,
}
