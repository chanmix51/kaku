use std::sync::Arc;

use thiserror::Error;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::adapter::{NoteBook, ProjectBook};
use crate::models::{
    CreateNoteCommand, CreateProjectCommand, ModelEvent, ModelKind, Note, NoteChangeKind, Project,
    ProjectChangeKind,
};
use crate::Result;

/// ThoughtServiceError
/// Different errors returned by the ThoughtService.
#[derive(Debug, Error)]
pub enum ThoughtServiceError {
    /// Project not found
    #[error("There is not project with slug '{0}'.")]
    ProjectNotFound(String),

    /// Note not found
    #[error("There is no note with noted_id='{0}'.")]
    NoteNotFound(Uuid),

    /// Project already exists
    #[error("Project with slug '{0}' already exists.")]
    ProjectAlreadyExists(String),

    /// Universe not found
    #[error("Universe not found.")]
    UniverseNotFound,
}

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

    /// Create a new note.
    ///
    /// The project pointed by the slug must exist since the slugification is a
    /// surjective function it is not possible to deduce the project name from
    /// the slug. An error is raised if the project does not exist.
    pub async fn create_note(&self, command: CreateNoteCommand) -> Result<Note> {
        let project = self
            .project_book
            .get_by_slug(&command.project_slug)
            .await?
            .ok_or_else(|| ThoughtServiceError::ProjectNotFound(command.project_slug.clone()))?;

        let note = self.note_book.add(command, project.project_id).await?;

        self.sender.send(ModelEvent {
            model: ModelKind::Note {
                note_id: note.note_id,
                project_id: note.project_id,
                change_kind: NoteChangeKind::Created,
            },
            timestamp: chrono::Utc::now(),
        })?;

        Ok(note)
    }

    /// Scratch a note.
    ///
    /// An error is raised if the Note does not exist.
    pub async fn scratch_note(&self, note_id: uuid::Uuid) -> Result<Note> {
        let note = self
            .note_book
            .delete(note_id)
            .await?
            .ok_or(ThoughtServiceError::NoteNotFound(note_id))?;

        self.sender.send(ModelEvent {
            model: ModelKind::Note {
                note_id: note.note_id,
                project_id: note.project_id,
                change_kind: NoteChangeKind::Scratched,
            },
            timestamp: chrono::Utc::now(),
        })?;

        Ok(note)
    }

    /// Create a Project
    /// This returns an error if the project already exists.
    /// This returns an error if the universe does not exist.
    pub async fn create_project(&self, command: CreateProjectCommand) -> Result<()> {
        let slug = Project::generate_slug(&command.project_name);

        if self
            .project_book
            .get_by_slug(&Project::generate_slug(&slug))
            .await?
            .is_some()
        {
            return Err(ThoughtServiceError::ProjectAlreadyExists(slug).into());
        }

        let project = self.project_book.create(command).await?;

        self.sender.send(ModelEvent {
            model: ModelKind::Project {
                project_id: project.project_id,
                universe_id: project.universe_id,
                change_kind: ProjectChangeKind::Created,
            },
            timestamp: chrono::Utc::now(),
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{models::ProjectChangeKind, Container};

    use super::*;

    #[tokio::test]
    async fn test_create_note_success_project_not_exist() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();
        let mut receiver = container.event_publisher_receiver().unwrap();
        container.destroy();

        let command = CreateNoteCommand {
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_slug: String::from("test-project"),
            content: "This is a test note.".to_string(),
        };

        let error = thought_service
            .create_note(command)
            .await
            .unwrap_err()
            .downcast::<ThoughtServiceError>()
            .expect("Expected ThoughtServiceError");

        assert!(matches!(error, ThoughtServiceError::ProjectNotFound(_)));

        // check that the event was not sent
        assert!(receiver.try_recv().is_err());
    }

    #[tokio::test]
    async fn test_create_note_success_project_exist() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();
        let project_book = container.project_book().unwrap();
        let mut receiver = container.event_publisher_receiver().unwrap();
        container.destroy();

        let project_command = crate::models::CreateProjectCommand {
            universe_id: Uuid::new_v4(),
            project_name: "Test Project".to_string(),
        };
        let project = project_book.create(project_command).await.unwrap();

        let command = CreateNoteCommand {
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_slug: project.slug,
            content: "This is a test note.".to_string(),
        };

        let note = thought_service.create_note(command).await.unwrap();

        assert_eq!(note.content, "This is a test note.");
        assert_eq!(note.project_id, project.project_id);

        // check that the event was sent
        let event = receiver.recv().await.unwrap();
        assert_eq!(
            event.model,
            ModelKind::Note {
                note_id: note.note_id,
                project_id: note.project_id,
                change_kind: NoteChangeKind::Created,
            }
        );
    }

    #[tokio::test]
    async fn test_scratch_note_success() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();
        let note_book = container.note_book().unwrap();
        let command = CreateNoteCommand {
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_slug: String::from("test-project"),
            content: "This is a test note.".to_string(),
        };
        let note = note_book.add(command, Uuid::new_v4()).await.unwrap();
        let note_id = note.note_id;
        let mut receiver = container.event_publisher_receiver().unwrap();
        container.destroy();

        let note = thought_service.scratch_note(note_id).await.unwrap();

        // Check that the note was scratched and is not available anymore
        assert!(note_book.get(note_id).await.unwrap().is_none());

        // check that the event was sent
        let event = receiver.recv().await.unwrap();
        assert_eq!(
            event.model,
            ModelKind::Note {
                note_id,
                project_id: note.project_id,
                change_kind: NoteChangeKind::Scratched,
            }
        );
    }

    #[tokio::test]
    async fn test_create_project_success() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();
        let project_book = container.project_book().unwrap();
        let mut receiver = container.event_publisher_receiver().unwrap();
        container.destroy();

        let command = CreateProjectCommand {
            universe_id: Uuid::new_v4(),
            project_name: "New Project".to_string(),
        };

        thought_service.create_project(command).await.unwrap();

        let project = project_book
            .get_by_slug("new-project")
            .await
            .unwrap()
            .expect("there should be a project");
        assert_eq!(project.project_name, "New Project");

        // check that the event was sent
        let event = receiver.recv().await.unwrap();
        assert_eq!(
            event.model,
            ModelKind::Project {
                project_id: project.project_id,
                universe_id: project.universe_id,
                change_kind: ProjectChangeKind::Created,
            }
        );
    }

    #[tokio::test]
    async fn test_create_project_error_project_already_exists() {
        let mut container = Container::default();
        let thought_service = container.thought_service().unwrap();
        let project_book = container.project_book().unwrap();
        container.destroy();

        let command = CreateProjectCommand {
            universe_id: Uuid::new_v4(),
            project_name: "Existing Project".to_string(),
        };

        // Create the project first
        project_book.create(command.clone()).await.unwrap();

        // Try to create the same project again
        let error = thought_service
            .create_project(command)
            .await
            .unwrap_err()
            .downcast::<ThoughtServiceError>()
            .expect("Expected ThoughtServiceError");

        assert!(matches!(
            error,
            ThoughtServiceError::ProjectAlreadyExists(_)
        ));
    }
}
