use bson; 
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
  pub _id: bson::oid::ObjectId,
  pub user_id: bson::oid::ObjectId,
  pub text: String,
  pub date_created: String
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub text: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostLike {
    pub tweet_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
  pub _id: String,
  pub user_id: String,
  pub text: String,
  pub likes: bson::Array,
  pub date_created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponseForUpdate {
  pub _id: bson::oid::ObjectId,
  pub likes: bson::Array,

}