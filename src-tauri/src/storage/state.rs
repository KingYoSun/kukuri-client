use std::sync::OnceLock;

use anyhow::Result;
use iroh_docs::{rpc::client::docs::Doc, NamespaceId};
use quic_rpc::transport::flume::FlumeConnector;
use tauri::Manager; // Import Manager trait for AppHandle::manage

use super::error::StorageError;
use super::iroh_node::IrohNode;

// Type alias for Document with proper connector type
type DocType = Doc<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;

// Global static variables to hold the initialized IrohNode and document handles
// OnceLock ensures they're initialized only once safely across threads.
#[cfg(test)]
pub(crate) static IROH_NODE: OnceLock<IrohNode> = OnceLock::new();
#[cfg(not(test))]
static IROH_NODE: OnceLock<IrohNode> = OnceLock::new();

#[cfg(test)]
pub(crate) static USER_DOC: OnceLock<DocType> = OnceLock::new();
#[cfg(not(test))]
static USER_DOC: OnceLock<DocType> = OnceLock::new();

#[cfg(test)]
pub(crate) static POST_DOC: OnceLock<DocType> = OnceLock::new();
#[cfg(not(test))]
static POST_DOC: OnceLock<DocType> = OnceLock::new();

#[cfg(test)]
pub(crate) static SETTINGS_DOC: OnceLock<DocType> = OnceLock::new();
#[cfg(not(test))]
static SETTINGS_DOC: OnceLock<DocType> = OnceLock::new();

/// Initializes the Iroh node and documents, storing them in global static variables.
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
    let base_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| StorageError::Internal(format!("Failed to get app_data_dir: {}", e)))?;
    let data_root = base_dir.join("iroh_data");

    // Create the IrohNode instance
    let node = IrohNode::new(data_root).await?;

    // Create or load the required documents
    let (user_doc, post_doc, settings_doc) = create_or_load_documents(&node).await?;

    // Attempt to set the documents in the OnceLocks
    let _ = USER_DOC.set(user_doc);
    let _ = POST_DOC.set(post_doc);
    let _ = SETTINGS_DOC.set(settings_doc);

    // Set the node last to indicate full initialization
    let _ = IROH_NODE.set(node);

    Ok(())
}

/// Retrieves a reference to the initialized user document.
/// Panics if the documents have not been initialized yet.
pub fn get_user_doc() -> &'static DocType {
    USER_DOC
        .get()
        .expect("User document has not been initialized. Call initialize_iroh during setup.")
}

/// Retrieves a reference to the initialized post document.
/// Panics if the documents have not been initialized yet.
pub fn get_post_doc() -> &'static DocType {
    POST_DOC
        .get()
        .expect("Post document has not been initialized. Call initialize_iroh during setup.")
}

/// Retrieves a reference to the initialized settings document.
/// Panics if the documents have not been initialized yet.
pub fn get_settings_doc() -> &'static DocType {
    SETTINGS_DOC
        .get()
        .expect("Settings document has not been initialized. Call initialize_iroh during setup.")
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

/// Test-only function to initialize the global state with provided documents
/// This is needed for integration tests that create their own IrohNode instances
#[cfg(test)]
pub async fn initialize_iroh_for_tests(node: IrohNode) -> Result<(), StorageError> {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    // Static mutex to ensure only one test initializes at a time
    lazy_static::lazy_static! {
        static ref INIT_LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    
    // Acquire the lock to prevent concurrent initialization
    let _lock = INIT_LOCK.lock().await;
    
    // For tests, we need to handle the case where tests run in parallel
    // and the static state might already be initialized by another test
    if IROH_NODE.get().is_some() {
        // Already initialized - this is OK for tests
        return Ok(());
    }

    // Create or load the required documents
    let (user_doc, post_doc, settings_doc) = create_or_load_documents(&node).await?;

    // Attempt to set the documents - ignore failures as another test might have set them
    let _ = USER_DOC.set(user_doc);
    let _ = POST_DOC.set(post_doc);
    let _ = SETTINGS_DOC.set(settings_doc);
    let _ = IROH_NODE.set(node);

    Ok(())
}

/// Creates or loads the required documents for the application.
/// Returns tuple of (user_doc, post_doc, settings_doc)
async fn create_or_load_documents(
    node: &IrohNode,
) -> Result<(DocType, DocType, DocType), StorageError> {
    // Load document namespace IDs from persistent storage if they exist
    let namespace_storage = load_namespace_ids(node).await?;

    // Create or open user document
    let user_doc = if let Some(user_ns_id) = namespace_storage.user_namespace_id {
        // Try to open existing document first
        match node.docs.open(user_ns_id).await {
            Ok(Some(doc)) => doc,
            Ok(None) | Err(_) => {
                // Document doesn't exist, create new one
                let doc = node.docs.create().await.map_err(|e| {
                    StorageError::Internal(format!("Failed to create user document: {}", e))
                })?;
                // Save the new namespace ID
                save_namespace_id(node, "user", doc.id()).await?;
                doc
            }
        }
    } else {
        // No saved namespace ID, create new document
        let doc = node.docs.create().await.map_err(|e| {
            StorageError::Internal(format!("Failed to create user document: {}", e))
        })?;
        // Save the namespace ID for future use
        save_namespace_id(node, "user", doc.id()).await?;
        doc
    };

    // Create or open post document
    let post_doc = if let Some(post_ns_id) = namespace_storage.post_namespace_id {
        match node.docs.open(post_ns_id).await {
            Ok(Some(doc)) => doc,
            Ok(None) | Err(_) => {
                let doc = node.docs.create().await.map_err(|e| {
                    StorageError::Internal(format!("Failed to create post document: {}", e))
                })?;
                save_namespace_id(node, "post", doc.id()).await?;
                doc
            }
        }
    } else {
        let doc = node.docs.create().await.map_err(|e| {
            StorageError::Internal(format!("Failed to create post document: {}", e))
        })?;
        save_namespace_id(node, "post", doc.id()).await?;
        doc
    };

    // Create or open settings document
    let settings_doc = if let Some(settings_ns_id) = namespace_storage.settings_namespace_id {
        match node.docs.open(settings_ns_id).await {
            Ok(Some(doc)) => doc,
            Ok(None) | Err(_) => {
                let doc = node.docs.create().await.map_err(|e| {
                    StorageError::Internal(format!("Failed to create settings document: {}", e))
                })?;
                save_namespace_id(node, "settings", doc.id()).await?;
                doc
            }
        }
    } else {
        let doc = node.docs.create().await.map_err(|e| {
            StorageError::Internal(format!("Failed to create settings document: {}", e))
        })?;
        save_namespace_id(node, "settings", doc.id()).await?;
        doc
    };

    Ok((user_doc, post_doc, settings_doc))
}

#[derive(Default)]
struct NamespaceStorage {
    user_namespace_id: Option<NamespaceId>,
    post_namespace_id: Option<NamespaceId>,
    settings_namespace_id: Option<NamespaceId>,
}

/// Loads saved namespace IDs from a special document in the node
async fn load_namespace_ids(node: &IrohNode) -> Result<NamespaceStorage, StorageError> {
    // We use a fixed namespace ID for storing application metadata
    let meta_namespace_id = {
        let hash = blake3::hash(b"kukuri-app-metadata-v1");
        NamespaceId::from(hash.as_bytes())
    };

    let mut storage = NamespaceStorage::default();

    if let Ok(Some(meta_doc)) = node.docs.open(meta_namespace_id).await {
        // Try to load each namespace ID using proper Query API
        let query =
            iroh_docs::store::Query::single_latest_per_key().key_exact(b"user_namespace_id");
        if let Ok(Some(entry)) = meta_doc.get_one(query).await {
            if let Ok(ns_bytes) = node.blobs.read_to_bytes(entry.content_hash()).await {
                if ns_bytes.len() == 32 {
                    let mut ns_array = [0u8; 32];
                    ns_array.copy_from_slice(&ns_bytes);
                    storage.user_namespace_id = Some(NamespaceId::from(ns_array));
                }
            }
        }

        let query =
            iroh_docs::store::Query::single_latest_per_key().key_exact(b"post_namespace_id");
        if let Ok(Some(entry)) = meta_doc.get_one(query).await {
            if let Ok(ns_bytes) = node.blobs.read_to_bytes(entry.content_hash()).await {
                if ns_bytes.len() == 32 {
                    let mut ns_array = [0u8; 32];
                    ns_array.copy_from_slice(&ns_bytes);
                    storage.post_namespace_id = Some(NamespaceId::from(ns_array));
                }
            }
        }

        let query =
            iroh_docs::store::Query::single_latest_per_key().key_exact(b"settings_namespace_id");
        if let Ok(Some(entry)) = meta_doc.get_one(query).await {
            if let Ok(ns_bytes) = node.blobs.read_to_bytes(entry.content_hash()).await {
                if ns_bytes.len() == 32 {
                    let mut ns_array = [0u8; 32];
                    ns_array.copy_from_slice(&ns_bytes);
                    storage.settings_namespace_id = Some(NamespaceId::from(ns_array));
                }
            }
        }
    }

    Ok(storage)
}

/// Saves a namespace ID to the metadata document
async fn save_namespace_id(
    node: &IrohNode,
    doc_type: &str,
    namespace_id: NamespaceId,
) -> Result<(), StorageError> {
    let meta_namespace_id = {
        let hash = blake3::hash(b"kukuri-app-metadata-v1");
        NamespaceId::from(hash.as_bytes())
    };

    // Create or open the metadata document
    let meta_doc = match node.docs.open(meta_namespace_id).await {
        Ok(Some(doc)) => doc,
        Ok(None) | Err(_) => node.docs.create().await.map_err(|e| {
            StorageError::Internal(format!("Failed to create metadata document: {}", e))
        })?,
    };

    // Save the namespace ID
    let key = format!("{}_namespace_id", doc_type);
    let content = namespace_id.as_bytes();

    // Get the default author with retry logic for tests
    let author = super::get_default_author_with_retry(node).await?;

    meta_doc
        .set_bytes(
            author,
            key.as_bytes().to_vec(),
            content.to_vec(),
        )
        .await
        .map_err(|e| StorageError::Internal(format!("Failed to save namespace ID: {}", e)))?;

    Ok(())
}
