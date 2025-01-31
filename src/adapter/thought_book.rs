use crate::models::{CreateThoughtCommand, Thought, ThoughtIdentifier};
use crate::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// ThoughtBook is a trait that defines the methods that are required to interact
/// with a thought database.
#[async_trait]
pub trait ThoughtBook: Sync + Send {
    /// Adds a new thought to the thought database.
    async fn add(&self, command: CreateThoughtCommand, project_id: Uuid) -> Result<Thought>;

    /// Gets a thought from the thought database.
    /// If the thought does not exist, None is returned.
    /// If the query could not be performed, an Error is raised.
    async fn get(&self, thought_id: ThoughtIdentifier) -> Result<Option<Thought>>;

    /// Syncs a thought in the thought database.
    /// The identifier cannot be updated.
    /// If the thought does not exist, an error is returned.
    async fn sync(&self, thought: Thought) -> Result<Thought>;
}

/// InMemoryThoughtBook is an in-memory implementation of the ThoughtBook trait.
/// Mostly used for testing purposes.
#[derive(Default)]
pub struct InMemoryThoughtBook {
    thoughts: Arc<RwLock<HashMap<Uuid, Thought>>>,
}

#[async_trait]
impl ThoughtBook for InMemoryThoughtBook {
    async fn add(&self, command: CreateThoughtCommand, project_id: Uuid) -> Result<Thought> {
        if let Some(parent_id) = command.parent_id {
            if !self.thoughts.read().await.contains_key(&parent_id) {
                return Err(anyhow::anyhow!("Parent thought does not exist"));
            }
        }

        let thought = Thought {
            thought_id: Uuid::new_v4(),
            parent_id: command.parent_id,
            imported_at: command.imported_at,
            scribe_id: command.scribe_id,
            project_id,
            content: command.content,
        };
        let mut thoughts = self.thoughts.write().await;
        thoughts.insert(thought.thought_id, thought.clone());

        Ok(thought)
    }

    async fn get(&self, thought_id: Uuid) -> Result<Option<Thought>> {
        Ok(self.thoughts.read().await.get(&thought_id).cloned())
    }

    async fn sync(&self, thought: Thought) -> Result<Thought> {
        let mut thoughts = self.thoughts.write().await;
        thoughts.insert(thought.thought_id, thought.clone());

        Ok(thought)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_thought_command() -> CreateThoughtCommand {
        CreateThoughtCommand {
            imported_at: Utc::now(),
            parent_id: None,
            scribe_id: Uuid::new_v4(),
            project_slug: "test-project".to_string(),
            content: "This is a test thought.".to_string(),
        }
    }

    fn create_thought() -> Thought {
        let thought_id = Uuid::new_v4();

        Thought {
            thought_id,
            parent_id: None,
            imported_at: Utc::now(),
            scribe_id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            content: "This is a test thought.".to_string(),
        }
    }

    #[tokio::test]
    async fn test_add_thought() {
        let thought_book = InMemoryThoughtBook::default();
        let command = create_test_thought_command();
        let project_id = Uuid::new_v4();
        let thought = thought_book.add(command, project_id).await.unwrap();

        assert_eq!(thought.content, "This is a test thought.");
    }

    #[tokio::test]
    async fn test_get_thought() {
        let thought_book = InMemoryThoughtBook::default();
        let thought = create_thought();
        thought_book
            .thoughts
            .write()
            .await
            .insert(thought.thought_id, thought.clone());
        let fetched_thought = thought_book
            .get(thought.thought_id)
            .await
            .unwrap()
            .expect("There must be a thought.");

        assert_eq!(fetched_thought.content, "This is a test thought.");
    }

    #[tokio::test]
    async fn test_sync_thought() {
        let thought_book = InMemoryThoughtBook::default();
        let mut thought = create_thought();
        let thought_id = thought.thought_id;
        thought_book
            .thoughts
            .write()
            .await
            .insert(thought_id, thought.clone());
        thought.content = "Updated Test Thought".to_string();
        let updated_thought = thought_book.sync(thought.clone()).await.unwrap();

        assert_eq!(updated_thought.content, "Updated Test Thought");
        assert_eq!(
            thought_book
                .thoughts
                .read()
                .await
                .get(&thought_id)
                .unwrap()
                .content,
            "Updated Test Thought"
        );
    }

    #[tokio::test]
    async fn test_add_thought_with_parent_id() {
        let thought_book = InMemoryThoughtBook::default();
        let parent_thought = create_thought();
        thought_book
            .thoughts
            .write()
            .await
            .insert(parent_thought.thought_id, parent_thought.clone());

        let command = CreateThoughtCommand {
            imported_at: Utc::now(),
            parent_id: Some(parent_thought.thought_id),
            scribe_id: Uuid::new_v4(),
            project_slug: "test-project".to_string(),
            content: "This is a child thought.".to_string(),
        };
        let project_id = Uuid::new_v4();
        let thought = thought_book.add(command, project_id).await.unwrap();

        assert_eq!(thought.content, "This is a child thought.");
        assert_eq!(thought.parent_id, Some(parent_thought.thought_id));
    }

    #[tokio::test]
    async fn test_add_thought_with_nonexistent_parent_id() {
        let thought_book = InMemoryThoughtBook::default();
        let nonexistent_parent_id = Uuid::new_v4();

        let command = CreateThoughtCommand {
            imported_at: Utc::now(),
            parent_id: Some(nonexistent_parent_id),
            scribe_id: Uuid::new_v4(),
            project_slug: "test-project".to_string(),
            content: "This is a child thought.".to_string(),
        };
        let project_id = Uuid::new_v4();
        let result = thought_book.add(command, project_id).await;

        assert!(result.is_err());
    }
}
