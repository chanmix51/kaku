use synapps::Event;
use uuid::Uuid;

use super::{NoteChangeKind, ProjectChangeKind};

/// Type of model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelKind {
    /// a note model
    Note {
        /// note identifier
        note_id: Uuid,

        /// project identifier
        /// This is the project the note is associated with.
        project_id: Uuid,

        /// change kind
        change_kind: NoteChangeKind,
    },

    /// a project model
    Project {
        /// project identifier
        project_id: Uuid,

        /// Organization identifier
        universe_id: Uuid,

        /// change kind
        change_kind: ProjectChangeKind,
    },
}
/// Model event structure
/// This sprays model changes to all actors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEvent {
    /// type of model
    pub model: ModelKind,

    /// model modification timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for ModelEvent {}
