use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct CreateNoteCommand {
    id: Option<i32>,
    imported_at: String,
    scribe_id: Uuid,
    project_id: Option<i32>,
    content: String,
}

/// ApiApp is an actor that represents the API application.
pub struct ApiApp;

impl ApiApp {
    /// Create a new instance of the API application.
    pub fn new() -> Self {
        Self
    }

    /// Get the router for the API application.
    pub fn router(&self) -> Router {
        Router::new()
            .route("/project/{project_id}/note", post(create_note))
            .route("/project/{project_id}/notes", get(fetch_notes_by_project))
            .route(
                "/project/{project_id}/note/{note_id}",
                get(fetch_note_by_id),
            )
    }
}

async fn create_note(Json(payload): Json<CreateNoteCommand>) -> impl IntoResponse {
    // Here you would add your logic to create a note
    // For now, we just return a 201 status code
    (StatusCode::CREATED, Json(payload))
}

async fn fetch_notes_by_project(Path(project_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(()))
}

async fn fetch_note_by_id(Path(note_id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(()))
}
