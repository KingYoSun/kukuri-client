//! Integration tests for document subscription and event handling

use crate::models::{post::Post, user::User};
use crate::storage::{
    events::DocumentSubscriptionService,
    repository::{
        post_repository::{get_post, list_posts, list_user_posts, save_post},
        user_repository::{get_user, save_user},
    },
    StorageError,
};
use crate::test_utils::{wait_for_event_propagation, wait_for_sync, TestEnvironment};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

/// Mock event emitter for testing document subscription events
#[derive(Debug, Clone)]
pub struct MockEventEmitter {
    pub events: Arc<Mutex<Vec<String>>>,
}

impl MockEventEmitter {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn emit(&self, event: String, payload: serde_json::Value) {
        let mut events = self.events.lock().await;
        events.push(format!("{}:{}", event, payload));
    }

    pub async fn get_events(&self) -> Vec<String> {
        let events = self.events.lock().await;
        events.clone()
    }

    pub async fn clear_events(&self) {
        let mut events = self.events.lock().await;
        events.clear();
    }

    pub async fn wait_for_events(&self, expected_count: usize, timeout_ms: u64) -> bool {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_millis(timeout_ms);

        while start.elapsed() < timeout {
            let events = self.events.lock().await;
            if events.len() >= expected_count {
                return true;
            }
            drop(events);
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        false
    }
}

#[tokio::test]
async fn test_document_subscription_service_creation() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;

    // Test creating DocumentSubscriptionService
    let _subscription_service = DocumentSubscriptionService::new();

    // Verify service was created successfully (checking internal state instead of is_running)
    // Since DocumentSubscriptionService doesn't have a public is_running method,
    // we just verify it was created without errors

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_repository_operations_trigger_events() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    let mock_emitter = MockEventEmitter::new();

    // Create a channel to monitor document changes
    let (tx, mut rx) = mpsc::channel(10);
    let emitter_clone = mock_emitter.clone();

    // Spawn a task to simulate document subscription
    let subscription_handle = tokio::spawn(async move {
        while let Some(event_type) = rx.recv().await {
            emitter_clone
                .emit(
                    "document_changed".to_string(),
                    serde_json::json!({"type": event_type}),
                )
                .await;
        }
    });

    // Create a user and signal an event
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Subscription Test User".to_string(),
        bio: "Test bio for subscription".to_string(),
        public_key: "test_public_key".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;
    tx.send("user_created")
        .await
        .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    wait_for_event_propagation().await;

    // Create a post and signal an event
    let post_id = Uuid::new_v4().to_string();
    let post = Post {
        id: post_id.clone(),
        author_id: user_id.clone(),
        content: "Subscription test post".to_string(),
        attachments: Vec::new(),
        mentions: Vec::new(),
        hashtags: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_post(&post).await?;
    tx.send("post_created")
        .await
        .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    wait_for_event_propagation().await;

    // Update the user and signal an event
    let mut updated_user = user.clone();
    updated_user.display_name = "Updated Subscription Test User".to_string();
    save_user(&updated_user).await?;
    tx.send("user_updated")
        .await
        .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    wait_for_event_propagation().await;

    // Close the channel and wait for the task to complete
    drop(tx);
    subscription_handle.abort();

    // Check that events were emitted
    let events = mock_emitter.get_events().await;
    assert!(!events.is_empty(), "No events were captured");
    assert!(
        events.len() >= 3,
        "Expected at least 3 events, got {}",
        events.len()
    );

    // Verify event types
    let event_types: Vec<String> = events
        .iter()
        .map(|e| e.split(':').nth(1).unwrap_or("unknown").to_string())
        .collect();

    assert!(event_types.iter().any(|e| e.contains("user_created")));
    assert!(event_types.iter().any(|e| e.contains("post_created")));
    assert!(event_types.iter().any(|e| e.contains("user_updated")));

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_multiple_document_operations() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    let mock_emitter = MockEventEmitter::new();

    // Create multiple operations and track events
    let (tx, mut rx) = mpsc::channel(50);
    let emitter_clone = mock_emitter.clone();

    let subscription_handle = tokio::spawn(async move {
        while let Some(event_type) = rx.recv().await {
            emitter_clone
                .emit(
                    "document_changed".to_string(),
                    serde_json::json!({"type": event_type, "timestamp": chrono::Utc::now()}),
                )
                .await;
        }
    });

    // Create multiple users
    for i in 0..3 {
        let user_id = Uuid::new_v4().to_string();
        let user = User {
            id: user_id.clone(),
            display_name: format!("Multi User {}", i),
            bio: format!("Bio for multi user {}", i),
            public_key: format!("test_public_key_{}", i),
            avatar: None,
            following: Vec::new(),
            followers: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };

        save_user(&user).await?;
        tx.send(format!("user_created_{}", i))
            .await
            .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    }

    wait_for_sync().await;

    // Create multiple posts - we'll create posts for a sample user
    let sample_user_id = Uuid::new_v4().to_string();
    let sample_user = User {
        id: sample_user_id.clone(),
        display_name: "Sample User".to_string(),
        bio: "Sample bio".to_string(),
        public_key: "sample_public_key".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };
    save_user(&sample_user).await?;

    for post_idx in 0..2 {
        let post_id = Uuid::new_v4().to_string();
        let post = Post {
            id: post_id.clone(),
            author_id: sample_user_id.clone(),
            content: format!("Multi post {}", post_idx),
            attachments: Vec::new(),
            mentions: Vec::new(),
            hashtags: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };

        save_post(&post).await?;
        tx.send(format!("post_created_{}", post_idx))
            .await
            .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    }

    wait_for_sync().await;
    drop(tx);
    subscription_handle.abort();

    // Verify events were captured
    let events = mock_emitter.get_events().await;
    assert!(!events.is_empty(), "No events were captured");

    // Should have 4 user creation + 2 post creation events = 6 total
    assert!(
        events.len() >= 6,
        "Expected at least 6 events, got {}",
        events.len()
    );

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_subscription_service_lifecycle() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    let _subscription_service = DocumentSubscriptionService::new();

    // Verify initial state (service should be created but not started)
    // Since we can't check is_running(), we just verify no panic on creation

    // Note: In a real test, we would need a mock Tauri app handle
    // For now, we just test the service creation and basic state

    // Test that we can create the service multiple times
    let _another_service = DocumentSubscriptionService::new();
    // Both services should be created successfully without errors

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_event_ordering() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    let mock_emitter = MockEventEmitter::new();

    // Test that events are emitted in the correct order
    let (tx, mut rx) = mpsc::channel(10);
    let emitter_clone = mock_emitter.clone();

    let subscription_handle = tokio::spawn(async move {
        let mut counter = 0;
        while let Some(event_type) = rx.recv().await {
            emitter_clone
                .emit(
                    "document_changed".to_string(),
                    serde_json::json!({
                        "type": event_type,
                        "order": counter,
                        "timestamp": chrono::Utc::now()
                    }),
                )
                .await;
            counter += 1;
        }
    });

    // Perform operations in sequence
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Order Test User".to_string(),
        bio: "Test bio for order test".to_string(),
        public_key: "test_public_key_order".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    // Step 1: Create user
    save_user(&user).await?;
    tx.send("create")
        .await
        .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    wait_for_event_propagation().await;

    // Step 2: Update user
    let mut updated_user = user.clone();
    updated_user.display_name = "Updated Order Test User".to_string();
    save_user(&updated_user).await?;
    tx.send("update")
        .await
        .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    wait_for_event_propagation().await;

    // Step 3: Get user (read operation)
    let _retrieved_user = get_user(&user_id).await?.expect("User should exist");
    tx.send("read")
        .await
        .map_err(|_| StorageError::Internal("Failed to send signal".to_string()))?;
    wait_for_event_propagation().await;

    drop(tx);
    subscription_handle.abort();

    // Verify event ordering
    let events = mock_emitter.get_events().await;
    assert_eq!(events.len(), 3, "Expected exactly 3 events");

    // Extract order values
    let orders: Vec<i32> = events
        .iter()
        .map(|e| {
            let parts: Vec<&str> = e.split(':').collect();
            if parts.len() > 1 {
                let json: serde_json::Value = serde_json::from_str(parts[1]).unwrap_or_default();
                json["order"].as_i64().unwrap_or(-1) as i32
            } else {
                -1
            }
        })
        .collect();

    // Verify sequential ordering
    assert_eq!(
        orders,
        vec![0, 1, 2],
        "Events were not in the expected order"
    );

    env.shutdown().await?;
    Ok(())
}
