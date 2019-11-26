use bson;
use bson::oid::ObjectId;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::{doc, error::Error};
use mongodb;
use chrono::{DateTime, Utc};
extern crate serde_json;

use crate::lib;
use crate::meta;

#[derive(Debug)]
pub struct Model {
  pub text: String,
  pub user_id: String,
  pub date_created: String
}

#[derive(Debug)]
pub struct RetweetModel {
  pub text: String,
  pub user_id: String,
  pub date_created: String,
  pub retweet_from: String
}

impl RetweetModel {
    pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
      
        doc! { 
          "text": self.text.to_owned(),
          "user_id": ObjectId::with_string(&self.user_id).unwrap().to_owned(),
          "date_created": self.date_created.to_owned(),
          "retweet_from": ObjectId::with_string(&self.retweet_from).unwrap().to_owned()
        }
      }
      
      pub fn insert(&self) -> Result<Option<bson::ordered::OrderedDocument>, Error> {
          let client = lib::mongo::establish_connection();
          let collection = client.db("twitter").collection("tweet");
  
          let r = collection.insert_one(self.to_bson().clone(), None).ok().expect("Failed to execute find.");
          
          let result = collection.find_one(Some(doc! { "_id" => r.inserted_id.unwrap() }), None)
          .ok().expect("Failed to execute find.");
  
          Ok(result)
  
      }
}

impl Model {
    pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
      
      doc! { 
        "text": self.text.to_owned(),
        "user_id": ObjectId::with_string(&self.user_id).unwrap().to_owned(),
        "date_created": self.date_created.to_owned()
      }
    }
    
    pub fn insert(&self) -> Result<Option<bson::ordered::OrderedDocument>, Error> {
        let client = lib::mongo::establish_connection();
        let collection = client.db("twitter").collection("tweet");

        let r = collection.insert_one(self.to_bson().clone(), None).ok().expect("Failed to execute find.");
        
        let result = collection.find_one(Some(doc! { "_id" => r.inserted_id.unwrap() }), None)
        .ok().expect("Failed to execute find.");

        Ok(result)

    }
}
  
pub fn find_one(tweet_id: String) -> Result<Option<bson::ordered::OrderedDocument>, Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");
    let id = ObjectId::with_string(&tweet_id).unwrap();

    let result = collection.find_one(Some(doc! { "_id" : id }), None)
        .ok().expect("Failed to execute find.");

    Ok(result)

}

pub fn find() -> Result<Vec<bson::ordered::OrderedDocument>, Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");

    let response_document = collection.find(None, None).unwrap();

    response_document
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<bson::ordered::OrderedDocument>, Error>>()

}

pub fn findByUser(user_id: String) -> Result<Vec<bson::ordered::OrderedDocument>, Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");
    let id = ObjectId::with_string(&user_id).unwrap();

    let response_document = collection.find(Some(doc! { "user_id" : id }), None)
        .ok().expect("Failed to execute find.");

    response_document
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<bson::ordered::OrderedDocument>, Error>>()

}

pub fn findFollowingsByUser(user_id: String) -> Result<Vec<bson::ordered::OrderedDocument>, Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");
    let collection_user = client.db("twitter").collection("usuario");
    let id = ObjectId::with_string(&user_id).unwrap();

    let user = collection_user.find_one(Some(doc! { "_id" => id }), None).unwrap();
    let result_user = bson::from_bson::<meta::user::GetResponse>(bson::Bson::Document(user.unwrap()));

    let response_document = collection.find(Some(doc! { "user_id" : {
        "$in" : result_user.unwrap().following }}), None)
        .ok().expect("Failed to execute find.");

    response_document
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<bson::ordered::OrderedDocument>, Error>>()
}

pub fn like(tweet_id: String, user_id: String) -> bool {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");

    collection.update_one(
        doc! { "_id" : ObjectId::with_string(&tweet_id).unwrap() }, 
        doc! { "$addToSet" => {"likes" : bson::Bson::ObjectId(ObjectId::with_string(&user_id).unwrap()) }},
        None).ok().expect("Failed to execute update.");

    true

}

pub fn retweet(tweet_id: String, user_id: String) -> Result<Option<bson::ordered::OrderedDocument>, Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");
    let id = ObjectId::with_string(&tweet_id).unwrap();

    let result = collection.find_one(Some(doc! { "_id" : id }), None)
        .ok().expect("Failed to execute find.").unwrap();

    match bson::from_bson::<meta::tweet::PostResponse>(bson::Bson::Document(result)) {
        Ok(t) => {
            
            println!("Deu certo a formatação");
            println!("{}", t.text.to_string());
            let hoje: DateTime<Utc> = Utc::now();  
            let model = RetweetModel {
                text: t.text.to_owned().to_string(),
                user_id: ObjectId::with_string(&user_id).unwrap().to_owned().to_string(),
                retweet_from: ObjectId::with_string(&tweet_id).unwrap().to_owned().to_string(),
                date_created: hoje.to_owned().to_string()
            };
            
            let return_result = model.insert().unwrap();
            Ok(return_result)
        },
        Err(_e) => {
            println!("{}", _e.to_string());
            Ok(None)
        }
    }
}