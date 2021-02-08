use crate::schema::{Db};
use warp::{Filter, Rejection};
use std::convert::Infallible;
use serde::de::DeserializeOwned;
use std::env;

pub fn with_db(db:Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 32).and(warp::body::json())
}

pub fn with_jwt_secret() -> impl Filter <Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || env::var("jwt_secret").unwrap())
}