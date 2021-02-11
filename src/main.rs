#[macro_use]
extern crate diesel;

mod auth;
mod db_schema;
mod errors;
mod filters;
mod handlers;
mod lib;
mod models;
mod routes;
mod schema;

use dotenv::dotenv;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let db_connection = lib::establish_connection();
    let db = Arc::new(Mutex::new(db_connection));

    let api = routes::parkings_routes(db);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
