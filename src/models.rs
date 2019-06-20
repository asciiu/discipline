use uuid::Uuid;
use chrono::{NaiveDateTime};
use crate::schema::users;

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