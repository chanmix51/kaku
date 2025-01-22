use crate::Result;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use unidecode::unidecode;
use uuid::Uuid;

/// Project
/// Project represents a workspace in the application.
/// It regroups Note, Thought, and other related entities.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    /// Unique identifier for the project
    pub project_id: Uuid,

    /// Reference to the universe this project belongs to
    pub universe_id: Uuid,

    /// Timestamp when the project was created
    pub created_at: DateTime<Utc>,

    /// Human readable name of the project
    pub project_name: String,

    /// URL-friendly version of the project name
    pub slug: String,

    /// Flag indicating if the project is locked for modifications
    pub locked: bool,
}

impl Project {
    /// Create a new project
    pub fn create(universe_id: Uuid, project_name: &str) -> Result<Self> {
        if project_name.trim().is_empty() {
            return Err(anyhow!("Project name cannot be empty".to_string()));
        }

        let this = Self {
            project_id: Uuid::new_v4(),
            universe_id,
            created_at: Utc::now(),
            project_name: project_name.trim().to_string(),
            slug: Self::generate_slug(project_name),
            locked: false,
        };

        Ok(this)
    }

    fn generate_slug(name: &str) -> String {
        let slug = unidecode(name)
            .trim()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>();

        // Replace multiple consecutive dashes with a single dash
        slug.split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let universe_id = Uuid::new_v4();
        let result = Project::create(universe_id, "Test Project");

        assert!(result.is_ok());

        let project = result.unwrap();

        assert_eq!(project.project_name, "Test Project");
        assert_eq!(project.slug, "test-project");
        assert!(!project.locked);
    }

    #[test]
    fn test_invalid_project_name() {
        let universe_id = Uuid::new_v4();
        let result = Project::create(universe_id, "  ");

        assert!(result.is_err());
    }

    #[test]
    fn test_slug_generation() {
        let universe_id = Uuid::new_v4();
        let project = Project::create(universe_id, "Test Project 123!@#").unwrap();

        assert_eq!(project.slug, "test-project-123");
    }

    #[test]
    fn test_slug_with_accents() {
        let universe_id = Uuid::new_v4();
        let project = Project::create(universe_id, "Ça a déjà où tête pète aïe").unwrap();

        assert_eq!(project.slug, "ca-a-deja-ou-tete-pete-aie");
    }

    #[test]
    fn test_slug_with_emojis() {
        let universe_id = Uuid::new_v4();
        let project = Project::create(universe_id, "My 📚 Project 🚀 Test 💫").unwrap();

        assert_eq!(project.slug, "my-project-test");
    }

    #[test]
    fn test_slug_with_consecutive_special_chars() {
        let universe_id = Uuid::new_v4();
        let project = Project::create(universe_id, "  Test!!!Project@#$%Test").unwrap();
        assert_eq!(project.slug, "test-project-test");

        let project = Project::create(universe_id, "Test   Project     Test").unwrap();
        assert_eq!(project.slug, "test-project-test");
    }
}
