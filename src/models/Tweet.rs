use std;
use std::io;
use bson;
use bson::oid::ObjectId;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::{doc, error::Error};
use mongodb;

use crate::lib;
use crate::meta;

#[derive(Debug)]
pub struct Model {
  pub text: String,
  pub user_id: String,
  pub date_created: String
}

impl Model {
    pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
      
      doc! { 
        "email": self.text.to_owned(),
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

pub fn like(tweet_id: String, user_id: String) -> bool {
    let client = lib::mongo::establish_connection();
    let collection = client.db("twitter").collection("tweet");
    let idTweet = ObjectId::with_string(&tweet_id).unwrap();
    let idUser = ObjectId::with_string(&user_id).unwrap();

    let result = collection.find_one(Some(doc! { "_id" : idTweet }), None)
        .ok().expect("Failed to execute find.").unwrap();
    
    let result_formatted = bson::from_bson::<meta::tweet::GetResponseForUpdate>(bson::Bson::Document(result));

    match result_formatted {
        Ok(tweet) => {

            let mut items = tweet.likes;
            let item = items.last().cloned();
            items.push(bson::Bson::ObjectId(ObjectId::with_string(&user_id).unwrap()));
            
            /*É necessário checar ainda se o usuário já curtiu o tweet*/
            let r = collection.update_one(
                doc! { "_id" : ObjectId::with_string(&tweet_id).unwrap() }, 
                doc! { "$set" => {"likes" : items }},
                None).ok().expect("Failed to execute update.");

        },
        Err(_e) => {
            println!("Apresentei um erro: {}", _e.to_string());
        }
    }

    true

}