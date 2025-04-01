use serde::{Deserialize, Serialize};

/// アプリケーション設定
///
/// ユーザーごとの設定、またはグローバルな設定を保持します。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)] // PartialEq を追加 for testing/comparison
pub struct Settings {
    /// 設定が紐づくユーザーID (Noneの場合はグローバル設定)
    pub user_id: Option<String>,
    /// 選択されたリレーサーバーのリスト
    pub selected_relays: Vec<String>,
    /// UIテーマ ("system", "light", "dark")
    pub theme: String,
    /// 表示言語 ("ja", "en" など)
    pub language: String,
    /// アプリケーションの自動起動設定
    pub autostart: bool,
    /// 通知の有効/無効設定
    pub notifications: bool,
}

impl Default for Settings {
    /// デフォルト設定を提供します。
    fn default() -> Self {
        Settings {
            user_id: None, // デフォルトはグローバル設定
            selected_relays: vec![],
            theme: "system".to_string(),
            language: "ja".to_string(),
            autostart: false,
            notifications: true,
        }
    }
}
