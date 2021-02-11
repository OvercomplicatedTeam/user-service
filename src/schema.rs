use serde::{Deserialize, Serialize};

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
