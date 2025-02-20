use axum::{extract::State, Json, http::StatusCode, extract::Path};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::models::{Task, AppState};

pub async fn list_tasks(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<Task>> {
    let state = state.lock().await;
    Json(state.tasks.clone())
}

pub async fn add_task(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Task>,
) -> (StatusCode, Json<Task>) {
    let mut state = state.lock().await;
    let new_task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
    };
    state.tasks.push(new_task.clone());
    (StatusCode::CREATED, Json(new_task))
}

pub async fn delete_task(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut state = state.lock().await;
    if let Some(pos) = state.tasks.iter().position(|task| task.id == id) {
        state.tasks.remove(pos);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}