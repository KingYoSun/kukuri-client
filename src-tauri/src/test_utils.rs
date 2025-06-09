//! Test utilities for integration tests

use crate::storage::{iroh_node::IrohNode, state::initialize_iroh_for_tests, StorageError};
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

// Global lock to ensure tests don't interfere with each other when using shared resources
lazy_static::lazy_static! {
    static ref TEST_LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

/// Test environment that manages a temporary iroh node for testing
pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub iroh_node: IrohNode,
    pub node_path: PathBuf,
    /// Whether this environment owns the node (vs using a shared one)
    owns_node: bool,
}

impl TestEnvironment {
    /// Create a new test environment with a temporary iroh node
    pub async fn new() -> Result<Self, StorageError> {
        // For integration tests, we'll use a shared node to avoid the "RemoteDropped" issue
        // This is necessary because the global state can only be initialized once
        Self::new_shared().await
    }

    /// Create a new isolated test environment 
    /// This should only be used for tests that don't interact with the global state
    pub async fn new_isolated() -> Result<Self, StorageError> {
        let temp_dir = tempfile::tempdir().map_err(StorageError::Io)?;
        let node_path = temp_dir.path().join("iroh_node");
        std::fs::create_dir_all(&node_path).map_err(StorageError::Io)?;

        let iroh_node = IrohNode::new(&node_path).await?;

        Ok(TestEnvironment {
            temp_dir,
            iroh_node,
            node_path,
            owns_node: true,
        })
    }

    /// Create a new test environment that shares the global node
    /// This should be used for integration tests that use the repository functions
    pub async fn new_shared() -> Result<Self, StorageError> {
        // Check if we already have a global node initialized
        if let Some(existing_node) = crate::storage::state::IROH_NODE.get() {
            // Reuse the existing node for this test
            let temp_dir = tempfile::tempdir().map_err(StorageError::Io)?;
            let node_path = temp_dir.path().join("iroh_node");
            
            Ok(TestEnvironment {
                temp_dir,
                iroh_node: existing_node.clone(),
                node_path,
                owns_node: false,
            })
        } else {
            // Create a new node and initialize global state
            let temp_dir = tempfile::tempdir().map_err(StorageError::Io)?;
            let node_path = temp_dir.path().join("iroh_node");
            std::fs::create_dir_all(&node_path).map_err(StorageError::Io)?;

            let iroh_node = IrohNode::new(&node_path).await?;

            // Initialize the global state for tests
            initialize_iroh_for_tests(iroh_node.clone()).await?;

            Ok(TestEnvironment {
                temp_dir,
                iroh_node,
                node_path,
                owns_node: false,
            })
        }
    }

    /// Shutdown the test environment and clean up resources
    pub async fn shutdown(self) -> Result<(), StorageError> {
        // Never shutdown the shared node as it would break other tests
        // The node will be cleaned up when the test process exits
        Ok(())
    }

    /// Initialize the global IROH_NODE state for testing
    /// This allows repository functions to work in tests by setting up the global singleton
    pub async fn initialize_global_state(&self) -> Result<(), StorageError> {
        // Use the test-specific initialization function
        crate::storage::state::initialize_iroh_for_tests(self.iroh_node.clone()).await
    }
}

/// Wait for event propagation in tests
pub async fn wait_for_event_propagation() {
    sleep(Duration::from_millis(100)).await;
}

/// Wait for synchronization between nodes
pub async fn wait_for_sync() {
    sleep(Duration::from_millis(500)).await;
}

/// Wait for a longer sync period for complex operations
pub async fn wait_for_long_sync() {
    sleep(Duration::from_millis(1000)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_creation() {
        let env = TestEnvironment::new_isolated()
            .await
            .expect("Failed to create test environment");
        assert!(env.node_path.exists());
        env.shutdown()
            .await
            .expect("Failed to shutdown test environment");
    }
}