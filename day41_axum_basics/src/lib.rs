use axum::{routing::{get,post}, Json, Router};
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Echo{ pub msg:String }

async fn health()->&'static str { "ok" }
async fn echo(Json(payload): Json<Echo>) -> Json<Echo> { Json(payload) }

pub fn app()->Router {
    Router::new().route("/health", get(health)).route("/echo_json", post(echo))
}
