use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Clone,Debug,Deserialize,Serialize,Default)]
pub struct Note{
    pub note_id: i32,
    pub title: String,
    pub content: String,
}