use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::modele::CreateNoteCommand;
use crate::service::ThoughtService;

#[derive(Deserialize)]
struct CreateNoteRequest {
    imported_at: DateTime<chrono::Utc>,
    scribe_id: Uuid,
    content: String,
}

/// ApiApp is an actor that represents the API application.
pub struct ApiApp {
    thought_service: Arc<ThoughtService>,
}

impl ApiApp {
    /// Create a new API application.
    pub fn new(thought_service: Arc<ThoughtService>) -> Self {
        Self { thought_service }
    }

    /// Get the router for the API application.
    pub fn router(&self) -> Router {
        Router::new()
            .route("/project/{project_id}/note", post(create_note))
            .with_state(self.thought_service.clone())
    }
}

/// Create a new note
async fn create_note(
    State(service): State<Arc<ThoughtService>>,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let command = CreateNoteCommand {
        project_id,
        imported_at: payload.imported_at,
        scribe_id: payload.scribe_id,
        content: payload.content,
    };

    let note = service.create_note(command).await;

    match note {
        Ok(note) => (StatusCode::CREATED, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
    }
}

async fn fetch_notes_by_project(Path(project_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(()))
}

async fn fetch_note_by_id(Path(note_id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(()))
}
