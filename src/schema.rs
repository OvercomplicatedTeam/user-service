use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub id: u64,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    pub login: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parking {
    pub id: u64,
    pub name: String,
    pub admin_id: u64,
    pub parking_consumers_id: Vec<u64>,
}

pub struct System {
    pub users: Vec<User>,
    pub parkings: Vec<Parking>,
}

pub type Db = Arc<Mutex<System>>;

pub fn get_db() -> Db {
    Arc::new(
        Mutex::new(
            System {
                users: vec![],
                parkings: vec![],
            }
        )
    )
}