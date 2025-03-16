use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

use crate::models::post::Post;
use crate::models::user::User;

// モックストレージ
#[derive(Default, Serialize, Deserialize)]
struct MockStorage {
    users: HashMap<String, User>,
    posts: HashMap<String, Post>,
}

// グローバルなモックストレージ
static STORAGE: Lazy<Mutex<MockStorage>> =
    Lazy::new(|| Mutex::new(load_or_create_storage().unwrap_or_else(|_| MockStorage::default())));

// データファイルのパス
fn get_data_file_path() -> Result<PathBuf, String> {
    // 標準ライブラリの関数を使用
    let app_dir = std::env::temp_dir();

    let data_dir = app_dir.join("kukuri-client").join("data");
    fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data directory: {}", e))?;

    Ok(data_dir.join("social_data.json"))
}

// ストレージの読み込みまたは新規作成
fn load_or_create_storage() -> Result<MockStorage, String> {
    let file_path = get_data_file_path()?;

    if file_path.exists() {
        let mut file =
            File::open(&file_path).map_err(|e| format!("Failed to open data file: {}", e))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read data file: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse storage data: {}", e))
    } else {
        let storage = MockStorage::default();
        save_storage(&storage)?;
        Ok(storage)
    }
}

// ストレージの保存
fn save_storage(storage: &MockStorage) -> Result<(), String> {
    let file_path = get_data_file_path()?;

    let content = serde_json::to_string_pretty(storage)
        .map_err(|e| format!("Failed to serialize storage: {}", e))?;

    let mut file =
        File::create(&file_path).map_err(|e| format!("Failed to create data file: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write data file: {}", e))?;

    Ok(())
}

// ユーザー保存
pub fn save_user(user: &User) -> Result<(), String> {
    let mut storage = STORAGE.lock().unwrap();
    storage.users.insert(user.id.clone(), user.clone());
    save_storage(&storage)
}

// ユーザー取得
pub fn get_user(user_id: &str) -> Result<Option<User>, String> {
    let storage = STORAGE.lock().unwrap();
    Ok(storage.users.get(user_id).cloned())
}

// 投稿保存
pub fn save_post(post: &Post) -> Result<(), String> {
    let mut storage = STORAGE.lock().unwrap();
    storage.posts.insert(post.id.clone(), post.clone());
    save_storage(&storage)
}

// 投稿取得（ページネーション付き）
pub fn get_posts(limit: usize, offset: usize) -> Result<Vec<Post>, String> {
    let storage = STORAGE.lock().unwrap();

    let mut posts: Vec<Post> = storage.posts.values().cloned().collect();

    // 作成日時の降順でソート
    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // ページネーション適用
    let end = std::cmp::min(offset + limit, posts.len());
    if offset < end {
        Ok(posts[offset..end].to_vec())
    } else {
        Ok(Vec::new())
    }
}

// 特定ユーザーの投稿取得（ページネーション付き）
pub fn get_user_posts(user_id: &str, limit: usize, offset: usize) -> Result<Vec<Post>, String> {
    let storage = STORAGE.lock().unwrap();

    let mut user_posts: Vec<Post> = storage
        .posts
        .values()
        .filter(|post| post.author_id == user_id)
        .cloned()
        .collect();

    // 作成日時の降順でソート
    user_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // ページネーション適用
    let end = std::cmp::min(offset + limit, user_posts.len());
    if offset < end {
        Ok(user_posts[offset..end].to_vec())
    } else {
        Ok(Vec::new())
    }
}

// 投稿検索（ローカルのみ）
pub fn search_posts(query: &str, limit: usize) -> Result<Vec<Post>, String> {
    let storage = STORAGE.lock().unwrap();
    let query_lower = query.to_lowercase();

    let mut matching_posts: Vec<Post> = storage
        .posts
        .values()
        .filter(|post| post.content.to_lowercase().contains(&query_lower))
        .take(limit)
        .cloned()
        .collect();

    // 作成日時の降順でソート
    matching_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(matching_posts)
}

// リモート変更のマージ（モック実装）
pub fn merge_remote_changes(_changes: &[u8]) -> Result<(), String> {
    // モック実装では何もしない
    Ok(())
}

// 最後のマージ以降の変更を取得（モック実装）
pub fn get_changes_since(_heads: Vec<String>) -> Result<Vec<u8>, String> {
    // モック実装では空のバイト列を返す
    Ok(Vec::new())
}

// ドキュメントの現在のヘッドハッシュを取得（モック実装）
pub fn get_heads() -> Result<Vec<String>, String> {
    // モック実装では空の配列を返す
    Ok(Vec::new())
}
