use crate::schema::ParkingWithoutPassword;
use diesel::*;

#[derive(Queryable, PartialEq, Debug)]
pub struct Parking {
    pub parking_id: i32,
    pub name: String,
    pub password: String,
    pub admin_id: i32,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub login: Option<String>,
    pub password: Option<String>,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct ParkingConsumer {
    pub parking_id: i32,
    pub consumer_id :i32
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
