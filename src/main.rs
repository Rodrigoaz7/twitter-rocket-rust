#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate chrono;
#[macro_use(bson, doc)] extern crate bson;

use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

mod lib;
mod meta;
mod models;
mod controllers;

#[get("/")]
fn index() -> &'static str {
    "Hello, beautiful people 123!"
}

fn main() {
    rocket::ignite().mount("/", routes![
        controllers::user::get, controllers::user::getAll, controllers::user::insert,
        controllers::tweet::get, controllers::tweet::getAll, controllers::tweet::insert, controllers::tweet::getAllFromUser
    ])
    .register(catchers![controllers::not_found::lookup])
    .launch();
}