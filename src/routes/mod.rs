use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use diesel::PgConnection;
use warp::{Filter, Rejection, Reply};

use crate::handlers::{error_handler, parking_handler, parking_password_handler, user_handler};
use crate::handlers::parking_handler::{CreateParkingRequest, JoinParkingRequest};
use crate::models::user::UserCredentials;

mod filters;
mod auth;

pub type Db = Arc<Mutex<PgConnection>>;

pub fn parkings_routes(
    db_connection: Db,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    register(db_connection.clone())
        .or(parking_create(db_connection.clone()))
        .or(log_in(db_connection.clone()))
        .or(list_parkings(db_connection.clone()))
        .or(parking_join(db_connection.clone()))
        .or(get_parking_password(db_connection.clone()))
        .recover(error_handler::handle_rejection)
}

pub fn get_parking_password(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings" / i32 / "password")
        .and(warp::get())
        .and(filters::with_db(db))
        .and(filters::with_auth(true))
        .and_then(parking_password_handler::get_parking_password)
}

pub fn parking_create(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings")
        .and(warp::post())
        .and(filters::json_body::<CreateParkingRequest>())
        .and(filters::with_db(db))
        .and(filters::with_auth(true))
        .and_then(parking_handler::create_parking)
}

pub fn parking_join(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("join_parking")
        .and(warp::post())
        .and(filters::json_body::<JoinParkingRequest>())
        .and(filters::with_db(db))
        .and(filters::with_auth(false))
        .and(filters::with_jwt_secret())
        .and_then(parking_handler::join_parking)
}

pub fn list_parkings(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings")
        .and(warp::get())
        .and(filters::with_db(db))
        .and(filters::with_auth(true))
        .and_then(parking_handler::list_parkings)
}

pub fn register(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(filters::json_body::<UserCredentials>())
        .and(filters::with_db(db))
        .and(filters::with_auth(false))
        .and_then(user_handler::register)
}

pub fn log_in(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(filters::json_body::<UserCredentials>())
        .and(filters::with_db(db))
        .and(filters::with_jwt_secret())
        .and_then(user_handler::log_in)
}
