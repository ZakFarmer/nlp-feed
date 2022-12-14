use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Avatar {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub description: String,
    pub location: String,
    pub keywords: String,
    pub repetition_penalty: f64,
    pub temperature: f64,
    pub top_p: f64,
}
