use argon2::{self, Config};
use chrono::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::handlers::error_handler::Error;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

pub fn create_jwt(id: &i32, jwt_secret: &[u8]) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(60))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        id: *id,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(jwt_secret))
        .map_err(|_| Error::JWTTokenCreationError)
}

