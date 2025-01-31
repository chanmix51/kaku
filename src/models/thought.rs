use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ThoughtIdentifier is a type alias for a UUID that represents a thought identifier.
pub type ThoughtIdentifier = Uuid;

/// Thought is a domain model that represents a thought.
/// A thought is a piece of information that is written by a scribe.
/// Thoughts are intended to be long term and are used to capture information.
/// The thought is associated with a project.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thought {
    /// The unique identifier of the thought.
    pub thought_id: ThoughtIdentifier,

    /// Thought may be chained to another thought.
    pub parent_id: Option<ThoughtIdentifier>,

    /// The date and time the thought was imported.
    pub imported_at: DateTime<chrono::Utc>,

    /// The unique identifier of the scribe that created the thought.
    pub scribe_id: Uuid,

    /// The unique identifier of the project that the thought is associated with.
    pub project_id: Uuid,

    /// The content of the thought.
    pub content: String,
}

/// CreateThoughtCommand is a command that is used to create a new thought.
#[derive(Serialize, Deserialize)]
pub struct CreateThoughtCommand {
    /// The date and time the thought was imported.
    pub imported_at: DateTime<chrono::Utc>,

    /// Eventual parent of the thought
    pub parent_id: Option<ThoughtIdentifier>,

    /// The unique identifier of the scribe that created the thought.
    pub scribe_id: Uuid,

    /// The unique identifier of the project that the thought is associated with.
    pub project_slug: String,

    /// The content of the thought.
    pub content: String,
}

/// Business changes on the Thought model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThoughtChangeKind {
    /// Thought created
    Created,

    /// Thought disputed
    Disputed(ThoughtIdentifier),
}
