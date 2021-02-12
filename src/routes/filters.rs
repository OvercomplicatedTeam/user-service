use crate::routes::{Db, auth};
use serde::de::DeserializeOwned;
use std::convert::Infallible;
use std::env;
use warp::http::{HeaderMap, HeaderValue};
use warp::{filters, Filter, Rejection};

pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 32).and(warp::body::json())
}

pub fn with_jwt_secret() -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || env::var("JWT_SECRET").unwrap())
}

pub fn with_auth(
    obligatory: bool,
) -> impl Filter<Extract = (Option<i32>,), Error = Rejection> + Clone {
    filters::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (headers, obligatory))
        .and_then(auth::authorize)
}
