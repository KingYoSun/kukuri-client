//! Storage layer implementation using iroh.

mod error;
pub mod events;
mod iroh_node;
pub mod state; // Make state public for initialization in lib.rs
pub mod traits; // Make traits module public // Add events module for document subscription

pub use events::DocumentSubscriptionService;
pub use traits::{HasId, PostEntry}; // Re-export the traits // Re-export DocumentSubscriptionService

pub use error::{StorageError, StorageResult};
// Re-export clients for easier access if needed elsewhere, though direct use might be discouraged
// pub use iroh_node::{AuthorsClient, BlobsClient, DocsClient, IrohNode};

pub mod repository;
// Placeholder for future modules like models, etc.
// pub mod repository;
// pub mod models;
