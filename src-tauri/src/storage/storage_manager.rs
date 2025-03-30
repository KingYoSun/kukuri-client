use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::commands::settings::Settings;
use crate::storage::docs::Docs;
use crate::storage::models::Doc;
use crate::storage::sync_manager::SyncManager;

// ユーザーIDを取得するためのトレイト
pub trait HasId {
    fn id(&self) -> &str;
}

// 投稿のトレイト
pub trait Post: HasId + Clone {
    fn author_id(&self) -> &str;
    fn content(&self) -> &str;
    fn created_at(&self) -> i64;
}

/// ストレージマネージャー
///
/// iroh-docsを使用したデータストレージを管理します。
pub struct StorageManager {
    /// iroh-docsインスタンス
    docs: Arc<Docs>,
    /// 同期マネージャー
    sync_manager: Option<SyncManager>,
}

impl StorageManager {
    /// 新しいStorageManagerを作成
    pub fn new() -> Result<Self, String> {
        // モックDocsインスタンスを作成
        let docs = Arc::new(Docs::new());

        Ok(Self {
            docs,
            sync_manager: None,
        })
    }

    /// ドキュメントの作成または取得
    ///
    /// 指定されたIDのドキュメントを作成または取得します。
    pub fn get_or_create_document(&mut self, doc_id: &str) -> Result<(), String> {
        // ドキュメントが存在するか確認
        let doc = match self.docs.get(doc_id) {
            Ok(doc) => doc,
            Err(_) => {
                // ドキュメントが存在しない場合は作成
                let mut docs = self.docs.documents.lock().unwrap();
                let doc = Doc::new(doc_id);
                docs.insert(doc_id.to_string(), doc.clone());
                doc
            }
        };

        // 同期マネージャーを作成
        self.sync_manager = Some(SyncManager::new(self.docs.clone(), doc_id.to_string()));

        Ok(())
    }

    /// ユーザーの保存
    ///
    /// ユーザープロフィールをドキュメントに保存します。
    pub fn save_user<T: Serialize + HasId>(&self, user: &T) -> Result<(), String> {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // ユーザーをJSONにシリアライズ
        let user_json =
            serde_json::to_string(user).map_err(|e| format!("Failed to serialize user: {}", e))?;

        // ユーザーをドキュメントに保存
        doc.set_bytes("users", user.id(), user_json.as_bytes())
    }

    /// ユーザーの取得
    ///
    /// ドキュメントからユーザープロフィールを取得します。
    pub fn get_user<T: for<'de> Deserialize<'de>>(
        &self,
        user_id: &str,
    ) -> Result<Option<T>, String> {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // ユーザーをドキュメントから取得
        match doc.get_content("users", user_id) {
            Ok(Some(content)) => {
                // JSONからデシリアライズ
                let user = serde_json::from_slice(&content)
                    .map_err(|e| format!("Failed to deserialize user: {}", e))?;

                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// 投稿の保存
    ///
    /// 投稿をドキュメントに保存します。
    pub fn save_post<T: Serialize + HasId>(&self, post: &T) -> Result<(), String> {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // 投稿をJSONにシリアライズ
        let post_json =
            serde_json::to_string(post).map_err(|e| format!("Failed to serialize post: {}", e))?;

        // 投稿をドキュメントに保存
        doc.set_bytes("posts", post.id(), post_json.as_bytes())
    }

    /// 投稿の取得
    ///
    /// ドキュメントから投稿を取得します。
    pub fn get_posts<T>(&self, limit: usize, offset: usize) -> Result<Vec<T>, String>
    where
        T: for<'de> Deserialize<'de> + Post,
    {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // 投稿をドキュメントから取得
        let entries = doc.list_entries_by_prefix("posts", "")?;

        let mut posts = Vec::new();

        // 投稿をデシリアライズ
        for entry in entries {
            if let Ok(Some(content)) = doc.get_content_by_entry(&entry) {
                if let Ok(post) = serde_json::from_slice::<T>(&content) {
                    posts.push(post);
                }
            }
        }

        // 作成日時の降順でソート
        posts.sort_by(|a, b| b.created_at().cmp(&a.created_at()));

        // ページネーション適用
        let end = std::cmp::min(offset + limit, posts.len());
        if offset < end {
            Ok(posts[offset..end].to_vec())
        } else {
            Ok(Vec::new())
        }
    }

    /// 特定ユーザーの投稿取得
    ///
    /// 特定のユーザーの投稿を取得します。
    pub fn get_user_posts<T>(
        &self,
        user_id: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<T>, String>
    where
        T: for<'de> Deserialize<'de> + Post,
    {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // 投稿をドキュメントから取得
        let entries = doc.list_entries_by_prefix("posts", "")?;

        let mut user_posts = Vec::new();

        // ユーザーIDに一致する投稿を収集
        for entry in entries {
            if let Ok(Some(content)) = doc.get_content_by_entry(&entry) {
                if let Ok(post) = serde_json::from_slice::<T>(&content) {
                    if post.author_id() == user_id {
                        user_posts.push(post);
                    }
                }
            }
        }

        // 作成日時の降順でソート
        user_posts.sort_by(|a, b| b.created_at().cmp(&a.created_at()));

        // ページネーション適用
        let end = std::cmp::min(offset + limit, user_posts.len());
        if offset < end {
            Ok(user_posts[offset..end].to_vec())
        } else {
            Ok(Vec::new())
        }
    }

    /// 投稿検索
    ///
    /// 投稿を検索します。
    pub fn search_posts<T>(&self, query: &str, limit: usize) -> Result<Vec<T>, String>
    where
        T: for<'de> Deserialize<'de> + Post,
    {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // 投稿をドキュメントから取得
        let entries = doc.list_entries_by_prefix("posts", "")?;

        let mut matching_posts = Vec::new();
        let query_lower = query.to_lowercase();

        // クエリに一致する投稿を収集
        for entry in entries {
            if let Ok(Some(content)) = doc.get_content_by_entry(&entry) {
                if let Ok(post) = serde_json::from_slice::<T>(&content) {
                    // コンテンツ内にクエリが含まれているか確認
                    if post.content().to_lowercase().contains(&query_lower) {
                        matching_posts.push(post);
                        if matching_posts.len() >= limit {
                            break;
                        }
                    }
                }
            }
        }

        // 作成日時の降順でソート
        matching_posts.sort_by(|a, b| b.created_at().cmp(&a.created_at()));

        Ok(matching_posts)
    }

    /// 設定の取得
    ///
    /// ドキュメントから設定を取得します。
    pub fn get_settings(&self, settings_key: &str) -> Result<Option<Settings>, String> {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // 設定をドキュメントから取得
        match doc.get_content("settings", settings_key) {
            Ok(Some(content)) => {
                // JSONからデシリアライズ
                let settings = serde_json::from_slice(&content)
                    .map_err(|e| format!("Failed to deserialize settings: {}", e))?;

                Ok(Some(settings))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// 設定の保存
    ///
    /// 設定をドキュメントに保存します。
    pub fn save_settings(&self, settings_key: &str, settings: &Settings) -> Result<(), String> {
        let sync_manager = self
            .sync_manager
            .as_ref()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        let doc = sync_manager.get_document()?;

        // 設定をJSONにシリアライズ
        let settings_json = serde_json::to_string(settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        // 設定をドキュメントに保存
        doc.set_bytes("settings", settings_key, settings_json.as_bytes())
    }

    /// 同期の実行
    ///
    /// すべてのアクティブなピアとの同期を実行します。
    pub fn sync_with_peers(&mut self) -> Result<(), String> {
        let sync_manager = self
            .sync_manager
            .as_mut()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        sync_manager.sync_with_peers()
    }

    /// ピアの追加
    ///
    /// 同期対象のピアを追加します。
    pub fn add_peer(&mut self, peer_id: &str) -> Result<(), String> {
        let sync_manager = self
            .sync_manager
            .as_mut()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        sync_manager.add_peer(peer_id);
        Ok(())
    }

    /// ピアの削除
    ///
    /// 同期対象のピアを削除します。
    pub fn remove_peer(&mut self, peer_id: &str) -> Result<(), String> {
        let sync_manager = self
            .sync_manager
            .as_mut()
            .ok_or_else(|| "Sync manager not initialized".to_string())?;

        sync_manager.remove_peer(peer_id);
        Ok(())
    }
}
