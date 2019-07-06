#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate jsonwebtoken as jwt;
extern crate uuid;

pub mod models;
pub mod schema;


use bcrypt::hash;
use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use jwt::{encode, decode, Header, Validation};
use models::{auth, NewUser, User};
use std::env;
use uuid::Uuid;

pub type DbConPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbCon = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
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

pub fn create_jwt(id: &str) -> String {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let hrs = std::env::var("JWT_EXPIRE_HR").expect("JWT_EXPIRE_HR not set");
    let hrs = hrs.parse::<u64>().unwrap();
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    let my_claims =
        auth::Claims { 
            id: id.to_owned(),
            sub: "flow.com".to_owned(), 
            company: "flow".to_owned(), 
            exp: (since_the_epoch.as_secs() * hrs *  3600) as usize
        };
        
    encode(&Header::default(), &my_claims, secret.as_ref()).unwrap()
}

pub fn validate_jwt(token: String) -> jwt::errors::Result<jwt::TokenData<auth::Claims>> {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let validation = Validation { sub: Some("flow.com".to_string()), ..Validation::default() };
    decode::<auth::Claims>(&token, secret.as_ref(), &validation) 
}

pub fn create_user<'a>(conn: &PgConnection, 
                       email: &'a str, 
                       username: &'a str, 
                       password: &'a str) -> Result<User, diesel::result::Error> {
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
}
