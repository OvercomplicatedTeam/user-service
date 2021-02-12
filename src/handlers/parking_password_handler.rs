use warp::{reject, reply, Rejection, Reply};

use crate::routes::Db;

use crate::db::db_schema::parkings;
use crate::handlers::error_handler;
use crate::models::parking::Parking;
use diesel::result::Error;
use diesel::*;
use std::ops::Deref;

pub async fn get_parking_password(
    parking_id: i32,
    db: Db,
    user_id: Option<i32>,
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    match user_id {
        None => Err(reject::custom(error_handler::Error::NoPermissionError)),
        Some(owner_id) => {
            let parking: Result<Parking, Error> = parkings::dsl::parkings
                .find::<i32>(parking_id)
                .first::<Parking>(db_conn);
            match parking {
                Err(_) => Err(reject::custom(error_handler::Error::NoPermissionError)),
                Ok(parking) => {
                    if parking.admin_id == owner_id {
                        Ok(reply::json(&parking))
                    } else {
                        Err(reject::custom(error_handler::Error::NoPermissionError))
                    }
                }
            }
        }
    }
}
