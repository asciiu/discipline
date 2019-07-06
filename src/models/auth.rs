extern crate base64;
extern crate crypto;

use base64::encode;
use chrono::{NaiveDateTime, Utc};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use rand::Rng;
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(juniper::GraphQLObject)]
pub struct AuthToken {
	pub jwt: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub sub: String,
    pub company: String,
    pub exp: usize,
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
        // selector
        let random_bytes = rand::thread_rng().gen::<[u8; 16]>();
        let selector = encode(&random_bytes);

        // authenticator
        let random_data = rand::thread_rng().gen::<[u8; 32]>();
        let authenticator = encode(&random_data);
        let token_hash = RefreshToken::hash_auth(&authenticator);

	    RefreshToken{
		    id: Uuid::new_v4(),
		    user_id: user_id,
            selector: selector,
            authenticator: authenticator,
            token_hash: token_hash,
            expiration: expiration,
	    }
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

    pub fn to_string(&self) -> String {
        format!("{}:{}", encode(&self.selector), self.authenticator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use base64::decode;

    #[test]
    fn valid_selector() {
        let now = Utc::now();
        let expires = (now + Duration::hours(24)).naive_utc();
        let user_id = Uuid::new_v4();
        let fresh = RefreshToken::new(user_id, expires);

        let token: String = fresh.to_string();
        let data: Vec<&str> = token.split(":").collect();

        let selector: Vec<u8> = decode(&data[0]).unwrap();
        let selector: String = String::from_utf8(selector).unwrap();

        assert_eq!(selector, fresh.selector);
    }

    #[test]
    fn is_valid() {
        let now = Utc::now();
        let expires = (now + Duration::hours(24)).naive_utc();
        let user_id = Uuid::new_v4();
        let fresh = RefreshToken::new(user_id, expires);
        let token: String = fresh.to_string();
        let data: Vec<&str> = token.split(":").collect();
        let authenticator = data[1];
        
        assert!(fresh.is_valid(&authenticator));
    }

    #[test]
    fn is_not_valid() {
        let now = Utc::now();
        let expired = (now - Duration::hours(1)).naive_utc();
        let user_id = Uuid::new_v4();
        let fresh = RefreshToken::new(user_id, expired);
        let token: String = fresh.to_string();
        let data: Vec<&str> = token.split(":").collect();
        let authenticator = data[1];
        
        assert!(fresh.is_valid(&authenticator) == false);
    }
}