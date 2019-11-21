use bson;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;
use chrono::{DateTime, Utc};
use rocket::request::{Form, LenientForm};
use std::collections::HashMap;

// #[derive(FromForm)]
// pub struct Filtros {
//     text: Option<String>,
//     user_id: Option<String>
// }

#[get("/tweet/<id>")]
pub fn get(id: String) -> JsonValue {

  match models::Tweet::find_one(id.to_owned()) {
    Ok(tweet) => {
      json!({
        "code": 200,
        "success": true,
        "data": tweet,
        "error": ""
      })
    },
    Err (_e) => {
      json!({
        "code": 400,
        "success": false,
        "data": {},
        "error": _e.to_string()
      })
    }
  }

}


#[get("/tweets")]
pub fn getAll() -> JsonValue {

  match models::Tweet::find() {
    Ok(tweets) => {
      json!({
        "code": 200,
        "success": true,
        "data": tweets,
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

#[get("/tweets/<user_id>")]
pub fn getAllFromUser(user_id: String) -> JsonValue {

    match models::Tweet::findByUser(user_id) {
        Ok(tweets) => {
            json!({
            "code": 200,
            "success": true,
            "data": tweets,
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

#[post("/tweet", format = "application/json", data = "<tweet>")]
pub fn insert(tweet: Json<meta::tweet::Post>) -> JsonValue {

  let user = models::User::find_one(tweet.user_id.to_owned()).unwrap();
  let result_user = bson::from_bson::<meta::user::GetResponse>(bson::Bson::Document(user.unwrap()));

  match result_user {
    Ok(u) => {

        let hoje: DateTime<Utc> = Utc::now();  
        let model = models::Tweet::Model {
            text: tweet.text.to_owned(),
            user_id: u._id.to_owned().to_string(),
            date_created: hoje.to_owned().to_string()
        };

        match model.insert() {
            Ok(tweet) => {
                json!({
                    "code": 201,
                    "success": true,
                    "data": tweet,
                    "error": ""
                })
            },
            Err (_e) => {
                json!({
                    "code": 412,
                    "success": false,
                    "data": {},
                    "error": _e.to_string()
                })
            }
        }
    }
    Err (_e) => {
        json!({
            "code": 412,
            "success": false,
            "data": {},
            "error": "Usuário não existe"
        })
    }
  }

}
