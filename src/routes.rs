use warp::{Filter, Rejection, Reply};
use crate::schema::Db;
use crate::filters;
use crate::handlers;

pub fn parkings_routes(db:Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    parking_create(db)
}

pub fn parking_create(db:Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("parkings")
        .and(warp::post())
        .and(filters::json_body())
        .and(filters::with_db(db))
        .and_then(handlers::create_parking)
}