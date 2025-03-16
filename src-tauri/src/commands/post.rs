use crate::models::post::Post;
use serde::{Deserialize, Serialize};
// Tauri v2ではcommandマクロを使用します
use chrono::Utc;
use tauri::command;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResult {
    pub post_id: String,
    pub success: bool,
    pub message: Option<String>,
}

#[command]
pub async fn create_post(author_id: String, content: String) -> Result<PostResult, String> {
    // 1. 投稿IDを生成
    let post_id = Uuid::new_v4().to_string();

    // 2. 投稿を作成
    let post = Post {
        id: post_id.clone(),
        author_id,
        content,
        attachments: vec![],
        mentions: vec![],
        hashtags: vec![],
        created_at: Utc::now().timestamp(),
    };

    // 3. 投稿を保存
    match crate::storage::automerge::save_post(&post) {
        Ok(_) => {
            // 4. iroh-gossipで投稿を発信
            match crate::network::iroh::publish_post(&post) {
                Ok(_) => Ok(PostResult {
                    post_id,
                    success: true,
                    message: None,
                }),
                Err(e) => {
                    // ネットワーク発信に失敗しても投稿は保存されている
                    Ok(PostResult {
                        post_id,
                        success: true,
                        message: Some(format!("Post created but failed to publish: {}", e)),
                    })
                }
            }
        }
        Err(e) => Err(format!("Failed to save post: {}", e)),
    }
}

#[command]
pub async fn get_posts(limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Post>, String> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    crate::storage::automerge::get_posts(limit, offset)
        .map_err(|e| format!("Failed to retrieve posts: {}", e))
}

#[command]
pub async fn get_user_posts(
    user_id: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<Post>, String> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    crate::storage::automerge::get_user_posts(&user_id, limit, offset)
        .map_err(|e| format!("Failed to retrieve user posts: {}", e))
}

#[command]
pub async fn search_posts(query: String, limit: Option<usize>) -> Result<Vec<Post>, String> {
    let limit = limit.unwrap_or(50);

    // ローカルの投稿からの簡易検索
    crate::storage::automerge::search_posts(&query, limit)
        .map_err(|e| format!("Failed to search posts: {}", e))
}
