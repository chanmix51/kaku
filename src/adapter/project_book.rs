use crate::models::Project;
use crate::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// ProjectBookError is an error type that is used to represent errors that occur
/// when interacting with the project database.
#[derive(Debug, thiserror::Error)]
pub enum ProjectBookError {
    /// An error that occurs when a project is not found in the project database.
    #[error("Project not found: UUID='{0}'.")]
    ProjectNotFound(Uuid),
    /// An error that occurs when a project with the same slug already exists.
    #[error("Project with slug '{0}' already exists.")]
    DuplicateSlug(String),
}

/// ProjectBook is a trait that defines the methods that are required to interact
/// with a project database.
#[async_trait]
pub trait ProjectBook: Sync + Send {
    /// Creates a new project in the project database.
    async fn create(&self, universe_id: Uuid, project_name: &str) -> Result<Project>;

    /// Gets a project from the project database by its ID.
    /// If the project does not exist, an error is returned.
    async fn get(&self, project_id: &Uuid) -> Result<Project>;

    /// Gets a project from the project database by its slug.
    /// If the project does not exist, an error is returned.
    async fn get_by_slug(&self, slug: &str) -> Result<Project>;

    /// Updates a project in the project database.
    /// If the project does not exist, an error is returned.
    async fn update(&self, project: Project) -> Result<Project>;

    /// Deletes a project from the project database.
    /// If the project does not exist, an error is returned.
    async fn delete(&self, project_id: &Uuid) -> Result<Project>;

    /// Lists all projects in a universe.
    async fn list_by_universe(&self, universe_id: &Uuid) -> Result<Vec<Project>>;
}

/// InMemoryProjectBook is an in-memory implementation of the ProjectBook trait.
/// Mostly used for testing purposes.
#[derive(Default)]
pub struct InMemoryProjectBook {
    projects: Arc<RwLock<HashMap<Uuid, Project>>>,
    slugs: Arc<RwLock<HashMap<String, Uuid>>>,
}

#[async_trait]
impl ProjectBook for InMemoryProjectBook {
    async fn create(&self, universe_id: Uuid, project_name: &str) -> Result<Project> {
        let project = Project::create(universe_id, project_name)?;

        // Check for duplicate slug
        if self.slugs.read().await.contains_key(&project.slug) {
            return Err(ProjectBookError::DuplicateSlug(project.slug).into());
        }

        let mut projects = self.projects.write().await;
        let mut slugs = self.slugs.write().await;

        projects.insert(project.project_id, project.clone());
        slugs.insert(project.slug.clone(), project.project_id);

        Ok(project)
    }

    async fn get(&self, project_id: &Uuid) -> Result<Project> {
        self.projects
            .read()
            .await
            .get(project_id)
            .cloned()
            .ok_or_else(|| ProjectBookError::ProjectNotFound(*project_id).into())
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Project> {
        let slugs = self.slugs.read().await;
        let project_id = slugs
            .get(slug)
            .ok_or_else(|| ProjectBookError::ProjectNotFound(Uuid::nil()))?;

        self.get(project_id).await
    }

    async fn update(&self, project: Project) -> Result<Project> {
        let mut projects = self.projects.write().await;
        let mut slugs = self.slugs.write().await;

        // Remove old slug mapping if it exists
        if let Some(existing) = projects.get(&project.project_id) {
            slugs.remove(&existing.slug);
        }

        // Check for duplicate slug with other projects
        if let Some(existing_id) = slugs.get(&project.slug) {
            if existing_id != &project.project_id {
                return Err(ProjectBookError::DuplicateSlug(project.slug).into());
            }
        }

        slugs.insert(project.slug.clone(), project.project_id);
        projects.insert(project.project_id, project.clone());

        Ok(project)
    }

    async fn delete(&self, project_id: &Uuid) -> Result<Project> {
        let mut projects = self.projects.write().await;
        let mut slugs = self.slugs.write().await;

        let project = projects
            .remove(project_id)
            .ok_or(ProjectBookError::ProjectNotFound(*project_id))?;

        slugs.remove(&project.slug);

        Ok(project)
    }

    async fn list_by_universe(&self, universe_id: &Uuid) -> Result<Vec<Project>> {
        let projects = self.projects.read().await;

        Ok(projects
            .values()
            .filter(|p| &p.universe_id == universe_id)
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_project() {
        let book = InMemoryProjectBook::default();
        let universe_id = Uuid::new_v4();
        let project = book.create(universe_id, "Test Project").await.unwrap();

        assert_eq!(project.project_name, "Test Project");
        assert_eq!(project.slug, "test-project");
    }

    #[tokio::test]
    async fn test_get_project() {
        let book = InMemoryProjectBook::default();
        let universe_id = Uuid::new_v4();
        let created = book.create(universe_id, "Test Project").await.unwrap();
        let fetched = book.get(&created.project_id).await.unwrap();

        assert_eq!(fetched.project_name, "Test Project");
    }

    #[tokio::test]
    async fn test_get_by_slug() {
        let book = InMemoryProjectBook::default();
        let universe_id = Uuid::new_v4();
        let created = book.create(universe_id, "Test Project").await.unwrap();
        let fetched = book.get_by_slug("test-project").await.unwrap();

        assert_eq!(fetched.project_id, created.project_id);
    }

    #[tokio::test]
    async fn test_duplicate_slug() {
        let book = InMemoryProjectBook::default();
        let universe_id = Uuid::new_v4();

        let _ = book.create(universe_id, "Test Project").await.unwrap();
        let result = book.create(universe_id, "Test Project").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_by_universe() {
        let book = InMemoryProjectBook::default();
        let universe_id1 = Uuid::new_v4();
        let universe_id2 = Uuid::new_v4();

        let _ = book.create(universe_id1, "Test Project 1").await.unwrap();
        let _ = book.create(universe_id1, "Test Project 2").await.unwrap();
        let _ = book.create(universe_id2, "Test Project 3").await.unwrap();

        let projects = book.list_by_universe(&universe_id1).await.unwrap();
        assert_eq!(projects.len(), 2);
    }
}
