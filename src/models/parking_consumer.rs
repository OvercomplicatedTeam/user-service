#[derive(Queryable, PartialEq, Debug)]
pub struct ParkingConsumer {
    pub parking_id: i32,
    pub consumer_id: i32,
}
