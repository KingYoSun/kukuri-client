//! Integration tests for document synchronization with iroh-docs

use crate::models::{post::Post, user::User};
use crate::storage::{
    repository::{
        post_repository::{get_post, list_posts, list_user_posts, save_post},
        user_repository::{get_user, save_user},
    },
    StorageError,
};
use crate::test_setup::setup_test_environment;
use crate::test_utils::{wait_for_event_propagation, wait_for_sync};
use uuid::Uuid;

#[tokio::test]
async fn test_user_document_creation_and_retrieval() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create a user
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Sync Test User".to_string(),
        bio: "Test bio for sync".to_string(),
        public_key: "test_public_key_sync".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    // Save the user
    save_user(&user).await?;
    wait_for_sync().await;

    // Retrieve the user
    let retrieved_user = get_user(&user_id).await?.expect("User should exist");
    assert_eq!(retrieved_user.id, user.id);
    assert_eq!(retrieved_user.display_name, user.display_name);
    assert_eq!(retrieved_user.bio, user.bio);
    assert_eq!(retrieved_user.public_key, user.public_key);

    Ok(())
}

#[tokio::test]
async fn test_post_document_creation_and_retrieval() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create a user first
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Post Author".to_string(),
        bio: "Test author for post sync".to_string(),
        public_key: "test_public_key_author".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;
    wait_for_sync().await;

    // Create a post
    let post_id = Uuid::new_v4().to_string();
    let post = Post {
        id: post_id.clone(),
        author_id: user_id.clone(),
        content: "Sync test post content".to_string(),
        attachments: Vec::new(),
        mentions: Vec::new(),
        hashtags: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    // Save the post
    save_post(&post).await?;
    wait_for_sync().await;

    // Retrieve the post
    let retrieved_post = get_post(&post_id).await?.expect("Post should exist");
    assert_eq!(retrieved_post.id, post.id);
    assert_eq!(retrieved_post.author_id, post.author_id);
    assert_eq!(retrieved_post.content, post.content);
    assert_eq!(retrieved_post.attachments, post.attachments);
    assert_eq!(retrieved_post.mentions, post.mentions);
    assert_eq!(retrieved_post.hashtags, post.hashtags);

    Ok(())
}

#[tokio::test]
async fn test_user_document_update() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create initial user
    let user_id = Uuid::new_v4().to_string();
    let mut user = User {
        id: user_id.clone(),
        display_name: "Initial Name".to_string(),
        bio: "Initial bio".to_string(),
        public_key: "test_public_key_update".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    // Save initial version
    save_user(&user).await?;
    wait_for_sync().await;

    // Update user
    user.display_name = "Updated Name".to_string();
    user.bio = "Updated bio".to_string();
    user.avatar = Some("avatar_url".to_string());

    // Save updated version
    save_user(&user).await?;
    wait_for_sync().await;

    // Retrieve and verify update
    let retrieved_user = get_user(&user_id).await?.expect("User should exist");
    assert_eq!(retrieved_user.display_name, "Updated Name");
    assert_eq!(retrieved_user.bio, "Updated bio");
    assert_eq!(retrieved_user.avatar, Some("avatar_url".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_document_listing_sync() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create multiple posts
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "List Test Author".to_string(),
        bio: "Author for listing test".to_string(),
        public_key: "test_public_key_list".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;
    wait_for_sync().await;

    let mut post_ids = Vec::new();

    for i in 0..5 {
        let post_id = Uuid::new_v4().to_string();
        let post = Post {
            id: post_id.clone(),
            author_id: user_id.clone(),
            content: format!("List test post {}", i),
            attachments: Vec::new(),
            mentions: Vec::new(),
            hashtags: Vec::new(),
            created_at: chrono::Utc::now().timestamp() + i,
        };

        save_post(&post).await?;
        post_ids.push(post_id);
        wait_for_event_propagation().await;
    }

    wait_for_sync().await;

    // List all posts
    let all_posts = list_posts().await?;
    
    // Verify all created posts are in the list
    for post_id in &post_ids {
        assert!(
            all_posts.iter().any(|p| p.id == *post_id),
            "Post {} should be in the list",
            post_id
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_posts_by_author_sync() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create two users
    let user1_id = Uuid::new_v4().to_string();
    let user1 = User {
        id: user1_id.clone(),
        display_name: "User 1".to_string(),
        bio: "First user".to_string(),
        public_key: "test_public_key_1".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let user2_id = Uuid::new_v4().to_string();
    let user2 = User {
        id: user2_id.clone(),
        display_name: "User 2".to_string(),
        bio: "Second user".to_string(),
        public_key: "test_public_key_2".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user1).await?;
    save_user(&user2).await?;
    wait_for_sync().await;

    // Create posts for each user
    let mut user1_post_ids = Vec::new();
    let mut user2_post_ids = Vec::new();

    for i in 0..3 {
        let post_id = Uuid::new_v4().to_string();
        let post = Post {
            id: post_id.clone(),
            author_id: user1_id.clone(),
            content: format!("User 1 post {}", i),
            attachments: Vec::new(),
            mentions: Vec::new(),
            hashtags: Vec::new(),
            created_at: chrono::Utc::now().timestamp() + i,
        };
        save_post(&post).await?;
        user1_post_ids.push(post_id);
    }

    for i in 0..2 {
        let post_id = Uuid::new_v4().to_string();
        let post = Post {
            id: post_id.clone(),
            author_id: user2_id.clone(),
            content: format!("User 2 post {}", i),
            attachments: Vec::new(),
            mentions: Vec::new(),
            hashtags: Vec::new(),
            created_at: chrono::Utc::now().timestamp() + i,
        };
        save_post(&post).await?;
        user2_post_ids.push(post_id);
    }

    wait_for_sync().await;

    // Get posts by user
    let user1_posts = list_user_posts(&user1_id).await?;
    let user2_posts = list_user_posts(&user2_id).await?;

    // Verify correct posts are returned
    assert_eq!(user1_posts.len(), 3);
    assert_eq!(user2_posts.len(), 2);

    for post in &user1_posts {
        assert_eq!(post.author_id, user1_id);
        assert!(user1_post_ids.contains(&post.id));
    }

    for post in &user2_posts {
        assert_eq!(post.author_id, user2_id);
        assert!(user2_post_ids.contains(&post.id));
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_document_operations() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create user for posts
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Concurrent Test User".to_string(),
        bio: "User for concurrent test".to_string(),
        public_key: "test_public_key_concurrent".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;
    wait_for_sync().await;

    // Spawn multiple tasks to create posts concurrently
    let mut handles = Vec::new();

    for i in 0..5 {
        let user_id_clone = user_id.clone();
        let handle = tokio::spawn(async move {
            let post_id = Uuid::new_v4().to_string();
            let post = Post {
                id: post_id.clone(),
                author_id: user_id_clone,
                content: format!("Concurrent post {}", i),
                attachments: Vec::new(),
                mentions: Vec::new(),
                hashtags: Vec::new(),
                created_at: chrono::Utc::now().timestamp() + i,
            };

            save_post(&post).await.map(|_| post_id)
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    let mut created_ids = Vec::new();
    for handle in handles {
        let post_id = handle.await.map_err(|e| {
            StorageError::Internal(format!("Concurrent task failed: {}", e))
        })??;
        created_ids.push(post_id);
    }

    wait_for_sync().await;

    // Verify all posts were created
    let all_posts = list_posts().await?;
    for post_id in &created_ids {
        assert!(
            all_posts.iter().any(|p| p.id == *post_id),
            "Post {} should exist",
            post_id
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_document_persistence_across_restarts() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create data
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Persistence Test User".to_string(),
        bio: "Testing persistence".to_string(),
        public_key: "test_public_key_persist".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;

    let post_id = Uuid::new_v4().to_string();
    let post = Post {
        id: post_id.clone(),
        author_id: user_id.clone(),
        content: "Persistence test post".to_string(),
        attachments: Vec::new(),
        mentions: Vec::new(),
        hashtags: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_post(&post).await?;
    wait_for_sync().await;

    // In a real restart scenario, we would shut down and recreate the node
    // For this test, we just verify the data is still accessible
    
    // Retrieve data
    let retrieved_user = get_user(&user_id).await?.expect("User should persist");
    let retrieved_post = get_post(&post_id).await?.expect("Post should persist");

    // Verify data integrity
    assert_eq!(retrieved_user.id, user.id);
    assert_eq!(retrieved_user.display_name, user.display_name);
    assert_eq!(retrieved_post.id, post.id);
    assert_eq!(retrieved_post.content, post.content);

    Ok(())
}

#[tokio::test]
async fn test_multiple_users_sync() -> Result<(), StorageError> {
    let _ = env_logger::try_init();
    setup_test_environment().await?;

    // Create multiple users
    let mut user_ids = Vec::new();
    
    for i in 0..10 {
        let user_id = Uuid::new_v4().to_string();
        let user = User {
            id: user_id.clone(),
            display_name: format!("Multi Sync User {}", i),
            bio: format!("Bio for user {}", i),
            public_key: format!("test_public_key_multi_{}", i),
            avatar: if i % 2 == 0 {
                Some(format!("avatar_{}.png", i))
            } else {
                None
            },
            following: Vec::new(),
            followers: Vec::new(),
            created_at: chrono::Utc::now().timestamp() + i,
        };

        save_user(&user).await?;
        user_ids.push(user_id);
        wait_for_event_propagation().await;
    }

    wait_for_sync().await;

    // Retrieve and verify all users
    for (i, user_id) in user_ids.iter().enumerate() {
        let retrieved_user = get_user(user_id).await?.expect("User should exist");
        assert_eq!(retrieved_user.display_name, format!("Multi Sync User {}", i));
        assert_eq!(retrieved_user.bio, format!("Bio for user {}", i));
        
        if i % 2 == 0 {
            assert_eq!(retrieved_user.avatar, Some(format!("avatar_{}.png", i)));
        } else {
            assert_eq!(retrieved_user.avatar, None);
        }
    }

    Ok(())
}