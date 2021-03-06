use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug, Deserialize, Serialize)]
pub struct Parking {
    pub parking_id: i32,
    pub name: String,
    pub password: String,
    pub admin_id: i32,
}

impl Parking {
    pub fn to_parking_without_password(&self) -> ParkingWithoutPassword {
        ParkingWithoutPassword {
            id: self.parking_id,
            name: self.name.clone(),
            admin_id: self.admin_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParkingWithoutPassword {
    pub id: i32,
    pub name: String,
    pub admin_id: i32,
}
