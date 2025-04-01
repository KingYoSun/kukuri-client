use crate::models::user::User;
use serde::{Deserialize, Serialize};
use tauri::command;

/// プロフィールエラー
///
/// プロフィール操作中に発生する可能性のあるエラーを定義します。
#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    /// ストレージエラー
    #[error("Storage error: {0}")]
    Storage(String),

    /// ネットワークエラー
    #[error("Network error: {0}")]
    Network(String),

    /// ユーザーが見つからない
    #[error("User not found")]
    UserNotFound,

    /// 入力検証エラー
    #[error("Validation error: {0}")]
    Validation(String),

    /// その他のエラー
    #[error("{0}")]
    Other(String),
}

// Implement From<StorageError> for ProfileError
impl From<crate::storage::StorageError> for ProfileError {
    fn from(err: crate::storage::StorageError) -> Self {
        ProfileError::Storage(err.to_string())
    }
}

/// エラーのシリアライズ実装
impl Serialize for ProfileError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// プロフィール更新結果
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileUpdateResult {
    pub success: bool,
    pub message: Option<String>,
}

/// プロフィール取得コマンド
///
/// 指定されたユーザーIDのプロフィールを取得します。
#[command]
pub async fn get_profile(user_id: String) -> Result<Option<User>, ProfileError> {
    crate::storage::repository::user_repository::get_user(&user_id)
        .await
        .map_err(Into::into) // Convert StorageError using From impl
}

/// プロフィール更新コマンド
///
/// ユーザープロフィールを更新します。
#[command]
pub async fn update_profile(
    user_id: String,
    display_name: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
) -> Result<ProfileUpdateResult, ProfileError> {
    // 入力検証
    if let Some(ref display_name) = display_name {
        if display_name.trim().is_empty() {
            return Err(ProfileError::Validation(
                "Display name cannot be empty".to_string(),
            ));
        }
        if display_name.len() > 50 {
            return Err(ProfileError::Validation(
                "Display name exceeds maximum length of 50 characters".to_string(),
            ));
        }
    }

    if let Some(ref bio) = bio {
        if bio.len() > 160 {
            return Err(ProfileError::Validation(
                "Bio exceeds maximum length of 160 characters".to_string(),
            ));
        }
    }

    // 1. 既存のプロフィールを取得
    let user = crate::storage::repository::user_repository::get_user(&user_id)
        .await // Updated path and added .await
        .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))? // Convert error to string
        .ok_or(ProfileError::UserNotFound)?;

    // 2. 提供されたフィールドを更新
    let mut updated_user = user.clone();

    if let Some(display_name) = display_name {
        updated_user.display_name = display_name;
    }

    if let Some(bio) = bio {
        updated_user.bio = bio;
    }

    if let Some(avatar) = avatar {
        updated_user.avatar = Some(avatar);
    }

    // 3. 更新されたプロフィールを保存
    crate::storage::repository::user_repository::save_user(&updated_user)
        .await // Updated path and added .await
        .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))?; // Convert error to string

    // 4. iroh-gossipでプロフィール更新を発信
    match crate::network::iroh::publish_profile(&updated_user).await {
        Ok(_) => Ok(ProfileUpdateResult {
            success: true,
            message: None,
        }),
        Err(e) => {
            // ネットワーク発信に失敗してもプロフィールは更新されている
            println!("Warning: Failed to publish profile update: {}", e);
            Ok(ProfileUpdateResult {
                success: true,
                message: Some(format!("Profile updated but failed to publish: {}", e)),
            })
        }
    }
}

/// フォローコマンド
///
/// 指定されたユーザーをフォローします。
#[command]
pub async fn follow_user(
    user_id: String,
    target_user_id: String,
) -> Result<ProfileUpdateResult, ProfileError> {
    // 自分自身をフォローしようとしていないか確認
    if user_id == target_user_id {
        return Err(ProfileError::Validation(
            "Cannot follow yourself".to_string(),
        ));
    }

    // 1. 現在のユーザープロフィールを取得
    let user = crate::storage::repository::user_repository::get_user(&user_id)
        .await // Updated path and added .await
        .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))? // Convert error to string
        .ok_or(ProfileError::UserNotFound)?;

    // 2. フォローリストに追加（重複確認）
    let mut updated_user = user.clone();

    if !updated_user.following.contains(&target_user_id) {
        // ターゲットユーザーが存在するか確認
        let target_exists = crate::storage::repository::user_repository::get_user(&target_user_id)
            .await // Updated path and added .await
            .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))? // Convert error to string
            .is_some();

        if !target_exists {
            return Err(ProfileError::UserNotFound);
        }

        updated_user.following.push(target_user_id.clone());

        // 3. 更新されたプロフィールを保存
        crate::storage::repository::user_repository::save_user(&updated_user)
            .await // Updated path and added .await
            .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))?; // Convert error to string

        // 4. フォロー関係を発信
        match crate::network::iroh::publish_follow(&user_id, &target_user_id).await {
            Ok(_) => Ok(ProfileUpdateResult {
                success: true,
                message: None,
            }),
            Err(e) => {
                println!("Warning: Failed to publish follow relationship: {}", e);
                Ok(ProfileUpdateResult {
                    success: true,
                    message: Some(format!("Follow successful but failed to publish: {}", e)),
                })
            }
        }
    } else {
        // 既にフォロー済み
        Ok(ProfileUpdateResult {
            success: true,
            message: Some("Already following this user".to_string()),
        })
    }
}

/// フォロー解除コマンド
///
/// 指定されたユーザーのフォローを解除します。
#[command]
pub async fn unfollow_user(
    user_id: String,
    target_user_id: String,
) -> Result<ProfileUpdateResult, ProfileError> {
    // 1. 現在のユーザープロフィールを取得
    let user = crate::storage::repository::user_repository::get_user(&user_id)
        .await // Updated path and added .await
        .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))? // Convert error to string
        .ok_or(ProfileError::UserNotFound)?;

    // 2. フォローリストから削除
    let mut updated_user = user.clone();

    // フォローしているかどうかを確認
    let was_following = updated_user.following.contains(&target_user_id);

    updated_user.following.retain(|id| id != &target_user_id);

    // 3. 更新されたプロフィールを保存
    crate::storage::repository::user_repository::save_user(&updated_user)
        .await // Updated path and added .await
        .map_err(|e: crate::storage::StorageError| ProfileError::Storage(e.to_string()))?; // Convert error to string

    // 4. フォロー解除を発信
    match crate::network::iroh::publish_unfollow(&user_id, &target_user_id).await {
        Ok(_) => {
            if was_following {
                Ok(ProfileUpdateResult {
                    success: true,
                    message: None,
                })
            } else {
                Ok(ProfileUpdateResult {
                    success: true,
                    message: Some("User was not being followed".to_string()),
                })
            }
        }
        Err(e) => {
            println!("Warning: Failed to publish unfollow relationship: {}", e);
            Ok(ProfileUpdateResult {
                success: true,
                message: Some(format!("Unfollow successful but failed to publish: {}", e)),
            })
        }
    }
}

// テストコードは省略
