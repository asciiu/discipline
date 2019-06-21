#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;

pub mod models;
pub mod schema;

use bcrypt::hash;
use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use models::{NewUser, User};
use std::env;
use uuid::Uuid;

pub type DbConPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbCon = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn db_pool() -> DbConPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    r2d2::Pool::builder()
        .max_size(10)
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("failed to create db connection pool")
}


pub fn create_user<'a>(conn: &PgConnection, 
                       username: &'a str, 
                       email: &'a str, 
                       password: &'a str) -> User {
    use schema::users;

    let hashed: String = hash(password, 9).expect("failed to hash password in create_user"); 

    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: email,
        username: username,
        password_hash: &hashed,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
