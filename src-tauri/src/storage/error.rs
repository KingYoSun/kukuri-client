use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Iroh initialization failed: {0}")]
    IrohInitialization(#[from] anyhow::Error),

    // Wrap Docs errors in anyhow::Error as its RpcError is private
    #[error("Iroh Docs operation failed: {0}")]
    Docs(anyhow::Error),

    // Use the public RpcError from iroh_blobs::rpc::proto
    #[error("Iroh Blobs operation failed: {0}")]
    Blobs(#[from] iroh_blobs::rpc::proto::RpcError),

    #[error("Iroh Gossip operation failed")] // TODO: Add specific error type if available
    Gossip(String),

    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Data not found for key: {0}")]
    NotFound(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    #[error("Filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Data validation failed: {0}")]
    Validation(String),

    #[error("Operation timed out")]
    Timeout,

    #[error("Internal storage error: {0}")]
    Internal(String),
}

// Helper type for Tauri command results
pub type StorageResult<T> = Result<T, StorageError>;

// Allow converting StorageError to String for Tauri command errors
impl From<StorageError> for String {
    fn from(err: StorageError) -> Self {
        err.to_string()
    }
}
