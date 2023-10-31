use std::{convert::Infallible, collections::HashMap};

use crate::models::Note;
use crate::db::Db;

pub async fn get_notes(db: Db) -> Result<impl warp::Reply,Infallible>{

    let notes = db.lock().await;
    let notes: HashMap<i32,Note> = notes.clone();
    Ok(warp::reply::json(&notes))
}

pub async fn get_note(id: i32, db: Db) -> Result<impl warp::Reply,Infallible>{
    let notes = db.lock().await;
    let note = notes.get(&id).unwrap();
    Ok(warp::reply::json(&note))
}

pub async fn create_note(mut new_note: Note, db: Db) -> Result<impl warp::Reply,Infallible>{
    let mut notes = db.lock().await;
    let latest_id = notes.keys().max().unwrap().clone();
    new_note.note_id = latest_id+1;
    notes.insert(latest_id+1, new_note);
    Ok(warp::reply::with_status(warp::reply(), warp::http::StatusCode::CREATED))
}