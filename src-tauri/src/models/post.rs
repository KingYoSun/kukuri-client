use crate::storage::{HasId, Post as PostTrait};
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

impl HasId for Post {
    fn id(&self) -> &str {
        &self.id
    }
}

impl PostTrait for Post {
    fn author_id(&self) -> &str {
        &self.author_id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn created_at(&self) -> i64 {
        self.created_at
    }
}
