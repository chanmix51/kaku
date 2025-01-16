use crate::modele::{CreateNoteCommand, Note};
use crate::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// NoteBookError is an error type that is used to represent errors that occur
/// when interacting with the note database.
#[derive(Debug, thiserror::Error)]
pub enum NoteBookError {
    /// An error that occurs when a note is not found in the note database.
    #[error("Note not found: UUID='{0}'.")]
    NoteNotFound(Uuid),
}

/// NoteBook is a trait that defines the methods that are required to interact
/// with a note database.
#[async_trait]
pub trait NoteBook: Sync + Send {
    /// Adds a new note to the note database.
    async fn add(&self, command: CreateNoteCommand) -> Result<Note>;

    /// Gets a note from the note database.
    /// If the note does not exist, an error is returned.
    async fn get(&self, note_id: Uuid) -> Result<Note>;

    /// Syncs a note in the note database.
    /// If the note does not exist, an error is returned.
    async fn sync(&self, note: Note) -> Result<Note>;

    /// Deletes a note from the note database.
    /// If the note does not exist, an error is returned.
    async fn delete(&self, note_id: Uuid) -> Result<Note>;
}

/// InMemoryNoteBook is an in-memory implementation of the NoteBook trait.
/// Mostly used for testing purposes.
#[derive(Default)]
pub struct InMemoryNoteBook {
    notes: Arc<RwLock<HashMap<Uuid, Note>>>,
}

#[async_trait]
impl NoteBook for InMemoryNoteBook {
    async fn add(&self, command: CreateNoteCommand) -> Result<Note> {
        let note = Note {
            note_id: Uuid::new_v4(),
            imported_at: command.imported_at,
            scribe_id: command.scribe_id,
            project_id: command.project_id,
            content: command.content,
        };
        let mut notes = self.notes.write().await;
        notes.insert(note.note_id, note.clone());

        Ok(note)
    }

    async fn get(&self, note_id: Uuid) -> Result<Note> {
        self.notes
            .read()
            .await
            .get(&note_id)
            .cloned()
            .ok_or_else(|| NoteBookError::NoteNotFound(note_id).into())
    }

    async fn sync(&self, note: Note) -> Result<Note> {
        let mut notes = self.notes.write().await;
        notes.insert(note.note_id, note.clone());

        Ok(note)
    }

    async fn delete(&self, note_id: Uuid) -> Result<Note> {
        self.notes
            .write()
            .await
            .remove(&note_id)
            .ok_or_else(|| NoteBookError::NoteNotFound(note_id).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_note_command() -> CreateNoteCommand {
        CreateNoteCommand {
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            content: "This is a test note.".to_string(),
        }
    }

    fn create_note() -> Note {
        let note_id = Uuid::new_v4();

        Note {
            note_id,
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            content: "This is a test note.".to_string(),
        }
    }

    #[tokio::test]
    async fn test_add_note() {
        let notebook = InMemoryNoteBook::default();
        let command = create_test_note_command();
        let result = notebook.add(command);
        let note = result.await.unwrap();

        assert_eq!(note.content, "This is a test note.");
    }

    #[tokio::test]
    async fn test_get_note() {
        let notebook = InMemoryNoteBook::default();
        let note = create_note();
        notebook
            .notes
            .write()
            .await
            .insert(note.note_id, note.clone());
        let fetched_note = notebook.get(note.note_id).await.unwrap();

        assert_eq!(fetched_note.content, "This is a test note.");
    }

    #[tokio::test]
    async fn test_sync_note() {
        let notebook = InMemoryNoteBook::default();
        let mut note = create_note();
        let note_id = note.note_id;
        notebook.notes.write().await.insert(note_id, note.clone());
        note.content = "Updated Test Note".to_string();
        let updated_note = notebook.sync(note.clone()).await.unwrap();

        assert_eq!(updated_note.content, "Updated Test Note");
        assert_eq!(
            notebook.notes.read().await.get(&note_id).unwrap().content,
            "Updated Test Note"
        );
    }

    #[tokio::test]
    async fn test_delete_note() {
        let notebook = InMemoryNoteBook::default();
        let note = create_note();
        let note_id = note.note_id;
        notebook.notes.write().await.insert(note_id, note.clone());
        let deleted_note = notebook.delete(note_id).await.unwrap();

        assert_eq!(deleted_note.content, "This is a test note.");
        assert!(notebook.notes.read().await.get(&note_id).is_none());
    }
}
