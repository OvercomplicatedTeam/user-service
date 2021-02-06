use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub login: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parking {
    pub id:u64,
    pub admin: User,
    pub parking_consumers: Vec<User>
}

pub type Db = Arc<Mutex<Vec<Parking>>>;

pub fn get_db() -> Db {
    Arc::new(Mutex::new(vec![]))
}