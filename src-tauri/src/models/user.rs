use crate::storage::HasId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub bio: String,
    pub public_key: String,
    pub avatar: Option<String>,
    pub following: Vec<String>,
    pub followers: Vec<String>,
    pub created_at: i64,
}

impl HasId for User {
    fn id(&self) -> &str {
        &self.id
    }
}
