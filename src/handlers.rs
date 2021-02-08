use std::convert::{Infallible};
use warp::{http::StatusCode, Reply, reject, reply, Rejection};

use crate::schema::{Parking, Db, User, UserCredentials, LoginResponse};
use crate::utils::{hash, verify, create_jwt};
use crate::errors::Error::WrongCredentialsError;


pub async fn create_parking(new_parking: Parking, db: Db) -> Result<impl Reply, Infallible> {
    let mut db = db.lock().await;

    match db.parkings
        .iter()
        .find(|parking| parking.id == new_parking.id) {
        Some(_) => {
            Ok(StatusCode::BAD_REQUEST)
        }
        None => {
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
                    if acc > user.id { acc }
                    else { user.id }
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

type WebResult<T> = std::result::Result<T, Rejection>;

pub async fn login(
    credentials: UserCredentials,
    db:Db,
    jwt_secret: String
) -> WebResult<impl Reply> {
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