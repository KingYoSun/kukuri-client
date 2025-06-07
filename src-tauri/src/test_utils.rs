//! Test utilities for integration tests

use crate::storage::{iroh_node::IrohNode, StorageError};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::time::{sleep, Duration};

/// Test environment that manages a temporary iroh node for testing
pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub iroh_node: IrohNode,
    pub node_path: PathBuf,
}

impl TestEnvironment {
    /// Create a new test environment with a temporary iroh node
    pub async fn new() -> Result<Self, StorageError> {
        let temp_dir = tempfile::tempdir().map_err(StorageError::Io)?;

        let node_path = temp_dir.path().join("iroh_node");
        std::fs::create_dir_all(&node_path).map_err(StorageError::Io)?;

        let iroh_node = IrohNode::new(&node_path).await?;

        Ok(TestEnvironment {
            temp_dir,
            iroh_node,
            node_path,
        })
    }

    /// Shutdown the test environment and clean up resources
    pub async fn shutdown(self) -> Result<(), StorageError> {
        self.iroh_node.shutdown().await?;
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
        let env = TestEnvironment::new()
            .await
            .expect("Failed to create test environment");
        assert!(env.node_path.exists());
        env.shutdown()
            .await
            .expect("Failed to shutdown test environment");
    }
}
