use serde::{Deserialize, Serialize};
use tauri::command;

use crate::models::settings::Settings; // Import Settings from models
use crate::storage::repository::settings_repository; // Import the repository
use crate::storage::StorageError as InternalStorageError; // Alias internal storage error

/// 設定エラー
///
/// 設定操作中に発生する可能性のあるエラーを定義します。
#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    /// ストレージエラー
    #[error("Storage error: {0}")]
    Storage(InternalStorageError), // Use the actual storage error type

    /// 入力検証エラー
    #[error("Validation error: {0}")]
    Validation(String),

    /// その他のエラー
    #[error("{0}")]
    Other(String),
}

// Implement From<StorageError> for SettingsError
impl From<InternalStorageError> for SettingsError {
    fn from(err: InternalStorageError) -> Self {
        // Directly wrap the storage error
        SettingsError::Storage(err)
    }
}

/// エラーのシリアライズ実装
impl Serialize for SettingsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// Settings struct is now imported from crate::models::settings

/// 設定更新結果
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsUpdateResult {
    pub success: bool,
    pub message: Option<String>,
}

/// 設定取得コマンド
///
/// アプリケーション設定を取得します。
#[command]
pub async fn get_settings(user_id: Option<String>) -> Result<Settings, SettingsError> {
    // Use the repository function to get settings
    match settings_repository::get_settings(user_id.as_deref()).await {
        Ok(Some(settings)) => Ok(settings),
        Ok(None) => {
            // If no settings found, return default settings, ensuring user_id is set correctly
            let mut default_settings = Settings::default();
            default_settings.user_id = user_id; // Set the provided user_id
            Ok(default_settings)
        }
        Err(e) => Err(SettingsError::from(e)), // Map StorageError to SettingsError
    }
}

/// 設定更新コマンド
///
/// アプリケーション設定を更新します。
#[command]
pub async fn update_settings(
    user_id: Option<String>,
    selected_relays: Option<Vec<String>>,
    theme: Option<String>,
    language: Option<String>,
    autostart: Option<bool>,
    notifications: Option<bool>,
) -> Result<SettingsUpdateResult, SettingsError> {
    // Get current settings or default if none exist
    let mut current_settings = settings_repository::get_settings(user_id.as_deref())
        .await
        .map_err(SettingsError::from)? // Map error
        .unwrap_or_else(|| {
            let mut default = Settings::default();
            default.user_id = user_id.clone(); // Ensure user_id is set
            default
        });

    // Update fields if provided
    if let Some(relays) = selected_relays {
        current_settings.selected_relays = relays;
    }
    if let Some(t) = theme {
        current_settings.theme = t;
    }
    if let Some(lang) = language {
        current_settings.language = lang;
    }
    if let Some(auto) = autostart {
        current_settings.autostart = auto;
    }
    if let Some(notif) = notifications {
        current_settings.notifications = notif;
    }

    // Save the updated settings using the repository function
    match settings_repository::save_settings(&current_settings).await {
        Ok(_) => Ok(SettingsUpdateResult {
            success: true,
            message: Some("Settings updated successfully.".to_string()),
        }),
        Err(e) => Err(SettingsError::from(e)), // Map StorageError to SettingsError
    }
}

// テストコードは省略
