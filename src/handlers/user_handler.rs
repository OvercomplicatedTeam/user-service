use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::auth::{create_jwt, hash, verify};

use crate::routes::Db;

use crate::db::db_schema::users;
use crate::handlers::error_handler;
use crate::handlers::error_handler::Error::{JWTTokenError, LoginInUseError, NoPermissionError};
use crate::models::user::{User, UserCredentials};
use diesel::result::Error;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

fn find_user_by_login(db_conn: &PgConnection, user_login: String) -> Result<User, Error> {
    users::dsl::users
        .filter(users::dsl::login.eq(&user_login))
        .first::<User>(db_conn)
}

pub async fn register(
    new_user: UserCredentials,
    db: Db,
    user_id: Option<i32>,
) -> Result<impl Reply, Rejection> {
    let db_conn_mutex = db.lock().unwrap();
    let db_conn = db_conn_mutex.deref();

    let user_by_name = find_user_by_login(db_conn, new_user.login.clone());
    let hashed_password = Some(hash(new_user.password.as_bytes()));
    let new_credentials = (
        users::dsl::login.eq(Some(new_user.login)),
        users::dsl::password.eq(hashed_password),
    );
    match user_by_name {
        Ok(_) => return Err(reject::custom(LoginInUseError)),
        Err(_) => {}
    };
    match user_id {
        None => {
            match insert_into(users::dsl::users)
                .values(new_credentials)
                .execute(db_conn)
            {
                Ok(res) => Ok(StatusCode::CREATED),
                Err(e) => Err(reject::reject()),
            }
        }
        Some(id) => {
            let target = users::dsl::users.filter(users::dsl::user_id.eq(id));
            let updated_user: Result<User, Error> = target.first::<User>(db_conn);
            match updated_user {
                Ok(user) => {
                    if user.login.is_none() || user.password.is_none() {
                        diesel::update(target)
                            .set(new_credentials.clone())
                            .execute(db_conn);
                        Ok(StatusCode::CREATED)
                    } else {
                        Ok(StatusCode::UNAUTHORIZED)
                    }
                }
                Err(_) => Ok(StatusCode::UNAUTHORIZED),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
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
            let user_password = found_users.password.clone().unwrap();
            if verify(&user_password, credentials.password.as_bytes()) {
                let token = create_jwt(&found_users.id, jwt_secret.as_bytes()).unwrap();
                Ok(reply::json(&LoginResponse { token }))
            } else {
                Err(reject::custom(error_handler::Error::WrongCredentialsError))
            }
        }
        _ => Err(reject::custom(error_handler::Error::WrongCredentialsError)),
    }
}
