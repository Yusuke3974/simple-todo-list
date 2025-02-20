use axum::{Router, routing::{get, post, delete}};
use tokio::sync::Mutex;
use std::sync::Arc;

mod routes;
mod models;

use routes::{list_tasks, add_task, delete_task};
use models::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(add_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(state);

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
