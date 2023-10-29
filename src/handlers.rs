use std::{convert::Infallible, collections::HashMap};

use crate::models::Note;
use crate::db::Db;

pub async fn get_note(db: Db) -> Result<impl warp::Reply,Infallible>{

    let notes = db.lock().await;
    let notes: HashMap<i32,Note> = notes.clone();
    Ok(warp::reply::json(&notes))
}