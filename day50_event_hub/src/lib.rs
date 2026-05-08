use axum::{extract::State, response::sse::{Event,KeepAlive,Sse}, routing::{get,post}, Json, Router};
use serde::{Deserialize,Serialize};
use std::{convert::Infallible, sync::{Arc,Mutex}};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct NewEvent{ pub kind:String, pub message:String }

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct StoredEvent{ pub id:u64, pub kind:String, pub message:String }

#[derive(Clone)]
pub struct AppState{ pub events: Arc<Mutex<Vec<StoredEvent>>>, pub tx: broadcast::Sender<StoredEvent>, pub next_id: Arc<Mutex<u64>> }

impl Default for AppState{
    fn default()->Self{
        let (tx,_) = broadcast::channel(256);
        Self{ events:Arc::new(Mutex::new(Vec::new())), tx, next_id:Arc::new(Mutex::new(1)) }
    }
}

async fn health()->&'static str{ "ok" }

async fn post_event(State(st): State<AppState>, Json(ne): Json<NewEvent>) -> Json<StoredEvent> {
    let mut idg = st.next_id.lock().unwrap();
    let id = *idg; *idg += 1;
    let ev = StoredEvent{ id, kind: ne.kind, message: ne.message };
    st.events.lock().unwrap().push(ev.clone());
    let _ = st.tx.send(ev.clone());
    Json(ev)
}

async fn list_events(State(st): State<AppState>) -> Json<Vec<StoredEvent>> {
    Json(st.events.lock().unwrap().clone())
}

async fn stream(State(st): State<AppState>) -> Sse<impl futures_core::Stream<Item=Result<Event, Infallible>>> {
    let rx = st.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(ev) => {
                let data = serde_json::to_string(&ev).unwrap();
                Some(Ok(Event::default().data(data)))
            }
            Err(_) => None,
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub fn app()->Router{
    let st = AppState::default();
    Router::new()
        .route("/health", get(health))
        .route("/events", post(post_event).get(list_events))
        .route("/stream", get(stream))
        .with_state(st)
}
