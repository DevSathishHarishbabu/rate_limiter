use std::{sync::Arc, collections::HashMap};
use tokio::sync::Mutex;
use crate::models::Note;

pub type Db = Arc<Mutex<HashMap<i32,Note>>>;

pub fn init_db() -> Db{
   
    let note1 = Note {
        note_id: 1,
        title: String::from("Hello World"),
        content: String::from("My first Note"),
    };

    let note2 = Note {
        note_id: 2,
        title: String::from("Rust Programming"),
        content: String::from("Rust is fun"),
    };

    let note3 = Note {
        note_id: 3,
        title: String::from("Hash Map"),
        content: String::from("Hash Map is part of the standard collections"),
    };
    let mut notes : HashMap< i32, Note> = HashMap :: new();

    notes.insert(note1.note_id, note1);
    notes.insert(note2.note_id, note2);
    notes.insert(note3.note_id, note3);

    Arc::new(Mutex::new(notes))
}