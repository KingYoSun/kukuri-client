use anyhow::anyhow;
use bytes::Bytes;
use futures_lite::StreamExt;
use iroh_docs::store::Query;

use crate::models::post::Post;
use crate::storage::error::{StorageError, StorageResult};
use crate::storage::state::{get_iroh_node, get_post_doc};
use crate::storage::get_default_author_with_retry;

const POST_KEY_PREFIX: &[u8] = b"post:";

/// Constructs the iroh-docs key for a post.
fn post_key(post_id: &str) -> Vec<u8> {
    [POST_KEY_PREFIX, post_id.as_bytes()].concat()
}

/// Saves or updates a post in the iroh-docs store.
///
/// This function uses the default author associated with the iroh node.
pub async fn save_post(post: &Post) -> StorageResult<()> {
    let iroh = get_iroh_node();
    let doc = get_post_doc();

    let author_id = get_default_author_with_retry(iroh).await?;

    let key = post_key(&post.id);
    let value_bytes = serde_json::to_vec(post).map_err(StorageError::Serialization)?;

    // Call set_bytes on the Doc handle
    doc.set_bytes(author_id, key, value_bytes)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    Ok(())
}

/// Retrieves a post from the iroh-docs store by post ID.
pub async fn get_post(post_id: &str) -> StorageResult<Option<Post>> {
    let iroh = get_iroh_node();
    let doc = get_post_doc();

    let key = post_key(post_id);

    // Query for the latest entry matching the exact key
    let query = Query::single_latest_per_key().key_exact(key);
    // Query on the Doc handle
    let maybe_entry = doc
        .get_one(query)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    match maybe_entry {
        Some(entry) => {
            // Check if it's a tombstone (empty content)
            if entry.content_len() == 0 {
                return Ok(None); // Treat empty entry as deleted/not found
            }

            let content_bytes = iroh
                .blobs
                .read_to_bytes(entry.content_hash())
                .await
                .map_err(|_| {
                    StorageError::NotFound(format!(
                        "Content not found for post {} (hash: {})",
                        post_id,
                        entry.content_hash()
                    ))
                })?;

            let post: Post =
                serde_json::from_slice(&content_bytes).map_err(StorageError::Serialization)?;

            Ok(Some(post))
        }
        None => Ok(None),
    }
}

/// Deletes a post by setting an empty entry (tombstone).
pub async fn delete_post(post_id: &str) -> StorageResult<()> {
    let iroh = get_iroh_node();
    let doc = get_post_doc();

    let author_id = iroh
        .authors
        .default()
        .await
        .map_err(|e| StorageError::Internal(format!("Failed to get default author: {}", e)))?;

    let key = post_key(post_id);

    // Setting empty bytes acts as a tombstone
    // Call set_bytes on the Doc handle
    doc.set_bytes(author_id, key, Bytes::new())
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    Ok(())
}

/// Lists all non-deleted posts.
/// Note: This iterates through all post keys. For large datasets, consider pagination or indexing.
pub async fn list_posts() -> StorageResult<Vec<Post>> {
    let iroh = get_iroh_node();
    let doc = get_post_doc();

    let mut posts = Vec::new();

    // Query for the latest entry for all keys starting with the prefix
    let query = Query::single_latest_per_key().key_prefix(POST_KEY_PREFIX);
    // Call get_many on the Doc handle
    let mut stream = doc
        .get_many(query)
        .await
        .map_err(|e| StorageError::Docs(anyhow!(e)))?;

    while let Some(entry_result) = stream.next().await {
        // Map the Result<Entry, RpcError> to Result<Entry, StorageError>
        let entry = entry_result.map_err(|e| StorageError::Docs(anyhow!(e)))?;

        // Skip tombstones
        if entry.content_len() == 0 {
            continue;
        }

        // Attempt to read content bytes first, mapping potential blob error
        let content_bytes_result = iroh
            .blobs
            .read_to_bytes(entry.content_hash())
            .await
            .map_err(|e| {
                // Log error here as we might skip this entry
                eprintln!(
                    "Failed to read content for post (key: {:?}, hash: {}): {}",
                    String::from_utf8_lossy(entry.key()),
                    entry.content_hash(),
                    e
                );
                // We don't return StorageError here, just log and skip
            });

        // Proceed only if reading bytes was successful
        if let Ok(content_bytes) = content_bytes_result {
            match serde_json::from_slice::<Post>(&content_bytes) {
                Ok(post) => posts.push(post),
                Err(e) => {
                    eprintln!(
                        "Failed to deserialize post content (key: {:?}): {}",
                        String::from_utf8_lossy(entry.key()),
                        e
                    );
                    // Skip this post if deserialization fails
                }
            }
        }
        // If content_bytes_result was Err, we've already logged it and just continue the loop
    }

    // Sort posts by creation time (descending, newest first)
    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(posts)
}

/// Lists all non-deleted posts by a specific author ID.
/// Note: This currently lists all posts and filters in memory.
/// A more efficient approach might involve using iroh-docs authors or indexing.
pub async fn list_user_posts(author_id_filter: &str) -> StorageResult<Vec<Post>> {
    let all_posts = list_posts().await?;
    let user_posts = all_posts
        .into_iter()
        .filter(|post| post.author_id == author_id_filter)
        .collect();
    Ok(user_posts)
}
