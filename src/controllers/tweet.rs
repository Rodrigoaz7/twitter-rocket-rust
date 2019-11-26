
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;
use crate::utils;
use chrono::{DateTime, Utc};

// Buscar um tweet específico
#[get("/tweet/<id>")]
pub fn get(id: String) -> JsonValue {
  
  match utils::validations::validateObjectId(id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
    },
    true => {
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
          return utils::validations::generateErrorJson(_e.to_string(), 404);
        }
      }
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
      return utils::validations::generateErrorJson("Nenhum tweet encontrado".to_string(), 404);
    }
  }
}

// Buscar todos os tweets do usuário
#[get("/tweets/profile/<user_id>")]
pub fn getAllFromUser(user_id: String) -> JsonValue {

  match utils::validations::validateObjectId(user_id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
    },
    true => {
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
          return utils::validations::generateErrorJson(_e.to_string(), 404);
        }
      }
    }
  }
    
}

// Buscar todos os tweets de quem o usuário segue
#[get("/tweets/<user_id>")]
pub fn getAllFromUsersFollowing(user_id: String) -> JsonValue {

  match utils::validations::validateObjectId(user_id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
    },
    true => {
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
          return utils::validations::generateErrorJson(_e.to_string(), 404);
        }
      }
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
          return utils::validations::generateErrorJson(_e.to_string(), 400);
        }
    }
  }

// Dar like em um tweet
#[put("/tweet/like", format = "application/json", data = "<tweet>")]
pub fn like(tweet: Json<meta::tweet::PostLike>) -> JsonValue {

  match utils::validations::validateObjectId(tweet.user_id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id do usuário incorreto".to_string(), 400);
    },
    true => {
      match utils::validations::validateObjectId(tweet.tweet_id.clone()) {
        false => {
          return utils::validations::generateErrorJson("Id do tweet incorreto".to_string(), 400);
        },
        true => {
          let result = models::Tweet::like(tweet.tweet_id.to_owned(), tweet.user_id.to_owned());

          if result {
            return json!({
              "code": 200,
              "success": true,
              "data": {},
              "error": ""
            })
          }

          return utils::validations::generateErrorJson("Um erro ocorreu".to_string(), 400);
        }
      }
    }
  }
  
}

// Retweetar um tweet
#[post("/tweet/retweet", format = "application/json", data = "<tweet>")]
pub fn retweet(tweet: Json<meta::tweet::PostLike>) -> JsonValue {

  match utils::validations::validateObjectId(tweet.user_id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id do usuário incorreto".to_string(), 400);
    },
    true => {
      match utils::validations::validateObjectId(tweet.tweet_id.clone()) {
        false => {
          return utils::validations::generateErrorJson("Id do tweet incorreto".to_string(), 400);
        },
        true => {
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
              return utils::validations::generateErrorJson(_e.to_string(), 400);
            }
          }
        }
      }
    }
  }
  
}
