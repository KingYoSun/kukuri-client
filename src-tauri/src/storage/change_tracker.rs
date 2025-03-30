use std::collections::HashSet;
use std::time::{Duration, Instant};

use crate::storage::models::Doc;

/// 変更の追跡を行うトラッカー
///
/// ドキュメントの変更を追跡し、新しい変更があるかどうかを判断します。
#[derive(Debug)]
pub struct ChangeTracker {
    /// 既知のエントリーIDのセット
    known_entries: HashSet<String>,
    /// 最後の更新時刻
    last_update: Instant,
}

impl ChangeTracker {
    /// 新しいChangeTrackerを作成
    pub fn new() -> Self {
        Self {
            known_entries: HashSet::new(),
            last_update: Instant::now(),
        }
    }

    /// 既知のエントリーIDで初期化
    pub fn with_entries(entries: Vec<String>) -> Self {
        let mut known_entries = HashSet::new();
        for entry in entries {
            known_entries.insert(entry);
        }

        Self {
            known_entries,
            last_update: Instant::now(),
        }
    }

    /// 新しい変更があるかチェック
    ///
    /// ドキュメントの現在のエントリーと既知のエントリーを比較し、
    /// 新しい変更があるかどうかを判断します。
    pub fn has_new_changes(&self, doc: &Doc) -> bool {
        let current_entries: HashSet<String> = doc
            .list_entries()
            .unwrap_or_default()
            .into_iter()
            .map(|entry| entry.id().to_string())
            .collect();

        !current_entries.is_subset(&self.known_entries)
    }

    /// 既知のエントリーIDを更新
    ///
    /// ドキュメントの現在のエントリーで既知のエントリーを更新します。
    pub fn update_known_entries(&mut self, doc: &Doc) {
        self.known_entries = doc
            .list_entries()
            .unwrap_or_default()
            .into_iter()
            .map(|entry| entry.id().to_string())
            .collect();

        self.last_update = Instant::now();
    }

    /// 最後の更新からの経過時間を取得
    pub fn time_since_last_update(&self) -> Duration {
        self.last_update.elapsed()
    }

    /// 既知のエントリーIDを取得
    pub fn get_known_entries(&self) -> Vec<String> {
        self.known_entries.iter().cloned().collect()
    }
}
