#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use(bson)] extern crate bson;

extern crate chrono;

mod lib;
mod meta;
mod models;
mod controllers;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, this is a API!"
}

fn main() {
    rocket::ignite().mount("/", routes![
        controllers::user::get, controllers::user::getAll, controllers::user::insert,
        controllers::user::follow,
        controllers::tweet::get, controllers::tweet::getAll, controllers::tweet::insert,
        controllers::tweet::getAllFromUser, controllers::tweet::like, controllers::tweet::getAllFromUsersFollowing,
        controllers::tweet::retweet
    ])
    .register(catchers![controllers::not_found::lookup])
    .launch();
}