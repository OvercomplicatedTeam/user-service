use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().expect(".env file not found");
    println!("admin login: {}", env::var("ADMIN_LOGIN").unwrap());
    println!("admin password: {}", env::var("ADMIN_PASSWORD").unwrap());

}
