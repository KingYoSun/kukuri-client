use anyhow::anyhow;
use bytes::Bytes;
use iroh_docs::store::Query;

use crate::models::user::User;
use crate::storage::error::{StorageError, StorageResult};
use crate::storage::state::{get_iroh_node, get_user_doc};

const USER_PROFILE_KEY_PREFIX: &[u8] = b"user_profile:";

/// Constructs the iroh-docs key for a user profile.
fn user_profile_key(user_id: &str) -> Vec<u8> {
    [USER_PROFILE_KEY_PREFIX, user_id.as_bytes()].concat()
}

/// Saves or updates a user profile in the iroh-docs store.
///
/// This function uses the default author associated with the iroh node.
pub async fn save_user(user: &User) -> StorageResult<()> {
    let iroh = get_iroh_node();
    let doc = get_user_doc();

    let author_id = iroh
        .authors
        .default()
        .await
        .map_err(|e| StorageError::Internal(format!("Failed to get default author: {}", e)))?;

    let key = user_profile_key(&user.id);
    let value_bytes = serde_json::to_vec(user).map_err(StorageError::Serialization)?;

    // Call set_bytes on the Doc handle
    doc.set_bytes(author_id, key, value_bytes)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    Ok(())
}

/// Retrieves a user profile from the iroh-docs store by user ID.
pub async fn get_user(user_id: &str) -> StorageResult<Option<User>> {
    let iroh = get_iroh_node();
    let doc = get_user_doc();

    let key = user_profile_key(user_id);

    // Query on the Doc handle
    let query = Query::single_latest_per_key().key_exact(key);
    let maybe_entry = doc
        .get_one(query)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    match maybe_entry {
        Some(entry) => {
            // Entry found, now get the content bytes from the blobs store
            let content_bytes = iroh
                .blobs
                .read_to_bytes(entry.content_hash())
                .await
                .map_err(|e| {
                    // Handle case where entry exists but content is missing (should ideally not happen with default settings)
                    StorageError::NotFound(format!(
                        "Content not found for user {} (hash: {})",
                        user_id,
                        entry.content_hash()
                    ))
                })?;

            // Deserialize the bytes into a User struct
            let user: User =
                serde_json::from_slice(&content_bytes).map_err(StorageError::Serialization)?;

            Ok(Some(user))
        }
        None => {
            // No entry found for the key
            Ok(None)
        }
    }
}

/// Deletes a user profile by setting an empty entry (tombstone).
/// Note: This performs a soft delete by overwriting with an empty record.
/// Consider if a hard delete (`docs.del`) is more appropriate depending on requirements.
pub async fn delete_user(user_id: &str) -> StorageResult<()> {
    let iroh = get_iroh_node();
    let doc = get_user_doc();

    let author_id = iroh
        .authors
        .default()
        .await
        .map_err(|e| StorageError::Internal(format!("Failed to get default author: {}", e)))?;

    let key = user_profile_key(user_id);

    // Call set_bytes on the Doc handle
    doc.set_bytes(author_id, key, Bytes::new()) // Set empty content
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    Ok(())
}

// TODO: Add functions for listing users, potentially using Query::prefix()
// pub async fn list_users() -> StorageResult<Vec<User>> { ... }
