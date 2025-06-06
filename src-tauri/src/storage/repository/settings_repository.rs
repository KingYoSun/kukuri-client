use anyhow::{anyhow, Context};
use bytes::Bytes;
use iroh_docs::store::Query;

use crate::models::settings::Settings;
use crate::storage::error::{StorageError, StorageResult};
use crate::storage::iroh_node::SETTINGS_NAMESPACE_ID; // Will be defined later
use crate::storage::state::get_iroh_node;

const SETTINGS_KEY_PREFIX: &[u8] = b"settings:";

/// Constructs the iroh-docs key for application settings.
///
/// Uses "global" for None user_id, and the user_id otherwise.
fn settings_key(user_id: Option<&str>) -> Vec<u8> {
    match user_id {
        Some(id) => [SETTINGS_KEY_PREFIX, id.as_bytes()].concat(),
        None => [SETTINGS_KEY_PREFIX, b"global"].concat(),
    }
}

/// Saves or updates application settings in the iroh-docs store.
pub async fn save_settings(settings: &Settings) -> StorageResult<()> {
    let iroh = get_iroh_node();
    // Get the Doc handle for the settings namespace (should be imported during initialization)
    let doc = iroh
        .docs
        .open(*SETTINGS_NAMESPACE_ID)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?
        .ok_or_else(|| {
            StorageError::Internal(
                "Settings namespace not imported. Initialize iroh properly.".to_string(),
            )
        })?;

    let author_id = iroh
        .authors
        .default()
        .await
        .map_err(|e| StorageError::Internal(format!("Failed to get default author: {}", e)))?;

    let key = settings_key(settings.user_id.as_deref());
    let value_bytes = serde_json::to_vec(settings).map_err(StorageError::Serialization)?;

    doc.set_bytes(author_id, key, value_bytes)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    Ok(())
}

/// Retrieves application settings from the iroh-docs store by user ID (or global).
pub async fn get_settings(user_id: Option<&str>) -> StorageResult<Option<Settings>> {
    let iroh = get_iroh_node();
    // Get the Doc handle (should be imported during initialization)
    let doc = iroh
        .docs
        .open(*SETTINGS_NAMESPACE_ID)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?
        .ok_or_else(|| {
            StorageError::Internal(
                "Settings namespace not imported. Initialize iroh properly.".to_string(),
            )
        })?;

    let key = settings_key(user_id);

    let query = Query::single_latest_per_key().key_exact(key);
    let maybe_entry = doc
        .get_one(query)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    match maybe_entry {
        Some(entry) => {
            let content_bytes = iroh
                .blobs
                .read_to_bytes(entry.content_hash())
                .await
                .map_err(|_| {
                    StorageError::NotFound(format!(
                        "Settings content not found for user {:?} (hash: {})",
                        user_id,
                        entry.content_hash()
                    ))
                })?;

            // Handle empty content (e.g., after a delete or if saved empty)
            if content_bytes.is_empty() {
                return Ok(None); // Treat empty content as not found or deleted
            }

            let settings: Settings =
                serde_json::from_slice(&content_bytes).map_err(StorageError::Serialization)?;

            Ok(Some(settings))
        }
        None => Ok(None),
    }
}

/// Deletes application settings by setting an empty entry (tombstone).
pub async fn delete_settings(user_id: Option<&str>) -> StorageResult<()> {
    let iroh = get_iroh_node();
    let doc = match iroh
        .docs
        .open(*SETTINGS_NAMESPACE_ID)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?
    {
        Some(doc) => doc,
        None => {
            // Document doesn't exist, settings are already "deleted"
            return Ok(());
        }
    };
    let author_id = iroh
        .authors
        .default()
        .await
        .map_err(|e| StorageError::Internal(format!("Failed to get default author: {}", e)))?;

    let key = settings_key(user_id);

    // Set empty content to mark as deleted
    doc.set_bytes(author_id, key, Bytes::new())
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    Ok(())
}

// Optional: Add list_settings if needed, though likely less common than list_users/posts.
