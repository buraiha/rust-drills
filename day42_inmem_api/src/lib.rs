use axum::{extract::{Path,State}, routing::{get,post}, Json, Router};
use serde::{Deserialize,Serialize};
use std::{collections::HashMap, sync::{Arc,Mutex}};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Item{ pub id:String, pub value:String }

#[derive(Clone,Default)]
pub struct AppState{ pub db: Arc<Mutex<HashMap<String, Item>>> }

async fn list(State(st): State<AppState>) -> Json<Vec<Item>> {
    let db=st.db.lock().unwrap();
    Json(db.values().cloned().collect())
}

async fn get_one(State(st): State<AppState>, Path(id): Path<String>) -> Option<Json<Item>> {
    let db=st.db.lock().unwrap();
    db.get(&id).cloned().map(Json)
}

async fn put(State(st): State<AppState>, Json(item): Json<Item>) -> Json<Item> {
    let mut db=st.db.lock().unwrap();
    db.insert(item.id.clone(), item.clone());
    Json(item)
}

pub fn app()->Router {
    let st=AppState::default();
    Router::new().route("/items", get(list).post(put)).route("/items/:id", get(get_one)).with_state(st)
}
