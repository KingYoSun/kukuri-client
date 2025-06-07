use futures_lite::StreamExt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::post::Post;
use crate::models::user::User;

/// メッセージタイプ
///
/// P2Pネットワーク上で交換されるメッセージの種類を定義します。
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

/// 実際のネットワーク実装
struct IrohNetwork {
    /// iroh-gossipのエンドポイント
    endpoint: iroh::Endpoint,
    /// iroh-gossipのインスタンス
    gossip: iroh_gossip::net::Gossip,
    /// トピックのマッピング
    topics: HashMap<String, iroh_gossip::proto::TopicId>,
    /// 最後のアクティビティのタイムスタンプ
    last_activity: i64,
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

// グローバルなネットワークインスタンス
static NETWORK: Lazy<Arc<Mutex<Option<IrohNetwork>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// ネットワークの初期化
///
/// アプリケーションの起動時に呼び出され、
/// P2Pネットワークを初期化します。
pub async fn initialize_network() -> Result<(), String> {
    let mut network_guard = NETWORK.lock().unwrap();

    if network_guard.is_some() {
        return Ok(());
    }

    // irohエンドポイントの作成
    let endpoint = iroh::Endpoint::builder()
        .discovery_n0()
        .bind()
        .await
        .map_err(|e| format!("Failed to create endpoint: {}", e))?;

    // iroh-gossipインスタンスの作成
    let gossip = iroh_gossip::net::Gossip::builder()
        .spawn(endpoint.clone())
        .await
        .map_err(|e| format!("Failed to create gossip: {}", e))?;

    // ルーターの設定 (未使用のためアンダースコアを追加)
    let _router = iroh::protocol::Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    // トピックのマッピングを初期化
    let topics = HashMap::new();

    // ネットワークインスタンスの作成
    *network_guard = Some(IrohNetwork {
        endpoint,
        gossip,
        topics,
        last_activity: chrono::Utc::now().timestamp(),
    });

    // ログメッセージ
    println!("Network initialized with iroh-gossip");

    Ok(())
}

/// トピックIDの取得または作成
///
/// 指定されたトピック名に対応するTopicIdを取得します。
/// トピックが存在しない場合は新しく作成します。
fn get_or_create_topic(network: &mut IrohNetwork, topic_name: &str) -> iroh_gossip::proto::TopicId {
    if let Some(topic_id) = network.topics.get(topic_name) {
        return *topic_id;
    }

    // 新しいトピックIDを作成
    let topic_id =
        iroh_gossip::proto::TopicId::from_bytes(*blake3::hash(topic_name.as_bytes()).as_bytes());

    network.topics.insert(topic_name.to_string(), topic_id);
    topic_id
}

/// ネットワークの状態を取得
///
/// 現在のネットワークの状態を取得します。
pub fn get_network_status() -> Result<NetworkStatus, String> {
    let network_guard = NETWORK.lock().unwrap();

    match &*network_guard {
        Some(network) => {
            // ピア数の取得
            // 注: 実際のiroh-gossipでは、connected_peersメソッドは存在しないため、
            // 代わりにgossipインスタンスから接続情報を取得する必要があります
            let peer_count = 0; // TODO: 実際の実装では、gossipインスタンスから接続情報を取得する

            Ok(NetworkStatus {
                peer_count,
                connected: true,
                last_activity: network.last_activity,
            })
        }
        None => Err("Network not initialized".to_string()),
    }
}

/// メッセージの送信
///
/// 指定されたトピックにメッセージを送信します。
async fn publish_message(topic_name: &str, message: &MessageType) -> Result<(), String> {
    let message_bytes =
        serde_json::to_vec(message).map_err(|e| format!("Failed to serialize message: {}", e))?;

    // MutexGuardのスコープを制限するためにブロックで囲む
    let topic = {
        let mut network_guard = NETWORK.lock().unwrap();
        let network = network_guard
            .as_mut()
            .ok_or_else(|| "Network not initialized".to_string())?;

        // トピックIDの取得または作成
        let topic_id = get_or_create_topic(network, topic_name);

        // トピックへの参加（まだ参加していない場合）
        match network.gossip.subscribe(topic_id, vec![]) {
            Ok(topic) => topic,
            Err(e) => return Err(format!("Failed to subscribe to topic: {}", e)),
        }
    }; // ここでnetwork_guardがドロップされる

    // メッセージのブロードキャスト
    topic
        .broadcast(message_bytes.into())
        .await
        .map_err(|e| format!("Failed to broadcast message: {}", e))?;

    // 最後のアクティビティのタイムスタンプを更新
    {
        let mut network_guard = NETWORK.lock().unwrap();
        if let Some(network) = network_guard.as_mut() {
            network.last_activity = chrono::Utc::now().timestamp();
        }
    }

    // ログメッセージ
    println!("Published message to topic: {}", topic_name);

    Ok(())
}

/// メッセージ受信ハンドラーの登録
///
/// 指定されたトピックのメッセージを受信するハンドラーを登録します。
pub async fn subscribe_to_topic<F>(topic_name: &str, handler: F) -> Result<(), String>
where
    F: FnMut(MessageType) -> Result<(), String> + Send + 'static,
{
    // MutexGuardのスコープを制限するためにブロックで囲む
    let topic = {
        let mut network_guard = NETWORK.lock().unwrap();
        let network = network_guard
            .as_mut()
            .ok_or_else(|| "Network not initialized".to_string())?;

        // トピックIDの取得または作成
        let topic_id = get_or_create_topic(network, topic_name);

        // トピックへの参加
        match network.gossip.subscribe(topic_id, vec![]) {
            Ok(topic) => topic,
            Err(e) => return Err(format!("Failed to subscribe to topic: {}", e)),
        }
    }; // ここでnetwork_guardがドロップされる

    // メッセージ受信ハンドラーの登録
    tokio::spawn(async move {
        let mut handler = handler;
        let mut receiver = topic.split().1;

        while let Ok(Some(event)) = receiver.try_next().await {
            if let iroh_gossip::net::Event::Gossip(iroh_gossip::net::GossipEvent::Received(msg)) =
                event
            {
                // メッセージのデシリアライズ
                if let Ok(message) = serde_json::from_slice::<MessageType>(&msg.content) {
                    // ハンドラーの呼び出し
                    if let Err(e) = handler(message) {
                        eprintln!("Error handling message: {}", e);
                    }
                }
            }
        }
    });

    Ok(())
}

/// 投稿の発信
///
/// 新しい投稿をP2Pネットワークに発信します。
pub async fn publish_post(post: &Post) -> Result<(), String> {
    let message = MessageType::NewPost(post.clone());

    // グローバルフィードに発信
    publish_message("global/posts", &message).await?;

    // 作成者のフィードにも発信
    let author_topic = format!("user/{}/posts", post.author_id);
    publish_message(&author_topic, &message).await
}

/// プロフィール更新の発信
///
/// プロフィール更新をP2Pネットワークに発信します。
pub async fn publish_profile(user: &User) -> Result<(), String> {
    let message = MessageType::UpdateProfile(user.clone());
    let topic_name = format!("user/{}/profile", user.id);

    publish_message(&topic_name, &message).await
}

/// フォロー関係の発信
///
/// フォロー関係をP2Pネットワークに発信します。
pub async fn publish_follow(from_id: &str, to_id: &str) -> Result<(), String> {
    let message = MessageType::Follow {
        from_id: from_id.to_string(),
        to_id: to_id.to_string(),
    };

    let topic_name = format!("user/{}/following", from_id);
    publish_message(&topic_name, &message).await
}

/// フォロー解除の発信
///
/// フォロー解除をP2Pネットワークに発信します。
pub async fn publish_unfollow(from_id: &str, to_id: &str) -> Result<(), String> {
    let message = MessageType::Unfollow {
        from_id: from_id.to_string(),
        to_id: to_id.to_string(),
    };

    let topic_name = format!("user/{}/following", from_id);
    publish_message(&topic_name, &message).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_initialize_network() {
        // ネットワークの初期化テスト
        let result = initialize_network().await;
        assert!(result.is_ok());
    }

    // 他のテストケースは実際の実装に合わせて追加
}
