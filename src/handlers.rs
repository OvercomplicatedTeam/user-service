use std::convert::Infallible;
use warp::{http::StatusCode, reject, reply, Rejection, Reply, Filter};

use crate::auth::{create_jwt, hash, verify};
use crate::errors::Error::{LoginInUseError, WrongCredentialsError, WrongParkingError, NoPermissionError};
use crate::routes::Db;
use crate::schema::{
    CreateParkingRequest, JoinParkingRequest, LoginResponse, ParkingWithoutPassword,
    UserCredentials,
};

use crate::db_schema::parkings;
use crate::db_schema::parkings_consumers;
use crate::db_schema::users::columns::password;
use crate::db_schema::users::dsl::{login, users};
use crate::models::{Parking, User, ParkingConsumer};
use diesel::result::Error;
use diesel::*;
use std::ops::Deref;

pub async fn get_parking_password(
    parking_id: i32,
    db:Db,
    user_id: Option<i32>
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    match user_id {
        None => Err(reject::custom(NoPermissionError)),
        Some(owner_id) => {

            let parking:Result<Parking, Error> = parkings::dsl::parkings
                .find::<i32>(parking_id)
                .first::<Parking>(db_conn);
            match parking {
                Err(_) => Err(reject::custom(NoPermissionError)),
                Ok(parking) => {
                    if parking.admin_id == owner_id{
                        Ok(reply::json(&parking))
                    }else {
                        Err(reject::custom(NoPermissionError))
                    }

                }
            }
        }
    }
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

pub async fn join_parking(
    body: JoinParkingRequest,
    db: Db,
    user_id: Option<i32>,
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
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
                    parkings_consumers::dsl::consumer_id.eq(user_id.unwrap()),
                ))
                .execute(db_conn);
            Ok(StatusCode::OK)
        }
        Err(_) => Err(reject::custom(WrongParkingError)),
    }
}

fn find_user_by_login(db_conn: &PgConnection, user_login: String) -> Result<Vec<User>, Error> {
    users.filter(login.eq(&user_login)).load::<User>(db_conn)
}

pub async fn register(new_user: UserCredentials, db: Db) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    let users_by_name = find_user_by_login(db_conn, new_user.login.clone());

    match users_by_name {
        Ok(result) => {
            if result.is_empty() {
                let hashed_password = Some(hash(new_user.password.as_bytes()));
                insert_into(users)
                    .values((login.eq(Some(new_user.login)), password.eq(hashed_password)))
                    .execute(db_conn);
                Ok(StatusCode::CREATED)
            } else {
                Err(reject::custom(LoginInUseError))
            }
        }
        Err(_) => Err(reject::reject()),
    }
}

pub async fn log_in(
    credentials: UserCredentials,
    db: Db,
    jwt_secret: String,
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    let users_by_name = find_user_by_login(db_conn, credentials.login.clone());

    match users_by_name {
        Ok(found_users) => {
            if found_users.is_empty() {
                Err(reject::custom(WrongCredentialsError))
            } else {
                let auth_user = found_users.first().unwrap();
                let user_password = auth_user.password.clone().unwrap();
                if verify(&user_password, credentials.password.as_bytes()) {
                    let token = create_jwt(&auth_user.id, jwt_secret.as_bytes()).unwrap();
                    Ok(reply::json(&LoginResponse { token }))
                } else {
                    Err(reject::custom(WrongCredentialsError))
                }
            }
        }
        _ => Err(reject::custom(WrongCredentialsError)),
    }
}

fn get_consumed_parkings(db_conn: &PgConnection, user_id:i32) -> Vec<Parking>{
    let consumers:Vec<ParkingConsumer>
        = match parkings_consumers::dsl::parkings_consumers
        .filter(parkings_consumers::dsl::consumer_id.eq(user_id))
        .load::<ParkingConsumer>(db_conn){
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    };
    consumers.into_iter()
        .map(|c| parkings::dsl::parkings.find(c.parking_id).first(db_conn).unwrap())
        .rev()
        .collect()

}

pub async fn list_parkings(db: Db, user_id: Option<i32>) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();

    let mut consumed_parkings: Vec<ParkingWithoutPassword> =
        get_consumed_parkings(db_conn,user_id.unwrap())
            .iter()
            .map(|parking|parking.to_parking_without_password())
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
    Ok(reply::json::<Vec<ParkingWithoutPassword>>(&consumed_parkings))
}
