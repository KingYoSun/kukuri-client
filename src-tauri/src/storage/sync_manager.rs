use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::storage::change_tracker::ChangeTracker;
use crate::storage::docs::Docs;
use crate::storage::models::Doc;

/// 同期マネージャー
///
/// 複数のピアとの同期を管理します。
#[derive(Debug)]
pub struct SyncManager {
    /// 変更トラッカー
    change_tracker: ChangeTracker,
    /// アクティブなピアのリスト
    active_peers: HashSet<String>,
    /// 実行中フラグ
    running: Arc<Mutex<bool>>,
    /// 最後の同期試行時刻
    last_sync_attempt: Instant,
    /// iroh-docsインスタンス
    docs: Arc<Docs>,
    /// ドキュメントID
    doc_id: String,
}

impl SyncManager {
    /// 新しいSyncManagerを作成
    pub fn new(docs: Arc<Docs>, doc_id: String) -> Self {
        Self {
            change_tracker: ChangeTracker::new(),
            active_peers: HashSet::new(),
            running: Arc::new(Mutex::new(false)),
            last_sync_attempt: Instant::now(),
            docs,
            doc_id,
        }
    }

    /// ピアの追加
    ///
    /// 同期対象のピアを追加します。
    pub fn add_peer(&mut self, peer_id: &str) {
        self.active_peers.insert(peer_id.to_string());
    }

    /// ピアの削除
    ///
    /// 同期対象のピアを削除します。
    pub fn remove_peer(&mut self, peer_id: &str) {
        self.active_peers.remove(peer_id);
    }

    /// アクティブなピアの取得
    ///
    /// 現在アクティブなピアのリストを取得します。
    pub fn get_active_peers(&self) -> Vec<String> {
        self.active_peers.iter().cloned().collect()
    }

    /// ドキュメントの取得
    ///
    /// 現在のドキュメントを取得します。
    pub fn get_document(&self) -> Result<Doc, String> {
        self.docs.get(&self.doc_id)
    }

    /// 同期の実行
    ///
    /// すべてのアクティブなピアとの同期を実行します。
    pub fn sync_with_peers(&mut self) -> Result<(), String> {
        let doc = self.get_document()?;

        for peer_id in &self.active_peers {
            // ピアとの同期を実行
            self.docs.sync_doc(&doc, peer_id)?;
        }

        // 変更トラッカーを更新
        self.change_tracker.update_known_entries(&doc);
        self.last_sync_attempt = Instant::now();

        Ok(())
    }

    /// 変更の検出
    ///
    /// ドキュメントに新しい変更があるかどうかを判断します。
    pub fn has_changes(&self) -> Result<bool, String> {
        let doc = self.get_document()?;
        Ok(self.change_tracker.has_new_changes(&doc))
    }

    /// 既知のエントリーIDの更新
    ///
    /// ドキュメントの現在のエントリーで既知のエントリーを更新します。
    pub fn update_known_entries(&mut self) -> Result<(), String> {
        let doc = self.get_document()?;
        self.change_tracker.update_known_entries(&doc);
        Ok(())
    }

    /// 最後の同期試行からの経過時間を取得
    pub fn time_since_last_sync_attempt(&self) -> Duration {
        self.last_sync_attempt.elapsed()
    }

    /// 同期の実行が必要かどうかを判断
    ///
    /// 以下の条件のいずれかが満たされる場合、同期の実行が必要と判断します：
    /// - ドキュメントに新しい変更がある
    /// - 最後の同期試行から一定時間が経過している
    pub fn should_sync(&self, min_interval: Duration) -> Result<bool, String> {
        Ok(self.has_changes()? || self.time_since_last_sync_attempt() > min_interval)
    }
}
