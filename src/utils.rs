use argon2::{self, Config};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::Rng;
use chrono::prelude::*;


use crate::schema::Claims;
use crate::errors::Error;


pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8;32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash:&str, password:&[u8]) -> bool {
    argon2::verify_encoded(hash,password).unwrap_or(false)
}

pub fn create_jwt(id: &u64, jwt_secret:&[u8]) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        id: *id,
        exp: expiration as usize
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(jwt_secret))
        .map_err(|_| Error::JWTTokenCreationError)
}