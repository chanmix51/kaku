use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::post, Router};
use chrono::DateTime;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::CreateNoteCommand;
use crate::service::ThoughtService;

/// Request payload for creating a new note.
/// This represents the JSON body that clients should send when creating a note.
/// The project_id is not included here as it's provided in the URL path.
#[derive(Deserialize)]
struct CreateNoteRequest {
    /// The date and time when the note was imported into the system.
    /// Format: ISO 8601 UTC datetime
    pub imported_at: DateTime<chrono::Utc>,

    /// The unique identifier of the scribe (user) creating the note.
    /// Format: UUID v4
    pub scribe_id: Uuid,

    /// The content of the note.
    /// This contains the actual text/information of the note.
    pub content: String,
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
            .route("/project/{project_slug}/note", post(create_note))
            .with_state(self.thought_service.clone())
    }
}

/// Create a new note
async fn create_note(
    State(service): State<Arc<ThoughtService>>,
    Path(project_slug): Path<String>,
    Json(payload): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let command = CreateNoteCommand {
        project_slug,
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
