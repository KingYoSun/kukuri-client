use crate::models::post::Post;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;

/// 投稿エラー
///
/// 投稿処理中に発生する可能性のあるエラーを定義します。
#[derive(Debug, thiserror::Error)]
pub enum PostError {
    /// ストレージエラー
    #[error("Storage error: {0}")]
    Storage(String),

    /// ネットワークエラー
    #[error("Network error: {0}")]
    Network(String),

    /// 入力検証エラー
    #[error("Validation error: {0}")]
    Validation(String),

    /// その他のエラー
    #[error("{0}")]
    Other(String),
}

/// エラーのシリアライズ実装
impl Serialize for PostError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// 投稿結果
#[derive(Debug, Serialize, Deserialize)]
pub struct PostResult {
    pub post_id: String,
    pub success: bool,
    pub message: Option<String>,
}

/// 投稿作成コマンド
///
/// 新しい投稿を作成し、ストレージに保存してネットワークに発信します。
#[command]
pub async fn create_post(author_id: String, content: String) -> Result<PostResult, PostError> {
    // 入力検証
    if content.trim().is_empty() {
        return Err(PostError::Validation("Content cannot be empty".to_string()));
    }

    if content.len() > 500 {
        return Err(PostError::Validation(
            "Content exceeds maximum length of 500 characters".to_string(),
        ));
    }

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
    crate::storage::automerge::save_post(&post).map_err(|e| PostError::Storage(e))?;

    // 4. iroh-gossipで投稿を発信
    match crate::network::iroh::publish_post(&post) {
        Ok(_) => Ok(PostResult {
            post_id,
            success: true,
            message: None,
        }),
        Err(e) => {
            // ネットワーク発信に失敗しても投稿は保存されている
            println!("Warning: Failed to publish post: {}", e);
            Ok(PostResult {
                post_id,
                success: true,
                message: Some(format!("Post created but failed to publish: {}", e)),
            })
        }
    }
}

/// 投稿取得コマンド
///
/// すべての投稿を取得します。
#[command]
pub async fn get_posts(
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<Post>, PostError> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    crate::storage::automerge::get_posts(limit, offset).map_err(|e| PostError::Storage(e))
}

/// ユーザー投稿取得コマンド
///
/// 特定のユーザーの投稿を取得します。
#[command]
pub async fn get_user_posts(
    user_id: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<Post>, PostError> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    crate::storage::automerge::get_user_posts(&user_id, limit, offset)
        .map_err(|e| PostError::Storage(e))
}

/// 投稿検索コマンド
///
/// 指定されたクエリに一致する投稿を検索します。
#[command]
pub async fn search_posts(query: String, limit: Option<usize>) -> Result<Vec<Post>, PostError> {
    // 入力検証
    if query.trim().is_empty() {
        return Err(PostError::Validation(
            "Search query cannot be empty".to_string(),
        ));
    }

    let limit = limit.unwrap_or(50);

    // ローカルの投稿からの簡易検索
    crate::storage::automerge::search_posts(&query, limit).map_err(|e| PostError::Storage(e))
}

// テストコードは省略
