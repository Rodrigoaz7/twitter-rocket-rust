use bson;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;

#[get("/user/<id>")]
pub fn get(id: String) -> JsonValue {

  let document = models::User::find_one(id.to_owned()).unwrap();
  let result = bson::from_bson::<meta::user::GetResponse>(bson::Bson::Document(document.unwrap()));

  match result {
    Ok(user) => {
      json!({
        "code": 200,
        "success": true,
        "data": user,
        "error": ""
      })
    },
    Err (_e) => {
      json!({
        "code": 400,
        "success": false,
        "data": {},
        "error": "An error has occured"
      })
    }
  }

}


#[get("/users")]
pub fn getAll() -> JsonValue {

  match models::User::find() {
    Ok(users) => {
      json!({
        "code": 200,
        "success": true,
        "data": users,
        "error": ""
      })
    },
    Err (_e) => {
      json!({
        "code": 400,
        "success": false,
        "data": {},
        "error": "An error has occured"
      })
    }
  }
}

#[post("/user", format = "application/json", data = "<user>")]
pub fn insert(user: Json<meta::user::Post>) -> JsonValue {

  let model = models::User::Model {
    email: user.email.to_owned(),
    name: user.name.to_owned()
  };

  let document = model.insert().unwrap();

  println!("inseri o model, ");

  let result = bson::from_bson::<meta::user::PostResponse>(bson::Bson::Document(document.unwrap()));

  match result {
    Ok(user) => {
      json!({
        "code": 201,
        "success": true,
        "data": user,
        "error": ""
      })
    },
    Err (_e) => {
      json!({
        "code": 412,
        "success": false,
        "data": {},
        "error": "An error has occured"
      })
    }
  }
}

#[put("/user/follow", format = "application/json", data = "<follow>")]
pub fn follow(follow: Json<meta::user::FollowRequest>) -> JsonValue {

  let result = models::User::follow(follow.user_id.to_owned(), follow.user_to_follow_id.to_owned());

  json!({
    "code": 200,
    "success": true,
    "data": {},
    "error": ""
  })

}
