use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::delete;
use axum::Json;
use axum::{routing::post, Router};
use chrono::DateTime;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{CreateNoteCommand, CreateProjectCommand, CreateThoughtCommand};
use crate::service::{ThoughtService, ThoughtServiceError};

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

/// Request payload for creating a new project.
#[derive(Deserialize)]
struct CreateProjectRequest {
    /// The name of the project.
    pub project_name: String,

    /// The universe identifier.
    pub universe_id: Uuid,
}

/// Request payload for creating a new thought.
#[derive(Deserialize)]
struct CreateThoughtRequest {
    pub imported_at: DateTime<chrono::Utc>,
    pub scribe_id: Uuid,
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
            .route("/project/{project_slug}/thought", post(create_thought))
            .route("/project/create", post(create_project))
            .route("/notes/{note_id}", delete(scratch_note))
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
        Ok(note) => {
            let headers = [(
                axum::http::header::LOCATION,
                format!("/note/{}", note.note_id),
            )];
            (StatusCode::CREATED, headers, Json(()))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(axum::http::header::LOCATION, "".to_string())],
            Json(()),
        ),
    }
}

/// Create a new thought
async fn create_thought(
    State(service): State<Arc<ThoughtService>>,
    Path(project_slug): Path<String>,
    Json(payload): Json<CreateThoughtRequest>,
) -> impl IntoResponse {
    let command = CreateThoughtCommand {
        project_slug,
        imported_at: payload.imported_at,
        scribe_id: payload.scribe_id,
        content: payload.content,
        parent_id: None,
    };

    let thought = service.create_thought(command).await;

    match thought {
        Ok(thought) => {
            let headers = [(
                axum::http::header::LOCATION,
                format!("/thought/{}", thought.thought_id),
            )];
            (StatusCode::CREATED, headers, Json(()))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(axum::http::header::LOCATION, String::new())],
            Json(()),
        ),
    }
}

/// Create a new project
async fn create_project(
    State(service): State<Arc<ThoughtService>>,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    let command = CreateProjectCommand {
        project_name: payload.project_name.clone(),
        universe_id: payload.universe_id,
    };

    let result = service.create_project(command).await;

    match result {
        Ok(project) => {
            let headers = [(
                axum::http::header::LOCATION,
                format!("/project/{}", project.slug),
            )];
            (StatusCode::CREATED, headers, Json(()))
        }
        Err(e)
            if matches!(
                e.downcast_ref::<ThoughtServiceError>(),
                Some(ThoughtServiceError::ProjectAlreadyExists(_))
            ) =>
        {
            (
                StatusCode::CONFLICT,
                [(axum::http::header::LOCATION, String::new())],
                Json(()),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(axum::http::header::LOCATION, String::new())],
            Json(()),
        ),
    }
}

/// Scratch a note by its ID
async fn scratch_note(
    State(service): State<Arc<ThoughtService>>,
    Path(note_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = service.scratch_note(note_id).await;

    match result {
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(e)
            if matches!(
                e.downcast_ref::<ThoughtServiceError>(),
                Some(ThoughtServiceError::NoteNotFound(_))
            ) =>
        {
            (StatusCode::NOT_FOUND, Json(()))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
    }
}
