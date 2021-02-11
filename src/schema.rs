use serde::{Deserialize, Serialize};
use crate::models::Parking;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinParkingResponse {
    pub token: Option<String>,
    pub parking: Parking
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
pub struct ParkingWithoutPassword {
    pub id: i32,
    pub name: String,
    pub admin_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateParkingRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinParkingRequest {
    pub name: String,
    pub password: String,
}
