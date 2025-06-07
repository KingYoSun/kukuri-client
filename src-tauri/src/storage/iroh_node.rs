use std::{path::PathBuf, sync::Arc};

use super::error::StorageError;
use anyhow::Result;
use iroh::protocol::Router;
use iroh_docs::NamespaceId; // Import NamespaceId
use quic_rpc::transport::flume::FlumeConnector;

// Define fixed Namespace IDs (replace with a better generation/storage mechanism if needed)
// These act like table names or document collections.
const USER_NAMESPACE_STR: &str = "kukuri-user-profiles-v1";
const POST_NAMESPACE_STR: &str = "kukuri-posts-v1";
const SETTINGS_NAMESPACE_STR: &str = "kukuri-settings-v1"; // Add settings namespace string

lazy_static::lazy_static! {
    // Create NamespaceIds by hashing the string identifiers to get deterministic 32-byte arrays
    pub static ref USER_NAMESPACE_ID: NamespaceId = {
        let hash = blake3::hash(USER_NAMESPACE_STR.as_bytes());
        NamespaceId::from(hash.as_bytes())
    };
    pub static ref POST_NAMESPACE_ID: NamespaceId = {
        let hash = blake3::hash(POST_NAMESPACE_STR.as_bytes());
        NamespaceId::from(hash.as_bytes())
    };
    pub static ref SETTINGS_NAMESPACE_ID: NamespaceId = {
        let hash = blake3::hash(SETTINGS_NAMESPACE_STR.as_bytes());
        NamespaceId::from(hash.as_bytes())
    };
}
// Type alias for the Flume-based Docs RPC client
pub(crate) type DocsClient = iroh_docs::rpc::client::docs::Client<
    FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>,
>;

// Type alias for the Flume-based Blobs RPC client
pub(crate) type BlobsClient = iroh_blobs::rpc::client::blobs::Client<
    FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>,
>;

// Type alias for the Flume-based Authors RPC client
pub(crate) type AuthorsClient = iroh_docs::rpc::client::authors::Client<
    FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>,
>;

/// Holds the initialized iroh node components and RPC clients.
#[derive(Clone, Debug)]
pub(crate) struct IrohNode {
    #[allow(dead_code)] // Router might be needed later for direct interaction or shutdown
    router: Router,
    pub(crate) blobs: BlobsClient,
    pub(crate) docs: DocsClient,
    pub(crate) authors: AuthorsClient,
}

impl IrohNode {
    /// Initializes the iroh node, sets up protocols (Gossip, Blobs, Docs),
    /// spawns the router, and returns an `IrohNode` instance containing RPC clients.
    ///
    /// `path`: The root directory for iroh data persistence.
    pub async fn new(path: PathBuf) -> Result<Self, StorageError> {
        // Ensure the data directory exists
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(StorageError::Io)?;

        // Load or create the secret key
        // Using iroh_blobs utility, but key is used for the main endpoint identity
        let key = iroh_blobs::util::fs::load_secret_key(path.join("keypair"))
            .await
            .map_err(StorageError::IrohInitialization)?;

        // Create the iroh endpoint
        let endpoint = iroh::Endpoint::builder()
            .discovery_n0() // Use n0 discovery service
            .secret_key(key)
            .bind()
            .await
            .map_err(StorageError::IrohInitialization)?;

        // Build the protocol router
        let mut builder = iroh::protocol::Router::builder(endpoint);

        // Initialize and add iroh-gossip protocol
        let gossip = iroh_gossip::net::Gossip::builder()
            .spawn(builder.endpoint().clone())
            .await
            .map_err(|e| StorageError::IrohInitialization(e.into()))?; // Wrap gossip error
        builder = builder.accept(iroh_gossip::ALPN, Arc::new(gossip.clone()));

        // Initialize and add iroh-blobs protocol (persistent)
        let blobs = iroh_blobs::net_protocol::Blobs::persistent(&path)
            .await
            .map_err(StorageError::IrohInitialization)?
            .build(builder.endpoint());
        builder = builder.accept(iroh_blobs::ALPN, blobs.clone());

        // Initialize and add iroh-docs protocol (persistent)
        let docs = iroh_docs::protocol::Docs::persistent(path) // Use the same root path
            .spawn(&blobs, &gossip)
            .await
            .map_err(StorageError::IrohInitialization)?;
        builder = builder.accept(iroh_docs::ALPN, Arc::new(docs.clone()));

        // Spawn the router to handle incoming connections for the registered protocols
        let router = builder.spawn();

        // Get RPC clients for interacting with the protocols
        let blobs_client = blobs.client().clone();
        let docs_client = docs.client().clone();
        // Get the authors client from the docs client
        let authors_client = docs_client.authors();

        Ok(Self {
            router,
            blobs: blobs_client,
            docs: docs_client,
            authors: authors_client,
        })
    }

    /// Gracefully shuts down the iroh router.
    #[allow(dead_code)]
    pub(crate) async fn shutdown(self) -> Result<(), StorageError> {
        self.router
            .shutdown()
            .await
            .map_err(StorageError::IrohInitialization)
    }
}
