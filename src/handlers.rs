use std::convert::{Infallible};
use warp::{http::StatusCode, Reply, reject, reply, Rejection};

use crate::schema::{Parking, Db, User, UserCredentials, LoginResponse, CreateParkingRequest};
use crate::auth::{hash, verify, create_jwt};
use crate::errors::Error::WrongCredentialsError;


pub async fn create_parking(parking: CreateParkingRequest, db: Db, user_id: u64) -> Result<impl Reply, Infallible> {
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
                admin_id: user_id,
                name: parking.name,
                parking_consumers_id: Vec::new(),
            };
            db.parkings.push(new_parking);
            Ok(StatusCode::CREATED)
        }
    }
}


pub async fn register(new_user: UserCredentials, db: Db) -> Result<impl Reply, Infallible> {
    let mut db = db.lock().await;

    match db
        .users
        .iter()
        .find(|user| user.login == new_user.login) {
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
                login: new_user.login,
                password: hash(new_user.password.as_bytes()),
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
    match db.users.iter().find(|user| user.login == credentials.login) {
        None => Err(reject::custom(WrongCredentialsError)),
        Some(user) => {
            if verify(&user.password, credentials.password.as_bytes()) {
                let token = create_jwt(&user.id, jwt_secret.as_bytes()).unwrap();
                Ok(reply::json(&LoginResponse { token }))
            } else {
                Err(reject::custom(WrongCredentialsError))
            }
        }
    }
}

pub async fn list_parkings(db: Db, user_id: u64) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let parkings = db
        .parkings
        .iter()
        .filter(|parking| parking.admin_id == user_id)
        .collect();
    Ok(reply::json::<Vec<&Parking>>(&parkings))
}