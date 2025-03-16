use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub author_id: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub mentions: Vec<String>,
    pub hashtags: Vec<String>,
    pub created_at: i64,
}