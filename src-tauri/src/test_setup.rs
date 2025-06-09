//! Test setup module for integration tests

use crate::storage::{iroh_node::IrohNode, StorageError};
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    /// Global lock to ensure test setup happens sequentially
    static ref SETUP_LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

/// Simple test environment that creates a fresh iroh node for each test
pub struct TestEnvironment {
    pub iroh_node: IrohNode,
    pub temp_dir: tempfile::TempDir,
}

impl TestEnvironment {
    /// Create a new test environment with a fresh iroh node
    pub async fn new() -> Result<Self, StorageError> {
        let _lock = SETUP_LOCK.lock().await;
        
        println!("[TEST SETUP] Creating fresh test environment");
        
        // Create a new node for each test
        let temp_dir = tempfile::tempdir().map_err(StorageError::Io)?;
        let node_path = temp_dir.path().join("test_iroh_node");
        std::fs::create_dir_all(&node_path).map_err(StorageError::Io)?;
        
        let iroh_node = IrohNode::new(&node_path).await?;
        println!("[TEST SETUP] Created fresh iroh node successfully");
        
        Ok(TestEnvironment {
            iroh_node,
            temp_dir,
        })
    }
}

/// Initialize test environment - for backward compatibility
/// This still uses global state but avoids the unsafe reset
pub async fn setup_test_environment() -> Result<(), StorageError> {
    let _lock = SETUP_LOCK.lock().await;
    
    // Check if already initialized
    if crate::storage::state::IROH_NODE.get().is_some() {
        println!("[TEST SETUP] Test environment already initialized, skipping");
        return Ok(());
    }
    
    println!("[TEST SETUP] Setting up test environment for first time");
    
    // Create a new node
    let temp_dir = tempfile::tempdir().map_err(StorageError::Io)?;
    let node_path = temp_dir.path().join("test_iroh_node");
    std::fs::create_dir_all(&node_path).map_err(StorageError::Io)?;
    
    let iroh_node = IrohNode::new(&node_path).await?;
    println!("[TEST SETUP] Created iroh node successfully");
    
    // Initialize global state
    crate::storage::state::initialize_iroh_for_tests(iroh_node).await?;
    println!("[TEST SETUP] Initialized global state successfully");
    
    // Leak the temp_dir to prevent cleanup
    std::mem::forget(temp_dir);
    
    println!("[TEST SETUP] Test environment setup complete");
    Ok(())
}