use std::convert::Infallible;
use warp::{self,Filter};

use crate::db::Db;
use crate::handlers;

fn with_db(db: Db) -> impl Filter<Extract = (Db,),Error= Infallible> + Clone{
    warp::any().map(move || db.clone())
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
        .and_then(handlers::get_note)
}

pub fn notes_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    root()
        .or(notes_list(db))
}