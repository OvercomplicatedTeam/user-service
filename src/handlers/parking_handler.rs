use std::convert::Infallible;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};


use crate::routes::Db;

use crate::db::db_schema::users;
use crate::db::db_schema::{parkings, parkings_consumers};
use crate::handlers::error_handler;
use crate::models::parking::{Parking, ParkingWithoutPassword};
use crate::models::parking_consumer::ParkingConsumer;
use crate::models::user::User;
use diesel::result::Error;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use crate::security::create_jwt;


fn get_consumed_parkings(db_conn: &PgConnection, user_id: i32) -> Vec<Parking> {
    let consumers: Vec<ParkingConsumer> = match parkings_consumers::dsl::parkings_consumers
        .filter(parkings_consumers::dsl::consumer_id.eq(user_id))
        .load::<ParkingConsumer>(db_conn)
    {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    };
    consumers
        .into_iter()
        .map(|c| {
            parkings::dsl::parkings
                .find(c.parking_id)
                .first(db_conn)
                .unwrap()
        })
        .rev()
        .collect()
}

pub async fn list_parkings(db: Db, user_id: Option<i32>) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();

    let mut consumed_parkings: Vec<ParkingWithoutPassword> =
        get_consumed_parkings(db_conn, user_id.unwrap())
            .iter()
            .map(|parking| parking.to_parking_without_password())
            .collect();

    let mut parkings: Vec<ParkingWithoutPassword> = match parkings::dsl::parkings
        .filter(parkings::dsl::admin_id.eq(user_id.unwrap()))
        .load::<Parking>(db_conn)
    {
        Ok(result) => result
            .iter()
            .map(|parking| parking.to_parking_without_password())
            .collect::<Vec<ParkingWithoutPassword>>(),
        Err(_) => return Err(reject()),
    };
    consumed_parkings.append(&mut parkings);
    Ok(reply::json::<Vec<ParkingWithoutPassword>>(
        &consumed_parkings,
    ))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateParkingRequest {
    pub name: String,
    pub password: String,
}

pub async fn create_parking(
    parking: CreateParkingRequest,
    db: Db,
    user_id: Option<i32>,
) -> Result<impl Reply, Infallible> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();

    match parkings::dsl::parkings
        .filter(parkings::dsl::name.eq(parking.name.clone()))
        .load::<Parking>(db_conn)
    {
        Ok(parkings_found) => {
            if parkings_found.is_empty() {
                let parking_id = insert_into(parkings::dsl::parkings)
                    .values((
                        parkings::dsl::admin_id.eq(user_id.unwrap()),
                        parkings::dsl::name.eq(parking.name.clone()),
                        parkings::dsl::password.eq(parking.password),
                    ))
                    .returning(parkings::dsl::parking_id)
                    .get_result::<i32>(db_conn);
                match parking_id {
                    Ok(_) => Ok(StatusCode::CREATED),
                    Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Ok(StatusCode::BAD_REQUEST)
            }
        }
        Err(_) => Ok(StatusCode::BAD_REQUEST),
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinParkingResponse {
    pub token: Option<String>,
    pub parking: Parking,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinParkingRequest {
    pub name: String,
    pub password: String,
}

pub async fn join_parking(
    body: JoinParkingRequest,
    db: Db,
    user_id: Option<i32>,
    jwt_secret: String,
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    let (user_id, token) = match user_id {
        None => {
            let user = insert_into(users::dsl::users)
                .values((
                    users::dsl::login.eq::<Option<String>>(Option::None),
                    users::dsl::password.eq::<Option<String>>(Option::None),
                ))
                .returning(users::dsl::users::all_columns())
                .get_results::<User>(db_conn);
            let id = user.unwrap().first().unwrap().id;
            let token = create_jwt(&id, jwt_secret.as_bytes()).unwrap();
            (id, Some(token))
        }
        Some(id) => (id, None),
    };

    let parking: Result<Parking, Error> = parkings::dsl::parkings
        .filter(
            parkings::dsl::name
                .eq(body.name)
                .and(parkings::dsl::password.eq(body.password)),
        )
        .first::<Parking>(db_conn);

    match parking {
        Ok(valid_parking) => {
            insert_into(parkings_consumers::dsl::parkings_consumers)
                .values((
                    parkings_consumers::dsl::parking_id.eq(valid_parking.parking_id),
                    parkings_consumers::dsl::consumer_id.eq(user_id),
                ))
                .execute(db_conn);
            Ok(reply::json::<JoinParkingResponse>(&JoinParkingResponse {
                token,
                parking: valid_parking,
            }))
        }
        Err(_) => Err(reject::custom(error_handler::Error::WrongParkingError)),
    }
}
