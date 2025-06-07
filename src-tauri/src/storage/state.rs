use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use tauri::Manager; // Import Manager trait for AppHandle::manage

use super::error::StorageError;
use super::iroh_node::IrohNode;

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
