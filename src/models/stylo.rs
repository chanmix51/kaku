use crate::Result;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// StyloIdentifier is a type alias for a UUID that represents a stylo identifier.
pub type StyloIdentifier = Uuid;

/// Stylo represents a right to write in the behalf of an organization given to
/// an organization member (both organizations may be the same or different).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stylo {
    /// The unique identifier of the stylo
    pub stylo_id: StyloIdentifier,

    /// The organization that owns this stylo
    pub owner_organization_id: Uuid,

    /// The organization that can use this stylo
    pub actor_organization_id: Uuid,

    /// Timestamp when the stylo was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// A human readable name for this stylo
    pub display_name: String,

    /// Whether this stylo is locked and can't be used anymore
    pub is_locked: bool,

    /// The email address associated with this stylo
    pub email: String,
}

/// Command to create a new Stylo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateStyloCommand {
    /// The organization that will own this stylo
    pub owner_organization_id: Uuid,

    /// The organization that will use this stylo
    pub actor_organization_id: Uuid,

    /// Display name for the stylo
    pub display_name: String,

    /// email address associated with the stylo
    pub email: String,
}

/// Different kinds of changes that can happen to a Stylo
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StyloChangeKind {
    /// Stylo was created
    Created,

    /// Stylo was locked
    Locked,

    /// Stylo was unlocked
    Unlocked,

    /// Stylo was revoked
    Revoked,
}

impl Stylo {
    /// Create a new stylo from a command
    pub fn create(command: CreateStyloCommand) -> Result<Self> {
        if command.display_name.trim().is_empty() {
            return Err(anyhow!("Display name cannot be empty"));
        }

        Ok(Self {
            stylo_id: Uuid::new_v4(),
            owner_organization_id: command.owner_organization_id,
            actor_organization_id: command.actor_organization_id,
            created_at: chrono::Utc::now(),
            display_name: command.display_name.trim().to_string(),
            is_locked: false,
            email: command.email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stylo_creation() {
        let command = CreateStyloCommand {
            owner_organization_id: Uuid::new_v4(),
            actor_organization_id: Uuid::new_v4(),
            display_name: "Test Stylo".to_string(),
            email: "whoever@internet.com".to_string(),
        };

        let result = Stylo::create(command);
        assert!(result.is_ok());

        let stylo = result.unwrap();
        assert_eq!(stylo.display_name, "Test Stylo");
        assert!(!stylo.is_locked);
    }

    #[test]
    fn test_invalid_display_name() {
        let command = CreateStyloCommand {
            owner_organization_id: Uuid::new_v4(),
            actor_organization_id: Uuid::new_v4(),
            display_name: "   ".to_string(),
            email: "whoever@internet.com".to_string(),
        };

        let result = Stylo::create(command);
        assert!(result.is_err());
    }

    #[test]
    fn test_same_organization() {
        let org_id = Uuid::new_v4();
        let command = CreateStyloCommand {
            owner_organization_id: org_id,
            actor_organization_id: org_id,
            display_name: "Self Owned".to_string(),
            email: "whoever@internet.com".to_string(),
        };

        let stylo = Stylo::create(command).unwrap();
        assert_eq!(stylo.owner_organization_id, stylo.actor_organization_id);
    }
}
