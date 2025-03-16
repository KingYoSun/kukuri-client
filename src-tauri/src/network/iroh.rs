use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::time::Duration;

use crate::models::user::User;
use crate::models::post::Post;

// メッセージタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    NewPost(Post),
    UpdateProfile(User),
    Follow { from_id: String, to_id: String },
    Unfollow { from_id: String, to_id: String },
    SyncRequest { heads: Vec<String> },
    SyncResponse { changes: Vec<u8> },
}

// モック実装 - 実際のiroh-gossipの代わりに使用
// 実際の実装では、iroh-gossipのAPIに合わせて実装する
struct MockNetwork {
    messages: Vec<(String, Vec<u8>)>,
}

impl MockNetwork {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    fn publish(&mut self, topic: &str, message: &[u8]) -> Result<(), String> {
        self.messages.push((topic.to_string(), message.to_vec()));
        Ok(())
    }
}

// グローバルなモックネットワークインスタンス
static NETWORK: Lazy<Arc<Mutex<Option<MockNetwork>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

// ネットワークの初期化
pub fn initialize_network() -> Result<(), String> {
    let mut network_guard = NETWORK.lock().unwrap();
    
    if network_guard.is_some() {
        return Ok(());
    }
    
    // モックネットワークの作成
    *network_guard = Some(MockNetwork::new());
    
    Ok(())
}

// メッセージの送信
fn publish_message(topic_name: &str, message: &MessageType) -> Result<(), String> {
    let message_bytes = serde_json::to_vec(message)
        .map_err(|e| format!("Failed to serialize message: {}", e))?;
    
    let mut network_guard = NETWORK.lock().unwrap();
    let network = network_guard.as_mut()
        .ok_or_else(|| "Network not initialized".to_string())?;
    
    network.publish(topic_name, &message_bytes)
}

// 投稿の発信
pub fn publish_post(post: &Post) -> Result<(), String> {
    let message = MessageType::NewPost(post.clone());
    
    // グローバルフィードに発信
    publish_message("global/posts", &message)?;
    
    // 作成者のフィードにも発信
    let author_topic = format!("user/{}/posts", post.author_id);
    publish_message(&author_topic, &message)
}

// プロフィール更新の発信
pub fn publish_profile(user: &User) -> Result<(), String> {
    let message = MessageType::UpdateProfile(user.clone());
    let topic_name = format!("user/{}/profile", user.id);
    
    publish_message(&topic_name, &message)
}

// フォロー関係の発信
pub fn publish_follow(from_id: &str, to_id: &str) -> Result<(), String> {
    let message = MessageType::Follow {
        from_id: from_id.to_string(),
        to_id: to_id.to_string(),
    };
    
    let topic_name = format!("user/{}/following", from_id);
    publish_message(&topic_name, &message)
}

// フォロー解除の発信
pub fn publish_unfollow(from_id: &str, to_id: &str) -> Result<(), String> {
    let message = MessageType::Unfollow {
        from_id: from_id.to_string(),
        to_id: to_id.to_string(),
    };
    
    let topic_name = format!("user/{}/following", from_id);
    publish_message(&topic_name, &message)
}