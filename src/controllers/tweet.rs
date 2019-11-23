use bson;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;
use chrono::{DateTime, Utc};
use rocket::request::{Form, LenientForm};
use std::collections::HashMap;

// Buscar um tweet específico
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

// Buscar todos os tweets do banco
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

// Buscar todos os tweets do usuário
#[get("/tweets/profile/<user_id>")]
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

// Buscar todos os tweets de quem o usuário segue
#[get("/tweets/<user_id>")]
pub fn getAllFromUsersFollowing(user_id: String) -> JsonValue {

    match models::Tweet::findFollowingsByUser(user_id) {
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

// Cria um novo tweet
#[post("/tweet", format = "application/json", data = "<tweet>")]
pub fn insert(tweet: Json<meta::tweet::Post>) -> JsonValue {

    let hoje: DateTime<Utc> = Utc::now();  
    let model = models::Tweet::Model {
        text: tweet.text.to_owned(),
        user_id: tweet.user_id.to_owned(),
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

// Dar like em um tweet
#[put("/tweet/like", format = "application/json", data = "<tweet>")]
pub fn like(tweet: Json<meta::tweet::PostLike>) -> JsonValue {
  let result = models::Tweet::like(tweet.tweet_id.to_owned(), tweet.user_id.to_owned());

  if result {
    return json!({
      "code": 200,
      "success": true,
      "data": {},
      "error": ""
    })
  }

  return json!({
    "code": 400,
    "success": false,
    "data": {},
    "error": "Erro ao dar o like"
  })
  
}

// Retweetar um tweet
#[post("/tweet/retweet", format = "application/json", data = "<tweet>")]
pub fn retweet(tweet: Json<meta::tweet::PostLike>) -> JsonValue {

  match models::Tweet::retweet(tweet.tweet_id.to_owned(), tweet.user_id.to_owned()) {
    Ok(ret) => {
      json!({
        "code": 200,
        "success": true,
        "data": ret,
        "error": ""
        })
    },
    Err(_e) => {
      json!({
        "code": 400,
        "success": false,
        "data": {},
        "error": _e.to_string()
      })
    }
  }
  
}
