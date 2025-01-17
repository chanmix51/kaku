#![deny(missing_docs)]
//! # Kaku internal library.
//!
//! This library contains the internal code of Kaku. It is composed of the following modules:
//! * `actor`: Contains the actors that are used to handle the business logic of the application.
//! * `adapter`: Contains the adapters that are used to interact with the external services.
//! * `modele`: Contains the models that are used to represent the data structures of the application.

/// Actor module.
pub mod actor;

/// Adapter module.
pub mod adapter;

/// Modele module.
pub mod modele;

/// Service module.
pub mod service;

/// Result type used in the application.
pub type Result<T> = anyhow::Result<T>;
