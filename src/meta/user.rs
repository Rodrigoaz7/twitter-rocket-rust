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
}