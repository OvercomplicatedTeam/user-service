use std::convert::{Infallible};
use warp::{http::StatusCode, Reply, reject, reply, Rejection};

use crate::schema::{Parking, Db, User, UserCredentials, LoginResponse, CreateParkingRequest, ParkingWithoutPassword, JoinParkingRequest};
use crate::auth::{hash, verify, create_jwt};
use crate::errors::Error::{WrongCredentialsError, WrongParkingError};


pub async fn create_parking(parking: CreateParkingRequest, db: Db, user_id: Option<u64>) -> Result<impl Reply, Infallible> {
    let mut db = db.lock().await;

    match db.parkings
        .iter()
        .find(|p| p.name == parking.name) {
        Some(_) => {
            Ok(StatusCode::BAD_REQUEST)
        }
        None => {
            let new_id = db
                .parkings
                .iter()
                .fold(0, |acc, parking|
                    if acc > parking.id { acc } else { parking.id },
                ) + 1;
            let new_parking = Parking {
                id: new_id,
                admin_id: user_id.unwrap(),
                name: parking.name,
                password: parking.password,
                parking_consumers_id: vec![],
            };
            db.parkings.push(new_parking);
            Ok(StatusCode::CREATED)
        }
    }
}

//TODO: finish this function
pub async fn join_parking(
    body: JoinParkingRequest,
    db: Db,
    user_id: Option<u64>,
) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
     match db
        .parkings
        .iter()
        .find(|parking| parking.name == body.name && parking.password == body.password) {
        None => return Err(reject::custom(WrongParkingError)),
        Some(_) => {
            if user_id.is_none() {
                //add new user as guest
            }else {
              //push userId  to the parking_consumers_ids
            }
        }
    };

    Ok(StatusCode::CREATED)//remove it
}


pub async fn register(new_user: UserCredentials, db: Db) -> Result<impl Reply, Infallible> {
    let mut db = db.lock().await;

    match db
        .users
        .iter()
        .filter(|user| user.login.is_some())
        .find(|user| user.login.clone().unwrap() == new_user.login) {
        Some(_) => { Ok(StatusCode::BAD_REQUEST) }
        None => {
            let new_id = db
                .users
                .iter()
                .fold(0, |acc, user|
                    if acc > user.id { acc } else { user.id },
                ) + 1;
            let hashed_user = User {
                id: new_id,
                login: Some(new_user.login),
                password: Some(hash(new_user.password.as_bytes())),
            };
            db.users.push(hashed_user);
            Ok(StatusCode::CREATED)
        }
    }
}


pub async fn login(
    credentials: UserCredentials,
    db: Db,
    jwt_secret: String,
) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    match db
        .users
        .iter()
        .filter(|user| user.login.is_some())
        .find(|user| user.login.clone().unwrap() == credentials.login) {
        None => Err(reject::custom(WrongCredentialsError)),
        Some(user) => {
            let user_password = user.password.clone().unwrap();
            if verify(&user_password, credentials.password.as_bytes()) {
                let token = create_jwt(&user.id, jwt_secret.as_bytes()).unwrap();
                Ok(reply::json(&LoginResponse { token }))
            } else {
                Err(reject::custom(WrongCredentialsError))
            }
        }
    }
}

pub async fn list_parkings(db: Db, user_id: Option<u64>) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let parkings = db
        .parkings
        .iter()
        .filter(|parking| parking.admin_id == user_id.unwrap())
        .map(|parking| parking.to_parking_without_password())
        .collect();
    Ok(reply::json::<Vec<ParkingWithoutPassword>>(&parkings))
}