use crate::handlers;
use crate::schema::{CreateParkingRequest, Db, JoinParkingRequest, UserCredentials};
use crate::{errors, filters};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn parkings_routes(db: Db) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    register(db.clone())
        .or(parking_create(db.clone()))
        .or(login(db.clone()))
        .or(list_parkings(db.clone()))
        .or(parking_join(db.clone()))
        .recover(errors::handle_rejection)
}

pub fn parking_create(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings")
        .and(warp::post())
        .and(filters::json_body::<CreateParkingRequest>())
        .and(filters::with_db(db))
        .and(filters::with_auth(true))
        .and_then(handlers::create_parking)
}

pub fn parking_join(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("join_parking")
        .and(warp::post())
        .and(filters::json_body::<JoinParkingRequest>())
        .and(filters::with_db(db))
        .and(filters::with_auth(false))
        .and_then(handlers::join_parking)
}

pub fn list_parkings(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings")
        .and(warp::get())
        .and(filters::with_db(db))
        .and(filters::with_auth(true))
        .and_then(handlers::list_parkings)
}

pub fn register(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(filters::json_body::<UserCredentials>())
        .and(filters::with_db(db))
        .and_then(handlers::register)
}

pub fn login(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(filters::json_body::<UserCredentials>())
        .and(filters::with_db(db))
        .and(filters::with_jwt_secret())
        .and_then(handlers::login)
}
