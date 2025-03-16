use crate::models::user::User;
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileUpdateResult {
    pub success: bool,
    pub message: Option<String>,
}

#[command]
pub async fn get_profile(user_id: String) -> Result<Option<User>, String> {
    crate::storage::automerge::get_user(&user_id)
        .map_err(|e| format!("Failed to retrieve user profile: {}", e))
}

#[command]
pub async fn update_profile(
    user_id: String, 
    display_name: Option<String>, 
    bio: Option<String>,
    avatar: Option<String>
) -> Result<ProfileUpdateResult, String> {
    // 1. 既存のプロフィールを取得
    match crate::storage::automerge::get_user(&user_id) {
        Ok(Some(mut user)) => {
            // 2. 提供されたフィールドを更新
            if let Some(display_name) = display_name {
                user.display_name = display_name;
            }
            
            if let Some(bio) = bio {
                user.bio = bio;
            }
            
            if let Some(avatar) = avatar {
                user.avatar = Some(avatar);
            }
            
            // 3. 更新されたプロフィールを保存
            match crate::storage::automerge::save_user(&user) {
                Ok(_) => {
                    // 4. iroh-gossipでプロフィール更新を発信
                    match crate::network::iroh::publish_profile(&user) {
                        Ok(_) => Ok(ProfileUpdateResult {
                            success: true,
                            message: None,
                        }),
                        Err(e) => {
                            // ネットワーク発信に失敗してもプロフィールは更新されている
                            Ok(ProfileUpdateResult {
                                success: true,
                                message: Some(format!("Profile updated but failed to publish: {}", e)),
                            })
                        }
                    }
                }
                Err(e) => Err(format!("Failed to save updated profile: {}", e)),
            }
        }
        Ok(None) => Err("User profile not found".to_string()),
        Err(e) => Err(format!("Failed to retrieve user profile: {}", e)),
    }
}

#[command]
pub async fn follow_user(user_id: String, target_user_id: String) -> Result<ProfileUpdateResult, String> {
    // 1. 現在のユーザープロフィールを取得
    match crate::storage::automerge::get_user(&user_id) {
        Ok(Some(mut user)) => {
            // 2. フォローリストに追加（重複確認）
            if !user.following.contains(&target_user_id) {
                user.following.push(target_user_id.clone());
                
                // 3. 更新されたプロフィールを保存
                match crate::storage::automerge::save_user(&user) {
                    Ok(_) => {
                        // 4. フォロー関係を発信
                        match crate::network::iroh::publish_follow(&user_id, &target_user_id) {
                            Ok(_) => Ok(ProfileUpdateResult {
                                success: true,
                                message: None,
                            }),
                            Err(e) => {
                                Ok(ProfileUpdateResult {
                                    success: true,
                                    message: Some(format!("Follow successful but failed to publish: {}", e)),
                                })
                            }
                        }
                    }
                    Err(e) => Err(format!("Failed to save follow relationship: {}", e)),
                }
            } else {
                // 既にフォロー済み
                Ok(ProfileUpdateResult {
                    success: true,
                    message: Some("Already following this user".to_string()),
                })
            }
        }
        Ok(None) => Err("User profile not found".to_string()),
        Err(e) => Err(format!("Failed to retrieve user profile: {}", e)),
    }
}

#[command]
pub async fn unfollow_user(user_id: String, target_user_id: String) -> Result<ProfileUpdateResult, String> {
    // フォロー解除の実装（followの逆操作）
    match crate::storage::automerge::get_user(&user_id) {
        Ok(Some(mut user)) => {
            user.following.retain(|id| id != &target_user_id);
            
            match crate::storage::automerge::save_user(&user) {
                Ok(_) => {
                    match crate::network::iroh::publish_unfollow(&user_id, &target_user_id) {
                        Ok(_) => Ok(ProfileUpdateResult {
                            success: true,
                            message: None,
                        }),
                        Err(e) => {
                            Ok(ProfileUpdateResult {
                                success: true,
                                message: Some(format!("Unfollow successful but failed to publish: {}", e)),
                            })
                        }
                    }
                }
                Err(e) => Err(format!("Failed to save unfollow relationship: {}", e)),
            }
        }
        Ok(None) => Err("User profile not found".to_string()),
        Err(e) => Err(format!("Failed to retrieve user profile: {}", e)),
    }
}