use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// モックエントリー
#[derive(Debug, Clone)]
pub struct Entry {
    pub(crate) id: String,
    pub(crate) namespace: String,
    pub(crate) key: String,
    pub(crate) content: Vec<u8>,
}

impl Entry {
    pub fn new(namespace: &str, key: &str, content: &[u8]) -> Self {
        Self {
            id: format!("{}:{}", namespace, key),
            namespace: namespace.to_string(),
            key: key.to_string(),
            content: content.to_vec(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

// モックドキュメント
#[derive(Debug, Clone)]
pub struct Doc {
    pub(crate) id: String,
    pub(crate) entries: Arc<Mutex<HashMap<String, Entry>>>,
}

impl Doc {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn list_entries(&self) -> Result<Vec<Entry>, String> {
        let entries = self.entries.lock().unwrap();
        Ok(entries.values().cloned().collect())
    }

    pub fn list_entries_by_prefix(
        &self,
        namespace: &str,
        key_prefix: &str,
    ) -> Result<Vec<Entry>, String> {
        let entries = self.entries.lock().unwrap();
        let filtered: Vec<Entry> = entries
            .values()
            .filter(|entry| entry.namespace == namespace && entry.key.starts_with(key_prefix))
            .cloned()
            .collect();
        Ok(filtered)
    }

    pub fn set_bytes(&self, namespace: &str, key: &str, content: &[u8]) -> Result<(), String> {
        let entry = Entry::new(namespace, key, content);
        let mut entries = self.entries.lock().unwrap();
        entries.insert(entry.id.clone(), entry);
        Ok(())
    }

    pub fn get_content(&self, namespace: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        let entries = self.entries.lock().unwrap();
        let id = format!("{}:{}", namespace, key);
        Ok(entries.get(&id).map(|entry| entry.content.clone()))
    }

    pub fn get_content_by_entry(&self, entry: &Entry) -> Result<Option<Vec<u8>>, String> {
        let entries = self.entries.lock().unwrap();
        Ok(entries.get(entry.id()).map(|e| e.content.clone()))
    }
}
