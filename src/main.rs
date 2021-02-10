mod auth;
mod errors;
mod filters;
mod handlers;
mod routes;
mod schema;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    // println!("admin login: {}", env::var("ADMIN_LOGIN").unwrap());
    // println!("admin password: {}", env::var("ADMIN_PASSWORD").unwrap());

    let db = schema::get_db();

    let api = routes::parkings_routes(db);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
