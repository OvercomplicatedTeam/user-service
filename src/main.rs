#[macro_use]
extern crate diesel;

use std::sync::{Arc, Mutex};

use dotenv::dotenv;

mod auth;
mod db;
mod filters;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let db_connection = db::connection::establish_connection();
    let db = Arc::new(Mutex::new(db_connection));

    let api = routes::parkings_routes(db);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
