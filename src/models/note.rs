use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// NoteIdentifier is a type alias for a UUID that represents a note identifier.
pub type NoteIdentifier = Uuid;

/// Note is a domain model that represents a note.
/// A note is a piece of information that is written by a stylo.
/// Notes are intended to be short term and are used to capture information.
/// The note is associated with a project.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    /// The unique identifier of the note.
    pub note_id: NoteIdentifier,

    /// The date and time the note was imported.
    pub imported_at: DateTime<chrono::Utc>,

    /// The unique identifier of the stylo that created the note.
    pub stylo_id: Uuid,

    /// The unique identifier of the project that the note is associated with.
    pub project_id: Uuid,

    /// The content of the note.
    pub content: String,
}

/// CreateNoteCommand is a command that is used to create a new note.
#[derive(Serialize, Deserialize)]
pub struct CreateNoteCommand {
    /// The date and time the note was imported.
    pub imported_at: DateTime<chrono::Utc>,

    /// The unique identifier of the stylo that created the note.
    pub stylo_id: Uuid,

    /// The unique identifier of the project that the note is associated with.
    pub project_slug: String,

    /// The content of the note.
    pub content: String,
}

/// Business changes on the Note model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteChangeKind {
    /// Note created
    Created,

    /// Note scratched
    Scratched,
}
