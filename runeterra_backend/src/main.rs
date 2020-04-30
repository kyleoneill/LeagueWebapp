#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

use rocket::http::RawStr;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::NewUser;

#[get("/")]
fn index() -> &'static str {
    "<p>Hello, world!</>"
}

#[get("/auth?<username>&<hashed_password>")]
fn hello(username: &RawStr, hashed_password: &RawStr) -> String {
    println!("Got an auth request from {0} with password {1}", username.as_str(), hashed_password.as_str());
    let connection = establish_connection();
    create_user(&connection, username, hashed_password);
    format!("Created a new user with username {}", username)
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &SqliteConnection, username: &str, hashed_password: &str) -> usize {
    use schema::users;
    let creation_date = "placeholder";

    let new_user = NewUser { username, hashed_password, creation_date };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post")
} 