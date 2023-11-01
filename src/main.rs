use std::{net::Ipv4Addr, str::FromStr};

use log::info;
use dotenv;

mod models;
mod handlers;
mod db;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let host = std::env::var("HOST").unwrap();
    let addr = Ipv4Addr::from_str(&host).unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();
    // Adding loggers 
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found");
    info!("Server starting at {}:{}",host,port);

    let db = db::init_db();
    let notes_routes = routes::notes_routes(db);

    warp::serve(notes_routes)
        .run((addr, port))
        .await;
}
