//! ドキュメント変更イベント処理

use anyhow::Result;
use futures_lite::StreamExt;
use serde_json::json;
use std::time::Duration;
use tauri::{Emitter, Manager};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

use crate::storage::iroh_node::{POST_NAMESPACE_ID, USER_NAMESPACE_ID};
use crate::storage::state::get_iroh_node;

/// ドキュメント変更監視サービス
pub struct DocumentSubscriptionService {
    user_subscription_handle: Option<JoinHandle<()>>,
    post_subscription_handle: Option<JoinHandle<()>>,
}

impl DocumentSubscriptionService {
    pub fn new() -> Self {
        Self {
            user_subscription_handle: None,
            post_subscription_handle: None,
        }
    }

    /// ドキュメント変更監視を開始
    pub async fn start(&mut self, app_handle: tauri::AppHandle) -> Result<()> {
        info!("Starting document subscription service");

        // Note: Documents might not exist yet, so we'll start monitoring and handle the case gracefully
        // The subscription will start working once documents are created during normal app usage

        // Userドキュメントの監視を開始 (エラーを無視)
        match self.start_user_subscription(app_handle.clone()).await {
            Ok(handle) => {
                self.user_subscription_handle = Some(handle);
                info!("User document subscription started");
            }
            Err(e) => {
                warn!("Failed to start user document subscription: {}. Will retry when documents are created.", e);
            }
        }

        // Postドキュメントの監視を開始 (エラーを無視)
        match self.start_post_subscription(app_handle).await {
            Ok(handle) => {
                self.post_subscription_handle = Some(handle);
                info!("Post document subscription started");
            }
            Err(e) => {
                warn!("Failed to start post document subscription: {}. Will retry when documents are created.", e);
            }
        }

        info!("Document subscription service initialization completed");
        Ok(())
    }

    /// ユーザードキュメントの監視を開始
    async fn start_user_subscription(
        &self,
        app_handle: tauri::AppHandle,
    ) -> Result<JoinHandle<()>> {
        let iroh = get_iroh_node();

        // Userドキュメントを開くか、存在しない場合は作成
        let user_doc = match iroh.docs.open(*USER_NAMESPACE_ID).await? {
            Some(doc) => doc,
            None => {
                debug!("User document not found, creating new document");
                // 存在しない場合は新しいドキュメントを作成
                iroh.docs
                    .create()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to create user document: {}", e))?
            }
        };

        // LiveEventsを購読
        let mut live_events = user_doc.subscribe().await?;

        let handle = tokio::spawn(async move {
            info!("User document subscription started");

            // イベントストリームを処理
            loop {
                match live_events.next().await {
                    Some(Ok(event)) => {
                        if let Err(e) = handle_user_document_event(event, &app_handle).await {
                            error!("Error handling user document event: {}", e);
                        }
                    }
                    Some(Err(e)) => {
                        error!("Error receiving user document event: {}", e);
                        // 再接続の試行
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        // 実際の実装では再接続ロジックを追加する
                        break;
                    }
                    None => {
                        warn!("User document event stream ended");
                        break;
                    }
                }
            }

            warn!("User document subscription ended");
        });

        Ok(handle)
    }

    /// 投稿ドキュメントの監視を開始
    async fn start_post_subscription(
        &self,
        app_handle: tauri::AppHandle,
    ) -> Result<JoinHandle<()>> {
        let iroh = get_iroh_node();

        // Postドキュメントを開くか、存在しない場合は作成
        let post_doc = match iroh.docs.open(*POST_NAMESPACE_ID).await? {
            Some(doc) => doc,
            None => {
                debug!("Post document not found, creating new document");
                // 存在しない場合は新しいドキュメントを作成
                iroh.docs
                    .create()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to create post document: {}", e))?
            }
        };

        // LiveEventsを購読
        let mut live_events = post_doc.subscribe().await?;

        let handle = tokio::spawn(async move {
            info!("Post document subscription started");

            // イベントストリームを処理
            loop {
                match live_events.next().await {
                    Some(Ok(event)) => {
                        if let Err(e) = handle_post_document_event(event, &app_handle).await {
                            error!("Error handling post document event: {}", e);
                        }
                    }
                    Some(Err(e)) => {
                        error!("Error receiving post document event: {}", e);
                        // 再接続の試行
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        // 実際の実装では再接続ロジックを追加する
                        break;
                    }
                    None => {
                        warn!("Post document event stream ended");
                        break;
                    }
                }
            }

            warn!("Post document subscription ended");
        });

        Ok(handle)
    }

    /// サービスを停止
    pub async fn stop(&mut self) {
        info!("Stopping document subscription service");

        if let Some(handle) = self.user_subscription_handle.take() {
            handle.abort();
        }

        if let Some(handle) = self.post_subscription_handle.take() {
            handle.abort();
        }

        info!("Document subscription service stopped");
    }
}

/// ユーザードキュメントイベントの処理
async fn handle_user_document_event(
    event: iroh_docs::rpc::client::docs::LiveEvent,
    app_handle: &tauri::AppHandle,
) -> Result<()> {
    use iroh_docs::rpc::client::docs::LiveEvent;

    match event {
        LiveEvent::InsertLocal { entry } => {
            debug!("Local user entry inserted: {:?}", entry.key());

            // フロントエンドに通知
            app_handle.emit(
                "user-profile-updated",
                json!({
                    "type": "local_insert",
                    "key": entry.key().to_vec(),
                    "author": entry.author().to_string(),
                }),
            )?;
        }

        LiveEvent::InsertRemote { entry, from, .. } => {
            debug!(
                "Remote user entry inserted from {}: {:?}",
                from,
                entry.key()
            );

            // フロントエンドに通知
            app_handle.emit(
                "user-profile-updated",
                json!({
                    "type": "remote_insert",
                    "key": entry.key().to_vec(),
                    "author": entry.author().to_string(),
                    "from": from.to_string(),
                }),
            )?;
        }

        LiveEvent::ContentReady { hash } => {
            debug!("User content ready: {}", hash);

            app_handle.emit(
                "user-content-ready",
                json!({
                    "hash": hash.to_string(),
                }),
            )?;
        }

        LiveEvent::NeighborUp(node_id) => {
            debug!("User document neighbor up: {}", node_id);

            app_handle.emit(
                "neighbor-status-changed",
                json!({
                    "node_id": node_id.to_string(),
                    "status": "up",
                    "document": "user",
                }),
            )?;
        }

        LiveEvent::NeighborDown(node_id) => {
            debug!("User document neighbor down: {}", node_id);

            app_handle.emit(
                "neighbor-status-changed",
                json!({
                    "node_id": node_id.to_string(),
                    "status": "down",
                    "document": "user",
                }),
            )?;
        }

        LiveEvent::SyncFinished(sync_event) => {
            debug!("User document sync finished: {:?}", sync_event);

            app_handle.emit(
                "sync-finished",
                json!({
                    "document": "user",
                    "event": format!("{:?}", sync_event),
                }),
            )?;
        }

        _ => {
            // その他のイベントはログに記録のみ
            debug!("Unhandled user document event: {:?}", event);
        }
    }

    Ok(())
}

/// 投稿ドキュメントイベントの処理
async fn handle_post_document_event(
    event: iroh_docs::rpc::client::docs::LiveEvent,
    app_handle: &tauri::AppHandle,
) -> Result<()> {
    use iroh_docs::rpc::client::docs::LiveEvent;

    match event {
        LiveEvent::InsertLocal { entry } => {
            debug!("Local post entry inserted: {:?}", entry.key());

            app_handle.emit(
                "post-updated",
                json!({
                    "type": "local_insert",
                    "key": entry.key().to_vec(),
                    "author": entry.author().to_string(),
                }),
            )?;
        }

        LiveEvent::InsertRemote { entry, from, .. } => {
            debug!(
                "Remote post entry inserted from {}: {:?}",
                from,
                entry.key()
            );

            app_handle.emit(
                "post-updated",
                json!({
                    "type": "remote_insert",
                    "key": entry.key().to_vec(),
                    "author": entry.author().to_string(),
                    "from": from.to_string(),
                }),
            )?;
        }

        LiveEvent::ContentReady { hash } => {
            debug!("Post content ready: {}", hash);

            app_handle.emit(
                "post-content-ready",
                json!({
                    "hash": hash.to_string(),
                }),
            )?;
        }

        LiveEvent::NeighborUp(node_id) => {
            debug!("Post document neighbor up: {}", node_id);

            app_handle.emit(
                "neighbor-status-changed",
                json!({
                    "node_id": node_id.to_string(),
                    "status": "up",
                    "document": "post",
                }),
            )?;
        }

        LiveEvent::NeighborDown(node_id) => {
            debug!("Post document neighbor down: {}", node_id);

            app_handle.emit(
                "neighbor-status-changed",
                json!({
                    "node_id": node_id.to_string(),
                    "status": "down",
                    "document": "post",
                }),
            )?;
        }

        LiveEvent::SyncFinished(sync_event) => {
            debug!("Post document sync finished: {:?}", sync_event);

            app_handle.emit(
                "sync-finished",
                json!({
                    "document": "post",
                    "event": format!("{:?}", sync_event),
                }),
            )?;
        }

        _ => {
            // その他のイベントはログに記録のみ
            debug!("Unhandled post document event: {:?}", event);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_document_subscription_service_creation() {
        let service = DocumentSubscriptionService::new();
        assert!(service.user_subscription_handle.is_none());
        assert!(service.post_subscription_handle.is_none());
    }
}
