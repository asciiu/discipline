extern crate base64;
extern crate crypto;

use crate::schema::users;
use base64::encode;
use chrono::{NaiveDateTime, Utc};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use rand::Rng;
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;


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

#[derive(Debug)]
#[derive(Queryable)]
pub struct RefreshToken {
	pub id: Uuid,
	pub user_id: Uuid,
	pub selector: String,
	pub authenticator: String,
	pub token_hash: String,
	pub expiration: NaiveDateTime,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, expiration: NaiveDateTime) -> RefreshToken {
        let nid = Uuid::new_v4(); 
        // selector
        let random_bytes = rand::thread_rng().gen::<[u8; 16]>();
        let selector = encode(&random_bytes);

        let random_data = rand::thread_rng().gen::<[u8; 32]>();
        let authenticator = encode(&random_data);
        let token_hash = RefreshToken::hash_auth(&authenticator);

	    RefreshToken{
		    id:     nid,
		    user_id: user_id,
            selector: selector,
            authenticator: authenticator,
            token_hash: token_hash,
            expiration: expiration,
	    }
    }

    pub fn tokenize(&self) -> String {
        format!("{}:{}", encode(&self.selector), self.authenticator)
    }

    fn hash_auth(authenticator: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(&authenticator);
        encode(&hasher.result_str())
    }

    pub fn is_valid(&self, authenticator: &str) -> bool {
        let token_hash = RefreshToken::hash_auth(authenticator);
        let now = Utc::now().naive_utc();

        token_hash == token_hash && self.expiration > now
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use base64::decode;

    #[test]
    fn new_refresh() {
        let now = Utc::now();
        let expires = (now + Duration::hours(24)).naive_utc();
        let user_id = Uuid::new_v4();
        let rt = RefreshToken::new(user_id, expires);

        let refresh: String = rt.tokenize();
        let sel: Vec<&str> = refresh.split(":").collect();

        let selector: Vec<u8> = decode(&sel[0]).unwrap();
        let selector: String = String::from_utf8(selector).unwrap();

        assert_eq!(selector, rt.selector);
    }

    #[test]
    fn is_valid() {
        let now = Utc::now();
        let expires = (now + Duration::hours(24)).naive_utc();
        let user_id = Uuid::new_v4();
        let rt = RefreshToken::new(user_id, expires);
        let token: String = rt.tokenize();
        let data: Vec<&str> = token.split(":").collect();
        
        assert!(rt.is_valid(&data[1]));
    }

    #[test]
    fn is_not_valid() {
        let now = Utc::now();
        let expired = (now - Duration::hours(1)).naive_utc();
        let user_id = Uuid::new_v4();
        let rt = RefreshToken::new(user_id, expired);
        let token: String = rt.tokenize();
        let data: Vec<&str> = token.split(":").collect();
        
        assert!(rt.is_valid(&data[1]) == false);
    }
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