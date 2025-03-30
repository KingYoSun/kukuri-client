use crate::models::user::User;
use base64::{engine::general_purpose, Engine as _};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::command;
use uuid::Uuid;

/// 認証エラー
///
/// 認証処理中に発生する可能性のあるエラーを定義します。
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// キーペア生成エラー
    #[error("Failed to generate key pair: {0}")]
    KeyGeneration(String),

    /// ストレージエラー
    #[error("Storage error: {0}")]
    Storage(String),

    /// ファイルシステムエラー
    #[error("File system error: {0}")]
    FileSystem(String),

    /// ユーザーが見つからない
    #[error("User not found")]
    UserNotFound,

    /// 認証情報が見つからない
    #[error("Credentials not found")]
    CredentialsNotFound,

    /// その他のエラー
    #[error("{0}")]
    Other(String),
}

/// エラーのシリアライズ実装
impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// 認証結果
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResult {
    pub user_id: String,
    pub success: bool,
    pub message: Option<String>,
}

/// ユーザーリストアイテム
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListItem {
    pub id: String,
    pub display_name: String,
}

/// ユーザー作成コマンド
///
/// 新しいユーザーを作成し、キーペアを生成して保存します。
#[command]
pub async fn create_user(
    display_name: String,
    bio: Option<String>,
) -> Result<AuthResult, AuthError> {
    // 1. 新しいキーペアを生成
    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| AuthError::KeyGeneration(format!("Failed to generate key pair: {:?}", e)))?;

    let key_pair = Ed25519KeyPair::from_pkcs8(&pkcs8_bytes.as_ref())
        .map_err(|e| AuthError::KeyGeneration(format!("Failed to parse key pair: {:?}", e)))?;

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
    crate::storage::iroh_docs_sync::save_user(&user).map_err(|e| AuthError::Storage(e))?;

    // 4. 秘密鍵を安全に保存
    let private_key_b64 = general_purpose::STANDARD.encode(pkcs8_bytes);
    let app_dir = std::env::temp_dir().join("kukuri-client");
    let key_dir = app_dir.join("keys");
    std::fs::create_dir_all(&key_dir)
        .map_err(|e| AuthError::FileSystem(format!("Failed to create key directory: {}", e)))?;

    let key_path = key_dir.join(format!("{}.key", user_id));
    std::fs::write(key_path, private_key_b64)
        .map_err(|e| AuthError::FileSystem(format!("Failed to save private key: {}", e)))?;

    // ネットワークにユーザープロファイルを発信
    if let Err(e) = crate::network::iroh::publish_profile(&user) {
        println!("Warning: Failed to publish profile: {}", e);
        // 発信に失敗しても処理は続行
    }

    Ok(AuthResult {
        user_id,
        success: true,
        message: None,
    })
}

/// サインインコマンド
///
/// 既存のユーザーでサインインします。
#[command]
pub async fn sign_in(user_id: String) -> Result<AuthResult, AuthError> {
    // ユーザーIDに基づいて秘密鍵を読み込み
    let app_dir = std::env::temp_dir().join("kukuri-client");
    let key_path = app_dir.join("keys").join(format!("{}.key", user_id));

    if !key_path.exists() {
        return Err(AuthError::CredentialsNotFound);
    }

    // ユーザープロファイルを取得して検証
    match crate::storage::iroh_docs_sync::get_user(&user_id) {
        Ok(Some(_user)) => {
            // ネットワーク状態を取得
            let network_status = crate::network::iroh::get_network_status()
                .map_err(|e| AuthError::Other(format!("Failed to get network status: {}", e)))?;

            // ネットワーク状態をログに出力
            println!("Network status: {:?}", network_status);

            Ok(AuthResult {
                user_id,
                success: true,
                message: Some(format!("Connected to {} peers", network_status.peer_count)),
            })
        }
        Ok(None) => Err(AuthError::UserNotFound),
        Err(e) => Err(AuthError::Storage(e)),
    }
}

/// ユーザーリスト取得コマンド
///
/// 利用可能なすべてのユーザーのリストを取得します。
#[command]
pub async fn list_users() -> Result<Vec<UserListItem>, AuthError> {
    // アプリのデータディレクトリからキーファイルを検索
    let app_dir = std::env::temp_dir().join("kukuri-client");
    let key_dir = app_dir.join("keys");

    // キーディレクトリが存在しない場合は空のリストを返す
    if !key_dir.exists() {
        return Ok(Vec::new());
    }

    let mut users = Vec::new();

    // キーディレクトリ内のファイルを走査
    let entries = fs::read_dir(&key_dir)
        .map_err(|e| AuthError::FileSystem(format!("Failed to read keys directory: {}", e)))?;

    for entry in entries {
        let entry = entry
            .map_err(|e| AuthError::FileSystem(format!("Failed to read directory entry: {}", e)))?;

        let path = entry.path();

        // .keyファイルのみを処理
        if path.is_file() && path.extension().map_or(false, |ext| ext == "key") {
            if let Some(file_stem) = path.file_stem() {
                if let Some(user_id) = file_stem.to_str() {
                    // ユーザープロファイルを取得
                    if let Ok(Some(user)) = crate::storage::iroh_docs_sync::get_user(user_id) {
                        users.push(UserListItem {
                            id: user.id,
                            display_name: user.display_name,
                        });
                    }
                }
            }
        }
    }

    Ok(users)
}

// テストコードは省略
