use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::adapter::{NoteBook, ProjectBook};
use crate::models::{CreateNoteCommand, ModelEvent, Note};
use crate::Result;

/// Thought service
pub struct ThoughtService {
    note_book: Arc<dyn NoteBook>,
    project_book: Arc<dyn ProjectBook>,
    sender: UnboundedSender<ModelEvent>,
}

impl ThoughtService {
    /// Create a new thought service
    pub fn new(
        note_book: Arc<dyn NoteBook>,
        project_book: Arc<dyn ProjectBook>,
        sender: UnboundedSender<ModelEvent>,
    ) -> Self {
        Self {
            note_book,
            project_book,
            sender,
        }
    }

    /// Create a new note
    pub async fn create_note(&self, command: CreateNoteCommand) -> Result<Note> {
        let project_id = Uuid::new_v4();
        self.note_book.add(command, project_id).await
    }

    /// Scratch a note
    pub async fn scratch_note(&self, note_id: uuid::Uuid) -> Result<Note> {
        self.note_book.delete(note_id).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::Container;

    use super::*;

    #[tokio::test]
    async fn test_create_note_success() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();

        let command = CreateNoteCommand {
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_slug: String::from("test-project"),
            content: "This is a test note.".to_string(),
        };

        let note = thought_service.create_note(command).await.unwrap();

        assert_eq!(note.content, "This is a test note.");
    }

    #[tokio::test]
    async fn test_scratch_note_success() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();

        let command = CreateNoteCommand {
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_slug: String::from("test-project"),
            content: "This is a test note.".to_string(),
        };

        let note = thought_service.create_note(command).await.unwrap();
        let note_id = note.note_id;

        thought_service.scratch_note(note_id).await.unwrap();

        let result = thought_service.note_book.get(note_id).await;
        assert!(result.is_err());
    }
}
