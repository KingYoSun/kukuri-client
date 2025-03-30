use serde::{Deserialize, Serialize};
use std::fs;
use tempfile::tempdir;

use crate::storage::storage_manager::{HasId, Post};
use crate::storage::StorageManager;

// テスト用のトレイトを定義
pub trait TestEntity {
    fn id(&self) -> &str;
    fn author_id(&self) -> &str;
    fn content(&self) -> &str;
    fn created_at(&self) -> i64;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUser {
    pub id: String,
    pub display_name: String,
    pub bio: String,
    pub created_at: i64,
}

// HasIdトレイトの実装
impl HasId for TestUser {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPost {
    pub id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: i64,
}

// HasIdトレイトの実装
impl HasId for TestPost {
    fn id(&self) -> &str {
        &self.id
    }
}

// Postトレイトの実装
impl Post for TestPost {
    fn author_id(&self) -> &str {
        &self.author_id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn created_at(&self) -> i64 {
        self.created_at
    }
}

impl TestEntity for TestPost {
    fn id(&self) -> &str {
        &self.id
    }

    fn author_id(&self) -> &str {
        &self.author_id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn created_at(&self) -> i64 {
        self.created_at
    }
}

// テスト用にストレージパスをオーバーライド
pub fn setup_test_environment() -> tempfile::TempDir {
    let test_dir = tempdir().unwrap();
    // テスト用のパスを設定
    std::env::set_var("APP_TEST_DATA_DIR", test_dir.path().to_str().unwrap());
    test_dir
}

#[test]
fn test_storage_manager_creation() {
    let _test_dir = setup_test_environment();
    let storage_manager = StorageManager::new();
    assert!(storage_manager.is_ok());
}

#[test]
fn test_document_creation() {
    let _test_dir = setup_test_environment();
    let mut storage_manager = StorageManager::new().unwrap();
    let result = storage_manager.get_or_create_document("test-doc");
    assert!(result.is_ok());
}

#[test]
fn test_user_save_and_get() {
    let _test_dir = setup_test_environment();
    let mut storage_manager = StorageManager::new().unwrap();
    storage_manager.get_or_create_document("test-doc").unwrap();

    let user = TestUser {
        id: "user1".to_string(),
        display_name: "Test User".to_string(),
        bio: "This is a test user".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let save_result = storage_manager.save_user(&user);
    assert!(save_result.is_ok());

    let get_result: Result<Option<TestUser>, String> = storage_manager.get_user("user1");
    assert!(get_result.is_ok());

    let retrieved_user = get_result.unwrap();
    assert!(retrieved_user.is_some());

    let retrieved_user = retrieved_user.unwrap();
    assert_eq!(retrieved_user.id, "user1");
    assert_eq!(retrieved_user.display_name, "Test User");
}

#[test]
fn test_post_save_and_get() {
    let _test_dir = setup_test_environment();
    let mut storage_manager = StorageManager::new().unwrap();
    storage_manager.get_or_create_document("test-doc").unwrap();

    let post = TestPost {
        id: "post1".to_string(),
        author_id: "user1".to_string(),
        content: "This is a test post".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let save_result = storage_manager.save_post(&post);
    assert!(save_result.is_ok());

    let get_result: Result<Vec<TestPost>, String> = storage_manager.get_posts(10, 0);
    assert!(get_result.is_ok());

    let posts = get_result.unwrap();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].id, "post1");
    assert_eq!(posts[0].content, "This is a test post");
}

#[test]
fn test_user_posts() {
    let _test_dir = setup_test_environment();
    let mut storage_manager = StorageManager::new().unwrap();
    storage_manager.get_or_create_document("test-doc").unwrap();

    // 複数の投稿を保存
    let post1 = TestPost {
        id: "post1".to_string(),
        author_id: "user1".to_string(),
        content: "Post by user1".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let post2 = TestPost {
        id: "post2".to_string(),
        author_id: "user2".to_string(),
        content: "Post by user2".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let post3 = TestPost {
        id: "post3".to_string(),
        author_id: "user1".to_string(),
        content: "Another post by user1".to_string(),
        created_at: chrono::Utc::now().timestamp() + 1,
    };

    storage_manager.save_post(&post1).unwrap();
    storage_manager.save_post(&post2).unwrap();
    storage_manager.save_post(&post3).unwrap();

    // user1の投稿を取得
    let get_result: Result<Vec<TestPost>, String> = storage_manager.get_user_posts("user1", 10, 0);
    assert!(get_result.is_ok());

    let posts = get_result.unwrap();
    assert_eq!(posts.len(), 2);
    // 作成日時の降順でソートされているか確認
    assert_eq!(posts[0].id, "post3");
    assert_eq!(posts[1].id, "post1");
}

#[test]
fn test_search_posts() {
    let _test_dir = setup_test_environment();
    let mut storage_manager = StorageManager::new().unwrap();
    storage_manager.get_or_create_document("test-doc").unwrap();

    // 複数の投稿を保存
    let post1 = TestPost {
        id: "post1".to_string(),
        author_id: "user1".to_string(),
        content: "This is a test post".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let post2 = TestPost {
        id: "post2".to_string(),
        author_id: "user2".to_string(),
        content: "Another test post".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    let post3 = TestPost {
        id: "post3".to_string(),
        author_id: "user1".to_string(),
        content: "This is a different post".to_string(),
        created_at: chrono::Utc::now().timestamp() + 1,
    };

    storage_manager.save_post(&post1).unwrap();
    storage_manager.save_post(&post2).unwrap();
    storage_manager.save_post(&post3).unwrap();

    // "test"を含む投稿を検索
    let search_result: Result<Vec<TestPost>, String> = storage_manager.search_posts("test", 10);
    assert!(search_result.is_ok());

    let posts = search_result.unwrap();
    assert_eq!(posts.len(), 2);

    // "different"を含む投稿を検索
    let search_result: Result<Vec<TestPost>, String> =
        storage_manager.search_posts("different", 10);
    assert!(search_result.is_ok());

    let posts = search_result.unwrap();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].id, "post3");
}
