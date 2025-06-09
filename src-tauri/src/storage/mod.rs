//! Storage layer implementation using iroh.

mod error;
pub mod events;
pub mod iroh_node; // Make iroh_node public for tests
pub mod state; // Make state public for initialization in lib.rs
pub mod traits; // Make traits module public // Add events module for document subscription

pub use events::DocumentSubscriptionService;
pub use iroh_node::IrohNode; // Re-export IrohNode for tests
pub use traits::{HasId, PostEntry}; // Re-export the traits // Re-export DocumentSubscriptionService

pub use error::{StorageError, StorageResult};
// Re-export clients for easier access if needed elsewhere, though direct use might be discouraged
// pub use iroh_node::{AuthorsClient, BlobsClient, DocsClient, IrohNode};

pub mod repository;
// Placeholder for future modules like models, etc.
// pub mod repository;
// pub mod models;

/// Helper function to get the default author with retry logic
/// This is especially useful in tests where the RPC connection might not be immediately available
#[allow(dead_code)]
pub(crate) async fn get_default_author_with_retry(
    node: &iroh_node::IrohNode,
) -> Result<iroh_docs::AuthorId, StorageError> {
    let mut attempts = 0;
    let max_attempts = if cfg!(test) { 10 } else { 3 };
    
    loop {
        match node.authors.default().await {
            Ok(author) => {
                #[cfg(test)]
                println!("[AUTHOR] Successfully got default author on attempt {}", attempts + 1);
                return Ok(author);
            },
            Err(e) => {
                attempts += 1;
                #[cfg(test)]
                println!("[AUTHOR] Failed attempt {} to get default author: {}", attempts, e);
                
                if attempts >= max_attempts {
                    return Err(StorageError::Internal(format!(
                        "Failed to get default author after {} attempts: {}",
                        attempts, e
                    )));
                }
                // Exponential backoff for retries
                let delay = std::cmp::min(100 * 2_u64.pow(attempts as u32 - 1), 1000);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
        }
    }
}
