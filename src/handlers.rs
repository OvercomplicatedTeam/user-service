use std::convert::Infallible;
use warp::{http::StatusCode, reject, reply, Filter, Rejection, Reply};

use crate::auth::{create_jwt, hash, verify};
use crate::errors::Error::{
    LoginInUseError, NoPermissionError, WrongCredentialsError, WrongParkingError,
};
use crate::routes::Db;
use crate::schema::{CreateParkingRequest, JoinParkingRequest, LoginResponse, ParkingWithoutPassword, UserCredentials, JoinParkingResponse};

use crate::db_schema::parkings;
use crate::db_schema::parkings_consumers;
use crate::db_schema::users::columns::password;
use crate::db_schema::users::dsl::{login};
use crate::db_schema::users;
use crate::models::{Parking, ParkingConsumer, User};
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
        None => Err(reject::custom(NoPermissionError)),
        Some(owner_id) => {
            let parking: Result<Parking, Error> = parkings::dsl::parkings
                .find::<i32>(parking_id)
                .first::<Parking>(db_conn);
            match parking {
                Err(_) => Err(reject::custom(NoPermissionError)),
                Ok(parking) => {
                    if parking.admin_id == owner_id {
                        Ok(reply::json(&parking))
                    } else {
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
    jwt_secret: String
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    let (user_id, token) = match user_id {
        None => {
            let user = insert_into(users::dsl::users)
                .values((
                    login.eq::<Option<String>>(Option::None),
                    password.eq::<Option<String>>(Option::None)
                ))
                .returning(users::dsl::users::all_columns())
                .get_results::<User>(db_conn);
            let id = user.unwrap().first().unwrap().id;
            let token = create_jwt(&id,jwt_secret.as_bytes()).unwrap();
            (id, Some(token))
        },
        Some(id) => (id,None)
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
            Ok(reply::json::<JoinParkingResponse>(&JoinParkingResponse{token, parking:valid_parking}))
        }
        Err(_) => Err(reject::custom(WrongParkingError)),
    }
}

fn find_user_by_login(db_conn: &PgConnection, user_login: String) -> Result<Vec<User>, Error> {
    users::dsl::users.filter(login.eq(&user_login)).load::<User>(db_conn)
}

pub async fn register(
    new_user: UserCredentials,
    db: Db,
    user_id:Option<i32>
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();
    let users_by_name = find_user_by_login(db_conn, new_user.login.clone());
    let hashed_password = Some(hash(new_user.password.as_bytes()));
    let new_credentials = (
        login.eq(Some(new_user.login)),
        password.eq(hashed_password)
    );
    match user_id {
        None => {
            match users_by_name {
                Ok(result) => {
                    if result.is_empty() {

                        insert_into(users::dsl::users)
                            .values(new_credentials)
                            .execute(db_conn);
                        Ok(StatusCode::CREATED)
                    } else {
                        Err(reject::custom(LoginInUseError))
                    }
                }
                Err(_) => Err(reject::reject()),
            }
        }
        Some(id) => {
            let target = users::dsl::users.filter((users::dsl::user_id.eq(id)));
            diesel::update(target).set(new_credentials.clone()).execute(db_conn);
            Ok(StatusCode::CREATED)
        }
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
