#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewUser, User};
use std::env;
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, 
                       username: &'a str, 
                       email: &'a str, 
                       password_hash: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: email,
        username: username,
        password_hash: password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
