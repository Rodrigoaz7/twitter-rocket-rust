use bson;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;
use crate::utils;

// Retorna dados do usuário
#[get("/user/<id>")]
pub fn get(id: String) -> JsonValue {

  match utils::validations::validateObjectId(id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
    },
    true => {
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
          return utils::validations::generateErrorJson(_e.to_string(), 404);
        }
      }
    }
  }

}

// Retorna todos os usuários cadastrados
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
      return utils::validations::generateErrorJson(_e.to_string(), 404);
    }
  }
}

// Cria um usuário
#[post("/user", format = "application/json", data = "<user>")]
pub fn insert(user: Json<meta::user::Post>) -> JsonValue {

  let model = models::User::Model {
    email: user.email.to_owned(),
    name: user.name.to_owned()
  };

  let document = model.insert().unwrap();
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
      return utils::validations::generateErrorJson(_e.to_string(), 400);
    }
  }
}

// Segue um usuário
#[put("/user/follow", format = "application/json", data = "<follow>")]
pub fn follow(follow: Json<meta::user::FollowRequest>) -> JsonValue {

  match utils::validations::validateObjectId(follow.user_id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
    },
    true => {
      match utils::validations::validateObjectId(follow.user_to_follow_id.clone()) {
        false => {
          return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
        },
        true => {
          models::User::follow(follow.user_id.to_owned(), follow.user_to_follow_id.to_owned());
          return json!({
            "code": 200,
            "success": true,
            "data": {},
            "error": ""
          });
        }
      }
    }
  }

}
