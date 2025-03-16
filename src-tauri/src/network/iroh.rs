use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::models::post::Post;
use crate::models::user::User;

/// メッセージタイプ
///
/// P2Pネットワーク上で交換されるメッセージの種類を定義します。
/// 実際の実装ではiroh-gossipを使用しますが、
/// MVPではシンプルなモック実装を使用します。
#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    /// 新しい投稿
    NewPost(Post),
    /// プロフィール更新
    UpdateProfile(User),
    /// フォロー関係
    Follow { from_id: String, to_id: String },
    /// フォロー解除
    Unfollow { from_id: String, to_id: String },
    /// 同期リクエスト
    SyncRequest { heads: Vec<String> },
    /// 同期レスポンス
    SyncResponse { changes: Vec<u8> },
}

/// モックネットワーク
///
/// 実際の実装ではiroh-gossipを使用しますが、
/// MVPではシンプルなモック実装を使用します。
struct MockNetwork {
    /// 送信されたメッセージのログ
    messages: Vec<(String, Vec<u8>)>,
    /// 接続されたピア（モック）
    peers: Vec<String>,
    /// 最後のアクティビティのタイムスタンプ
    last_activity: i64,
}

impl MockNetwork {
    /// 新しいモックネットワークを作成
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            peers: vec!["peer1".to_string(), "peer2".to_string()], // モックピア
            last_activity: chrono::Utc::now().timestamp(),
        }
    }

    /// メッセージを発行
    fn publish(&mut self, topic: &str, message: &[u8]) -> Result<(), String> {
        self.messages.push((topic.to_string(), message.to_vec()));
        self.last_activity = chrono::Utc::now().timestamp();

        // 実際の実装では、ここでP2Pネットワークにメッセージを送信します
        // モック実装では、メッセージをログに記録するだけです

        // ログメッセージ
        println!("Published message to topic: {}", topic);

        Ok(())
    }

    /// 接続されたピアの数を取得
    fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// ネットワークの状態を取得
    fn status(&self) -> NetworkStatus {
        NetworkStatus {
            peer_count: self.peer_count(),
            connected: true,
            last_activity: self.last_activity,
        }
    }
}

/// ネットワークの状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// 接続されたピアの数
    pub peer_count: usize,
    /// ネットワークに接続されているかどうか
    pub connected: bool,
    /// 最後のアクティビティのタイムスタンプ
    pub last_activity: i64,
}

// グローバルなモックネットワークインスタンス
static NETWORK: Lazy<Arc<Mutex<Option<MockNetwork>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// ネットワークの初期化
///
/// アプリケーションの起動時に呼び出され、
/// P2Pネットワークを初期化します。
pub fn initialize_network() -> Result<(), String> {
    let mut network_guard = NETWORK.lock().unwrap();

    if network_guard.is_some() {
        return Ok(());
    }

    // モックネットワークの作成
    *network_guard = Some(MockNetwork::new());

    // ログメッセージ
    println!("Network initialized");

    Ok(())
}

/// ネットワークの状態を取得
///
/// 現在のネットワークの状態を取得します。
pub fn get_network_status() -> Result<NetworkStatus, String> {
    let network_guard = NETWORK.lock().unwrap();

    match &*network_guard {
        Some(network) => Ok(network.status()),
        None => Err("Network not initialized".to_string()),
    }
}

/// メッセージの送信
///
/// 指定されたトピックにメッセージを送信します。
fn publish_message(topic_name: &str, message: &MessageType) -> Result<(), String> {
    let message_bytes =
        serde_json::to_vec(message).map_err(|e| format!("Failed to serialize message: {}", e))?;

    let mut network_guard = NETWORK.lock().unwrap();
    let network = network_guard
        .as_mut()
        .ok_or_else(|| "Network not initialized".to_string())?;

    network.publish(topic_name, &message_bytes)
}

/// 投稿の発信
///
/// 新しい投稿をP2Pネットワークに発信します。
pub fn publish_post(post: &Post) -> Result<(), String> {
    let message = MessageType::NewPost(post.clone());

    // グローバルフィードに発信
    publish_message("global/posts", &message)?;

    // 作成者のフィードにも発信
    let author_topic = format!("user/{}/posts", post.author_id);
    publish_message(&author_topic, &message)
}

/// プロフィール更新の発信
///
/// プロフィール更新をP2Pネットワークに発信します。
pub fn publish_profile(user: &User) -> Result<(), String> {
    let message = MessageType::UpdateProfile(user.clone());
    let topic_name = format!("user/{}/profile", user.id);

    publish_message(&topic_name, &message)
}

/// フォロー関係の発信
///
/// フォロー関係をP2Pネットワークに発信します。
pub fn publish_follow(from_id: &str, to_id: &str) -> Result<(), String> {
    let message = MessageType::Follow {
        from_id: from_id.to_string(),
        to_id: to_id.to_string(),
    };

    let topic_name = format!("user/{}/following", from_id);
    publish_message(&topic_name, &message)
}

/// フォロー解除の発信
///
/// フォロー解除をP2Pネットワークに発信します。
pub fn publish_unfollow(from_id: &str, to_id: &str) -> Result<(), String> {
    let message = MessageType::Unfollow {
        from_id: from_id.to_string(),
        to_id: to_id.to_string(),
    };

    let topic_name = format!("user/{}/following", from_id);
    publish_message(&topic_name, &message)
}

// テストコードは省略
