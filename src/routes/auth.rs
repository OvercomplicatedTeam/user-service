use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use warp::http::HeaderMap;
use warp::hyper::header::AUTHORIZATION;
use warp::hyper::http::HeaderValue;
use warp::{reject, Rejection};
use std::env;
use crate::handlers::error_handler::Error;
use crate::security::Claims;

const BEARER: &str = "Bearer ";

pub async fn authorize(
    (headers, obligatory): (HeaderMap<HeaderValue>, bool),
) -> Result<Option<i32>, Rejection> {
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
