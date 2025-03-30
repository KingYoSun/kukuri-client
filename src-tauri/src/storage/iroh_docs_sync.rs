// このファイルは、automergeからiroh-docsへの移行のためのモック実装です。
// 実際の実装では、iroh-docsを使用してデータの同期を行います。

use crate::commands::settings::Settings;
use crate::models::post::Post;
use crate::models::user::User;
use crate::storage::StorageManager;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// グローバルなStorageManagerインスタンス
static STORAGE_MANAGER: Lazy<Mutex<StorageManager>> = Lazy::new(|| {
    let mut manager = StorageManager::new().expect("Failed to create StorageManager");
    manager
        .get_or_create_document("default")
        .expect("Failed to create default document");
    Mutex::new(manager)
});

/// ユーザーの保存
///
/// ユーザープロフィールをストレージに保存します。
pub fn save_user(user: &User) -> Result<(), String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.save_user(user)
}

/// ユーザーの取得
///
/// ストレージからユーザープロフィールを取得します。
pub fn get_user(user_id: &str) -> Result<Option<User>, String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.get_user(user_id)
}

/// 投稿の保存
///
/// 投稿をストレージに保存します。
pub fn save_post(post: &Post) -> Result<(), String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.save_post(post)
}

/// 投稿の取得
///
/// ストレージから投稿を取得します。
pub fn get_posts(limit: usize, offset: usize) -> Result<Vec<Post>, String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.get_posts(limit, offset)
}

/// 特定ユーザーの投稿取得
///
/// 特定のユーザーの投稿を取得します。
pub fn get_user_posts(user_id: &str, limit: usize, offset: usize) -> Result<Vec<Post>, String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.get_user_posts(user_id, limit, offset)
}

/// 投稿検索
///
/// 投稿を検索します。
pub fn search_posts(query: &str, limit: usize) -> Result<Vec<Post>, String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.search_posts(query, limit)
}

/// 設定の取得
///
/// ストレージから設定を取得します。
pub fn get_settings(settings_key: &str) -> Result<Option<Settings>, String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.get_settings(settings_key)
}

/// 設定の保存
///
/// 設定をストレージに保存します。
pub fn save_settings(settings_key: &str, settings: &Settings) -> Result<(), String> {
    let storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.save_settings(settings_key, settings)
}

/// 同期の実行
///
/// すべてのアクティブなピアとの同期を実行します。
pub fn sync_with_peers() -> Result<(), String> {
    let mut storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.sync_with_peers()
}

/// ピアの追加
///
/// 同期対象のピアを追加します。
pub fn add_peer(peer_id: &str) -> Result<(), String> {
    let mut storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.add_peer(peer_id)
}

/// ピアの削除
///
/// 同期対象のピアを削除します。
pub fn remove_peer(peer_id: &str) -> Result<(), String> {
    let mut storage_manager = STORAGE_MANAGER.lock().unwrap();
    storage_manager.remove_peer(peer_id)
}
