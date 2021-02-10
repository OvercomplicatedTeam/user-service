use argon2::{self, Config};
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use warp::http::HeaderMap;
use warp::hyper::header::AUTHORIZATION;
use warp::hyper::http::HeaderValue;
use warp::{reject, Rejection};

use crate::errors::Error;
use crate::schema::Claims;
use std::env;

const BEARER: &str = "Bearer ";

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

pub fn create_jwt(id: &u64, jwt_secret: &[u8]) -> Result<String, Error> {
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

pub async fn authorize(
    (headers, obligatory): (HeaderMap<HeaderValue>, bool),
) -> Result<Option<u64>, Rejection> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            Ok(Some(decoded.claims.id))
        }
        Err(e) => {
            if obligatory {
                Err(reject::custom(e))
            } else {
                Ok(None)
            }
        }
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    let header = match headers.get(AUTHORIZATION) {
        Some(h) => h,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(h) => h,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };

    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
