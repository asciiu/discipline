//use crate::schema::*;

//use diesel::prelude::*;
use uuid::Uuid;
use chrono::{NaiveDateTime};

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
