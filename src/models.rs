use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug)]
pub struct AppState {
    pub tasks: Vec<Task>,
}

impl AppState {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}