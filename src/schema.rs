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
    pub login: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parking {
    pub id: u64,
    pub name: String,
    pub password: String,
    pub admin_id: u64,
    pub parking_consumers_id: Vec<u64>,
}

impl Parking {
    pub fn to_parking_without_password(&self) -> ParkingWithoutPassword {
        ParkingWithoutPassword{
            id: self.id,
            name: self.name.clone(),
            admin_id: self.admin_id,
            parking_consumers_id: self.parking_consumers_id.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParkingWithoutPassword {
    pub id: u64,
    pub name: String,
    pub admin_id: u64,
    pub parking_consumers_id: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateParkingRequest {
    pub name: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinParkingRequest {
    pub name: String,
    pub password: String
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