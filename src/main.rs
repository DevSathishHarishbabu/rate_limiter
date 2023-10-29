use log::info;

mod models;
mod handlers;
mod db;
mod routes;

#[tokio::main]
async fn main() {
    // Adding loggers 
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found");
    info!("Server starting at 3771");

    let db = db::init_db();
    let notes_routes = routes::notes_routes(db);

    warp::serve(notes_routes)
        .run(([127,0,0,1], 3771))
        .await;
}
