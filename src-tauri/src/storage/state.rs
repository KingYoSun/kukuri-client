use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use tauri::Manager; // Import Manager trait for AppHandle::manage

use super::error::StorageError;
use super::iroh_node::{IrohNode, POST_NAMESPACE_ID, SETTINGS_NAMESPACE_ID, USER_NAMESPACE_ID};

// Global static variable to hold the initialized IrohNode.
// OnceLock ensures it's initialized only once safely across threads.
static IROH_NODE: OnceLock<IrohNode> = OnceLock::new();

/// Initializes the Iroh node and stores it in the global static variable.
/// This should be called once during Tauri's setup phase.
///
/// `app_handle`: The Tauri AppHandle, used to resolve the app's data directory.
pub async fn initialize_iroh<R: tauri::Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> Result<(), StorageError> {
    if IROH_NODE.get().is_some() {
        // Already initialized
        return Ok(());
    }

    // Resolve the application data directory using the Manager trait
    // Resolve the application data directory using the Manager trait
    // app_data_dir() returns Result<PathBuf, tauri::Error>
    let base_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| StorageError::Internal(format!("Failed to get app_data_dir: {}", e)))?; // Map Tauri error to StorageError
    let data_root = base_dir.join("iroh_data"); // Use a subdirectory for iroh data

    // Create the IrohNode instance
    let node = IrohNode::new(data_root).await?;

    // Import the required namespaces for the application
    import_required_namespaces(&node).await?;

    // Attempt to set the node in the OnceLock.
    // This will fail if another thread initialized it concurrently, which is fine.
    let _ = IROH_NODE.set(node);

    Ok(())
}

/// Retrieves a reference to the initialized IrohNode.
/// Panics if the node has not been initialized yet (call `initialize_iroh` first).
pub fn get_iroh_node() -> &'static IrohNode {
    IROH_NODE
        .get()
        .expect("Iroh node has not been initialized. Call initialize_iroh during setup.")
}

/// Retrieves a clone of the initialized IrohNode.
/// Panics if the node has not been initialized yet.
/// Useful for passing to Tauri command handlers that need ownership or longer lifetimes.
#[allow(dead_code)] // Might be useful later
pub fn clone_iroh_node() -> IrohNode {
    get_iroh_node().clone()
}

/// Test-only function to initialize the global IROH_NODE with a provided node
/// This is needed for integration tests that create their own IrohNode instances
#[cfg(test)]
pub async fn initialize_iroh_for_tests(node: IrohNode) -> Result<(), StorageError> {
    if IROH_NODE.get().is_some() {
        // Already initialized
        return Ok(());
    }

    // Import the required namespaces for tests
    import_required_namespaces(&node).await?;

    let _ = IROH_NODE.set(node);
    Ok(())
}

/// Import the required namespaces for the application.
/// This ensures that the namespaces are available for document operations.
async fn import_required_namespaces(node: &IrohNode) -> Result<(), StorageError> {
    use iroh_docs::sync::Capability;

    // For write access, we need to create documents instead of importing existing ones
    // with fixed NamespaceIds. Create new documents with write capability.

    // Try to get existing documents first, create if they don't exist
    let user_doc = match node.docs.open(*USER_NAMESPACE_ID).await {
        Ok(Some(doc)) => doc,
        Ok(None) | Err(_) => {
            // Create a new document for users
            node.docs.create().await.map_err(|e| {
                StorageError::Internal(format!("Failed to create user document: {}", e))
            })?
        }
    };

    let post_doc = match node.docs.open(*POST_NAMESPACE_ID).await {
        Ok(Some(doc)) => doc,
        Ok(None) | Err(_) => {
            // Create a new document for posts
            node.docs.create().await.map_err(|e| {
                StorageError::Internal(format!("Failed to create post document: {}", e))
            })?
        }
    };

    let settings_doc = match node.docs.open(*SETTINGS_NAMESPACE_ID).await {
        Ok(Some(doc)) => doc,
        Ok(None) | Err(_) => {
            // Create a new document for settings
            node.docs.create().await.map_err(|e| {
                StorageError::Internal(format!("Failed to create settings document: {}", e))
            })?
        }
    };

    Ok(())
}
