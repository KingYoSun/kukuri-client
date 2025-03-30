use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::storage::models::{Doc, Entry};

// モックDocs
#[derive(Debug, Clone)]
pub struct Docs {
    pub(crate) documents: Arc<Mutex<HashMap<String, Doc>>>,
}

impl Docs {
    pub fn new() -> Self {
        Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, doc_id: &str) -> Result<Doc, String> {
        let documents = self.documents.lock().unwrap();
        match documents.get(doc_id) {
            Some(doc) => Ok(doc.clone()),
            None => Err(format!("Document not found: {}", doc_id)),
        }
    }

    pub fn create(&self) -> Result<Doc, String> {
        let doc_id = Uuid::new_v4().to_string();
        let doc = Doc::new(&doc_id);
        let mut documents = self.documents.lock().unwrap();
        documents.insert(doc_id.clone(), doc.clone());
        Ok(doc)
    }

    pub fn sync_doc(&self, doc: &Doc, peer_id: &str) -> Result<(), String> {
        // モック実装では何もしない
        println!("Syncing document {} with peer {}", doc.id, peer_id);
        Ok(())
    }
}
