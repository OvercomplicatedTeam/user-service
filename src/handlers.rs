use std::convert::Infallible;
use warp::{http::StatusCode, Reply};

use crate::schema::{Parking, Db};


pub async fn create_parking(new_parking:Parking, db:Db) -> Result<impl Reply, Infallible> {

    let mut parkings = db.lock().await;

    match parkings
        .iter()
        .find(|parking| parking.id == new_parking.id){
        Some(_) => {
            Ok(StatusCode::BAD_REQUEST)
        }
        None => {
            parkings.push(new_parking);
            Ok(StatusCode::CREATED)
        }
    }

}