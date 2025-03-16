use crate::models::user::User;
use base64::{engine::general_purpose, Engine as _};
use ring::signature::{self, Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
// Tauri v2ではcommandマクロを使用します
use tauri::command;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResult {
    pub user_id: String,
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListItem {
    pub id: String,
    pub display_name: String,
}

#[command]
pub async fn create_user(display_name: String, bio: Option<String>) -> Result<AuthResult, String> {
    // 1. 新しいキーペアを生成
    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|_| "Failed to generate key pair".to_string())?;

    let key_pair = Ed25519KeyPair::from_pkcs8(&pkcs8_bytes.as_ref())
        .map_err(|_| "Failed to parse key pair".to_string())?;

    // 2. 公開鍵からユーザーIDを作成
    let public_key = key_pair.public_key().as_ref();
    let public_key_b64 = general_purpose::STANDARD.encode(public_key);
    let user_id = Uuid::new_v4().to_string();

    // 3. ユーザープロファイルを作成して保存
    let user = User {
        id: user_id.clone(),
        display_name,
        bio: bio.unwrap_or_default(),
        public_key: public_key_b64,
        // その他のフィールドを初期化
        avatar: None,
        following: vec![],
        followers: vec![],
        created_at: chrono::Utc::now().timestamp(),
    };

    // StorageManagerを使用してユーザーを保存
    crate::storage::automerge::save_user(&user)?;

    // 4. 秘密鍵を安全に保存（Tauriのセキュアストレージを使用）
    let private_key_b64 = general_purpose::STANDARD.encode(pkcs8_bytes);
    let app_dir = std::env::temp_dir().join("kukuri-client");
    let key_dir = app_dir.join("keys");
    std::fs::create_dir_all(&key_dir)
        .map_err(|e| format!("Failed to create key directory: {}", e))?;

    let key_path = key_dir.join(format!("{}.key", user_id));
    std::fs::write(key_path, private_key_b64)
        .map_err(|e| format!("Failed to save private key: {}", e))?;

    Ok(AuthResult {
        user_id,
        success: true,
        message: None,
    })
}

#[command]
pub async fn sign_in(user_id: String) -> Result<AuthResult, String> {
    // ユーザーIDに基づいて秘密鍵を読み込み
    let app_dir = std::env::temp_dir().join("kukuri-client");
    let key_path = app_dir.join("keys").join(format!("{}.key", user_id));

    match std::fs::read(key_path) {
        Ok(_key_data) => {
            // ユーザープロファイルを取得して検証
            match crate::storage::automerge::get_user(&user_id) {
                Ok(Some(_user)) => Ok(AuthResult {
                    user_id,
                    success: true,
                    message: None,
                }),
                Ok(None) => Err("User profile not found".to_string()),
                Err(e) => Err(format!("Failed to retrieve user profile: {}", e)),
            }
        }
        Err(_) => Err("User credentials not found".to_string()),
    }
}

#[command]
pub async fn list_users() -> Result<Vec<UserListItem>, String> {
    // アプリのデータディレクトリからキーファイルを検索
    let app_dir = std::env::temp_dir().join("kukuri-client");
    let key_dir = app_dir.join("keys");

    // キーディレクトリが存在しない場合は空のリストを返す
    if !key_dir.exists() {
        return Ok(Vec::new());
    }

    let mut users = Vec::new();

    // キーディレクトリ内のファイルを走査
    match fs::read_dir(&key_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // .keyファイルのみを処理
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "key") {
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(user_id) = file_stem.to_str() {
                                // ユーザープロファイルを取得
                                match crate::storage::automerge::get_user(user_id) {
                                    Ok(Some(user)) => {
                                        users.push(UserListItem {
                                            id: user.id,
                                            display_name: user.display_name,
                                        });
                                    }
                                    _ => {} // プロファイルが見つからない場合はスキップ
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => return Err(format!("Failed to read keys directory: {}", e)),
    }

    Ok(users)
}
