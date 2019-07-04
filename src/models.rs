use uuid::Uuid;
use chrono::{NaiveDateTime};
use crate::schema::users;
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[derive(juniper::GraphQLObject)]
#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub password_hash: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub deleted_on: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub password_hash: &'a str,
}

#[derive(juniper::GraphQLObject)]
pub struct AuthToken {
	pub jwt: String,
    pub refresh: String,
}