use std::convert::Infallible;
use warp::{self,Filter};

use crate::db::Db;
use crate::handlers;
use crate::models;

fn with_db(db: Db) -> impl Filter<Extract = (Db,),Error= Infallible> + Clone{
    warp::any().map(move || db.clone())
}

fn extract_json_body() -> impl Filter<Extract = (models::Note,), Error = warp::Rejection> + Clone{
    warp::body::content_length_limit(1024*16)
        .and(warp::body::json())
}

fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .map(|| {"Hello World"})
}

fn notes_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("notes")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_notes)
}

fn get_note(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("note" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_note)
}

fn create_note(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply , Error = warp::Rejection> + Clone {
    warp::path("note")
        .and(warp::post())
        .and(extract_json_body())
        .and(with_db(db))
        .and_then(handlers::create_note)
}

fn delete_note(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("note" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_note)
}

pub fn notes_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    root()
        .or(delete_note(db.clone()))
        .or(create_note(db.clone()))
        .or(notes_list(db.clone()))
        .or(get_note(db))
}