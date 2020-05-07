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

//TODO - the rust backend should serve the react frontend

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/auth?<username>&<password>")]
fn auth(username: &RawStr, password: &RawStr) -> String {
    println!("Got an auth request from {0} with password {1}", username.as_str(), password.as_str());
    let connection = establish_connection();
    let token = auth_user(&connection, username, password); //TODO CHANGE THIS TO GET TOKEN/SOMETHING
    if token {
        format!("Authenticated user {}", username)
    }
    else {
        format!("Failed authentication for user {}", username)
    }
}

#[post("/create?<username>&<password>")]
fn new_user(username: &RawStr, password: &RawStr) -> String {
    println!("Got a request to create a new user with username {}", username);
    //TODO: Check to make sure user doesn't exist
    let connection = establish_connection();
    //TODO: need to hash the password here - store a salt?
    //How exactly does salting work?
    create_user(&connection, username, password);
    format!("Created a new user with username {}", username)
}

fn main() {
    rocket::ignite().mount("/", routes![index, auth, new_user]).launch();
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

pub fn auth_user(conn: &SqliteConnection, auth_usernme: &str, auth_password: &str) -> bool {
    //let hashed_password = do_something(password);
    use schema::users::dsl::*;
    use crate::models::User;
    let mut authenticated = false;
    let res = users
        .filter(username.eq(auth_usernme))
        .filter(hashed_password.eq(auth_password))
        .load::<User>(conn)
        .expect("Error getting user");
    if res.len() == 1 {
        authenticated = true;
    }
    //TODO: This should return a token or something
    //Make a new table of tokens that matches to the users ID
    authenticated
}