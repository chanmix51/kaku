use synapps::Event;

/// Type of model modification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    /// a new model was created
    Create,

    /// a model was updated
    Update,

    /// a model was deleted
    Delete,
}

/// Type of model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelKind {
    /// a note model
    Note,

    /// a project model
    Project,
}
/// Model event structure
/// This sprays model changes to all actors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEvent {
    /// type of model modification
    pub kind: EventKind,

    /// type of model
    pub model: ModelKind,

    /// model id
    pub id: uuid::Uuid,

    /// model modification timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for ModelEvent {}
