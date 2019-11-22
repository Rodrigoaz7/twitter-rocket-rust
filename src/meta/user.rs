use bson; 

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
  pub _id: bson::oid::ObjectId,
  pub name: String,
  pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
  pub _id: bson::oid::ObjectId,
  pub name: String,
  pub email: String,
  pub following: bson::Array,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowRequest {
  pub user_id: String,
  pub user_to_follow_id: String,
}