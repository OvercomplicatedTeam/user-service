use warp::{Filter, Rejection, Reply};
use crate::schema::{Db, Parking, UserCredentials};
use crate::{filters, errors};
use crate::handlers;
use std::convert::Infallible;

pub fn parkings_routes(db:Db) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    register(db.clone())
        .or(parking_create(db.clone()))
        .or(login(db.clone()))
        .recover(errors::handle_rejection)
}

pub fn parking_create(db:Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings")
        .and(warp::post())
        .and(filters::json_body::<Parking>())
        .and(filters::with_db(db))
        .and_then(handlers::create_parking)
}

pub fn register(db:Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(filters::json_body::<UserCredentials>())
        .and(filters::with_db(db))
        .and_then(handlers::register)
}

pub fn login(db:Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(filters::json_body::<UserCredentials>())
        .and(filters::with_db(db))
        .and(filters::with_jwt_secret())
        .and_then(handlers::login)
}