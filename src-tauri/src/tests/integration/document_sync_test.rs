//! Integration tests for document synchronization with iroh-docs

use crate::models::{post::Post, user::User};
use crate::storage::{
    repository::{
        post_repository::{get_post, list_posts, list_user_posts, save_post},
        user_repository::{get_user, save_user},
    },
    StorageError,
};
use crate::test_utils::{wait_for_event_propagation, wait_for_sync, TestEnvironment};
use uuid::Uuid;

#[tokio::test]
async fn test_user_document_creation_and_retrieval() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

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

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_post_document_creation_and_retrieval() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

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

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_user_document_update() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

    // Create initial user
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Original Name".to_string(),
        bio: "Original bio".to_string(),
        public_key: "test_public_key_update".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;
    wait_for_sync().await;

    // Update the user
    let mut updated_user = user.clone();
    updated_user.display_name = "Updated Name".to_string();
    updated_user.bio = "Updated bio".to_string();
    updated_user.following = vec!["friend1".to_string(), "friend2".to_string()];
    updated_user.followers = vec!["follower1".to_string()];

    save_user(&updated_user).await?;
    wait_for_sync().await;

    // Retrieve and verify the update
    let retrieved_user = get_user(&user_id).await?.expect("User should exist");
    assert_eq!(retrieved_user.display_name, "Updated Name");
    assert_eq!(retrieved_user.bio, "Updated bio");
    assert_eq!(retrieved_user.following.len(), 2);
    assert_eq!(retrieved_user.followers.len(), 1);
    assert!(retrieved_user.following.contains(&"friend1".to_string()));
    assert!(retrieved_user.following.contains(&"friend2".to_string()));
    assert!(retrieved_user.followers.contains(&"follower1".to_string()));

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_multiple_users_sync() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

    // Create multiple users
    let mut users = Vec::new();
    for i in 0..5 {
        let user_id = Uuid::new_v4().to_string();
        let user = User {
            id: user_id.clone(),
            display_name: format!("User {}", i),
            bio: format!("Bio for user {}", i),
            public_key: format!("test_public_key_{}", i),
            avatar: None,
            following: Vec::new(),
            followers: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };

        save_user(&user).await?;
        users.push(user);
    }

    wait_for_sync().await;

    // Verify each created user can be retrieved
    for user in &users {
        let retrieved_user = get_user(&user.id).await?.expect("User should exist");
        assert_eq!(retrieved_user.id, user.id);
        assert_eq!(retrieved_user.display_name, user.display_name);
    }

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_posts_by_author_sync() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

    // Create a user
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        display_name: "Post Creator".to_string(),
        bio: "Creates many posts".to_string(),
        public_key: "test_public_key_creator".to_string(),
        avatar: None,
        following: Vec::new(),
        followers: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
    };

    save_user(&user).await?;
    wait_for_sync().await;

    // Create multiple posts by the same author
    let mut posts = Vec::new();
    for i in 0..3 {
        let post_id = Uuid::new_v4().to_string();
        let post = Post {
            id: post_id.clone(),
            author_id: user_id.clone(),
            content: format!("Post content number {}", i),
            attachments: Vec::new(),
            mentions: if i % 2 == 0 {
                vec!["@mention".to_string()]
            } else {
                Vec::new()
            },
            hashtags: if i % 3 == 0 {
                vec!["#test".to_string()]
            } else {
                Vec::new()
            },
            created_at: chrono::Utc::now().timestamp(),
        };

        save_post(&post).await?;
        posts.push(post);
    }

    wait_for_sync().await;

    // Retrieve posts by author
    let author_posts = list_user_posts(&user_id).await?;
    assert_eq!(author_posts.len(), 3, "Expected 3 posts by author");

    // Verify all posts are present and correct
    for post in &posts {
        let found = author_posts.iter().any(|p| p.id == post.id);
        assert!(found, "Post {} not found in author's posts", post.id);
    }

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_document_listing_sync() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

    // Create users and posts
    let mut user_ids = Vec::new();
    for i in 0..2 {
        let user_id = Uuid::new_v4().to_string();
        let user = User {
            id: user_id.clone(),
            display_name: format!("List User {}", i),
            bio: format!("User for listing test {}", i),
            public_key: format!("test_public_key_list_{}", i),
            avatar: None,
            following: Vec::new(),
            followers: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };

        save_user(&user).await?;
        user_ids.push(user_id);
    }

    wait_for_sync().await;

    // Create posts by different authors
    for (i, user_id) in user_ids.iter().enumerate() {
        for j in 0..2 {
            let post_id = Uuid::new_v4().to_string();
            let post = Post {
                id: post_id.clone(),
                author_id: user_id.clone(),
                content: format!("Post {} by user {}", j, i),
                attachments: Vec::new(),
                mentions: Vec::new(),
                hashtags: Vec::new(),
                created_at: chrono::Utc::now().timestamp(),
            };

            save_post(&post).await?;
        }
    }

    wait_for_sync().await;

    // Test listing operations
    let all_posts = list_posts().await?;
    assert!(all_posts.len() >= 4, "Expected at least 4 posts");

    // Verify each user has their posts
    for user_id in &user_ids {
        let user_posts = list_user_posts(user_id).await?;
        assert_eq!(user_posts.len(), 2, "Expected 2 posts per user");
    }

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_concurrent_document_operations() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    let env = TestEnvironment::new().await?;
    env.initialize_global_state().await?; // Initialize global state for repository functions

    // Create multiple users concurrently
    let mut handles = Vec::new();
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            let user_id = Uuid::new_v4().to_string();
            let user = User {
                id: user_id.clone(),
                display_name: format!("Concurrent User {}", i),
                bio: format!("Concurrently created user {}", i),
                public_key: format!("test_public_key_concurrent_{}", i),
                avatar: None,
                following: Vec::new(),
                followers: Vec::new(),
                created_at: chrono::Utc::now().timestamp(),
            };

            save_user(&user).await?;
            Result::<String, StorageError>::Ok(user_id)
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    let mut created_user_ids = Vec::new();
    for handle in handles {
        let user_id = handle
            .await
            .map_err(|e| StorageError::Internal(format!("Task join error: {}", e)))??;
        created_user_ids.push(user_id);
    }

    wait_for_sync().await;

    // Verify all users were created successfully
    for user_id in &created_user_ids {
        let user = get_user(user_id).await?.expect("User should exist");
        assert_eq!(user.id, *user_id);
    }

    env.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_document_persistence_across_restarts() -> Result<(), StorageError> {
    let _ = env_logger::try_init();

    // First session: create data
    let user_id = Uuid::new_v4().to_string();
    let post_id = Uuid::new_v4().to_string();

    {
        let env = TestEnvironment::new().await?;
        env.initialize_global_state().await?; // Initialize global state for repository functions

        let user = User {
            id: user_id.clone(),
            display_name: "Persistent User".to_string(),
            bio: "User for persistence test".to_string(),
            public_key: "test_public_key_persistent".to_string(),
            avatar: None,
            following: Vec::new(),
            followers: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };

        let post = Post {
            id: post_id.clone(),
            author_id: user_id.clone(),
            content: "Persistent post content".to_string(),
            attachments: Vec::new(),
            mentions: Vec::new(),
            hashtags: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
        };

        save_user(&user).await?;
        save_post(&post).await?;
        wait_for_sync().await;

        env.shutdown().await?;
    }

    // Give some time for cleanup
    wait_for_event_propagation().await;

    // Second session: verify data persists
    {
        let env = TestEnvironment::new().await?;

        wait_for_sync().await;

        // Try to retrieve the data
        let retrieved_user = get_user(&user_id).await?.expect("User should exist");
        assert_eq!(retrieved_user.id, user_id);
        assert_eq!(retrieved_user.display_name, "Persistent User");

        let retrieved_post = get_post(&post_id).await?.expect("Post should exist");
        assert_eq!(retrieved_post.id, post_id);
        assert_eq!(retrieved_post.author_id, user_id);
        assert_eq!(retrieved_post.content, "Persistent post content");

        env.shutdown().await?;
    }

    Ok(())
}
