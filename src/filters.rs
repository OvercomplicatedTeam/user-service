use crate::schema::{Db, Parking};
use warp::{Filter, Rejection};
use std::convert::Infallible;

pub fn with_db(db:Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body() -> impl Filter<Extract = (Parking,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 32).and(warp::body::json())
}