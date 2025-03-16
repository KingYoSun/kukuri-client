use automerge::{Automerge, transaction::Transactable};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::api::path;

use crate::models::user::User;
use crate::models::post::Post;

// グローバルなAutomergeドキュメント
static DOCUMENT: Lazy<Mutex<Automerge>> = Lazy::new(|| {
    Mutex::new(load_or_create_document().unwrap_or_else(|_| {
        let mut doc = Automerge::new();
        initialize_document(&mut doc).unwrap();
        doc
    }))
});

// データファイルのパス
fn get_data_file_path() -> Result<PathBuf, String> {
    let app_dir = path::app_data_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let data_dir = app_dir.join("data");
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;
    
    Ok(data_dir.join("social_data.automerge"))
}

// ドキュメントの読み込みまたは新規作成
fn load_or_create_document() -> Result<Automerge, String> {
    let file_path = get_data_file_path()?;
    
    if file_path.exists() {
        let mut file = File::open(&file_path)
            .map_err(|e| format!("Failed to open data file: {}", e))?;
        
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|e| format!("Failed to read data file: {}", e))?;
        
        Automerge::load(&bytes)
            .map_err(|e| format!("Failed to parse Automerge document: {}", e))
    } else {
        let mut doc = Automerge::new();
        initialize_document(&mut doc)?;
        save_document(&doc)?;
        Ok(doc)
    }
}

// ドキュメントの初期構造を設定
fn initialize_document(doc: &mut Automerge) -> Result<(), String> {
    let mut tx = doc.transaction();
    
    // ルートオブジェクトにコレクションを作成
    tx.put_object(automerge::ROOT, "users", automerge::ObjType::Map)
        .map_err(|e| format!("Failed to create users collection: {}", e))?;
    
    tx.put_object(automerge::ROOT, "posts", automerge::ObjType::Map)
        .map_err(|e| format!("Failed to create posts collection: {}", e))?;
    
    tx.put_object(automerge::ROOT, "settings", automerge::ObjType::Map)
        .map_err(|e| format!("Failed to create settings collection: {}", e))?;
    
    tx.commit();
    Ok(())
}

// ドキュメントの保存
fn save_document(doc: &Automerge) -> Result<(), String> {
    let file_path = get_data_file_path()?;
    
    let bytes = doc.save();
    let mut file = File::create(&file_path)
        .map_err(|e| format!("Failed to create data file: {}", e))?;
    
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write data file: {}", e))?;
    
    Ok(())
}

// ユーザー保存
pub fn save_user(user: &User) -> Result<(), String> {
    let mut doc = DOCUMENT.lock().unwrap();
    let mut tx = doc.transaction();
    
    // ユーザーJSONにシリアライズ
    let user_json = serde_json::to_string(user)
        .map_err(|e| format!("Failed to serialize user: {}", e))?;
    
    // users/{user_id}にユーザー情報を保存
    let users_obj = tx.get(automerge::ROOT, "users")
        .map_err(|e| format!("Failed to get users collection: {}", e))?
        .expect("users collection should exist");
    
    tx.put(users_obj, &user.id, user_json)
        .map_err(|e| format!("Failed to save user: {}", e))?;
    
    tx.commit();
    
    // ドキュメントをディスクに保存
    save_document(&doc)
}

// ユーザー取得
pub fn get_user(user_id: &str) -> Result<Option<User>, String> {
    let doc = DOCUMENT.lock().unwrap();
    
    let users_obj = doc.get(automerge::ROOT, "users")
        .map_err(|e| format!("Failed to get users collection: {}", e))?
        .expect("users collection should exist");
    
    match doc.get(users_obj, user_id) {
        Ok(Some(automerge::Value::Text(user_json))) => {
            serde_json::from_str(&user_json)
                .map_err(|e| format!("Failed to deserialize user: {}", e))
                .map(Some)
        }
        Ok(None) => Ok(None),
        Ok(Some(_)) => Err("Invalid user data format".to_string()),
        Err(e) => Err(format!("Failed to get user: {}", e)),
    }
}

// 投稿保存
pub fn save_post(post: &Post) -> Result<(), String> {
    let mut doc = DOCUMENT.lock().unwrap();
    let mut tx = doc.transaction();
    
    // 投稿JSONにシリアライズ
    let post_json = serde_json::to_string(post)
        .map_err(|e| format!("Failed to serialize post: {}", e))?;
    
    // posts/{post_id}に投稿情報を保存
    let posts_obj = tx.get(automerge::ROOT, "posts")
        .map_err(|e| format!("Failed to get posts collection: {}", e))?
        .expect("posts collection should exist");
    
    tx.put(posts_obj, &post.id, post_json)
        .map_err(|e| format!("Failed to save post: {}", e))?;
    
    tx.commit();
    
    // ドキュメントをディスクに保存
    save_document(&doc)
}

// 投稿取得（ページネーション付き）
pub fn get_posts(limit: usize, offset: usize) -> Result<Vec<Post>, String> {
    let doc = DOCUMENT.lock().unwrap();
    
    let posts_obj = doc.get(automerge::ROOT, "posts")
        .map_err(|e| format!("Failed to get posts collection: {}", e))?
        .expect("posts collection should exist");
    
    let mut posts = Vec::new();
    
    // すべての投稿キーを取得
    let keys = doc.keys(posts_obj)
        .map_err(|e| format!("Failed to get post keys: {}", e))?;
    
    // タイムスタンプでソートするためのデータを収集
    let mut post_data = Vec::new();
    for key in keys {
        if let Ok(Some(automerge::Value::Text(post_json))) = doc.get(posts_obj, &key) {
            if let Ok(post) = serde_json::from_str::<Post>(&post_json) {
                post_data.push(post);
            }
        }
    }
    
    // 作成日時の降順でソート
    post_data.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    // ページネーション適用
    let end = std::cmp::min(offset + limit, post_data.len());
    if offset < end {
        posts = post_data[offset..end].to_vec();
    }
    
    Ok(posts)
}

// 特定ユーザーの投稿取得（ページネーション付き）
pub fn get_user_posts(user_id: &str, limit: usize, offset: usize) -> Result<Vec<Post>, String> {
    let doc = DOCUMENT.lock().unwrap();
    
    let posts_obj = doc.get(automerge::ROOT, "posts")
        .map_err(|e| format!("Failed to get posts collection: {}", e))?
        .expect("posts collection should exist");
    
    let mut posts = Vec::new();
    
    // すべての投稿キーを取得
    let keys = doc.keys(posts_obj)
        .map_err(|e| format!("Failed to get post keys: {}", e))?;
    
    // ユーザーIDに一致する投稿を収集
    let mut user_posts = Vec::new();
    for key in keys {
        if let Ok(Some(automerge::Value::Text(post_json))) = doc.get(posts_obj, &key) {
            if let Ok(post) = serde_json::from_str::<Post>(&post_json) {
                if post.author_id == user_id {
                    user_posts.push(post);
                }
            }
        }
    }
    
    // 作成日時の降順でソート
    user_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    // ページネーション適用
    let end = std::cmp::min(offset + limit, user_posts.len());
    if offset < end {
        posts = user_posts[offset..end].to_vec();
    }
    
    Ok(posts)
}

// 投稿検索（ローカルのみ）
pub fn search_posts(query: &str, limit: usize) -> Result<Vec<Post>, String> {
    let doc = DOCUMENT.lock().unwrap();
    
    let posts_obj = doc.get(automerge::ROOT, "posts")
        .map_err(|e| format!("Failed to get posts collection: {}", e))?
        .expect("posts collection should exist");
    
    let mut matching_posts = Vec::new();
    let query_lower = query.to_lowercase();
    
    // すべての投稿キーを取得
    let keys = doc.keys(posts_obj)
        .map_err(|e| format!("Failed to get post keys: {}", e))?;
    
    // クエリに一致する投稿を収集
    for key in keys {
        if let Ok(Some(automerge::Value::Text(post_json))) = doc.get(posts_obj, &key) {
            if let Ok(post) = serde_json::from_str::<Post>(&post_json) {
                // コンテンツ内にクエリが含まれているか確認
                if post.content.to_lowercase().contains(&query_lower) {
                    matching_posts.push(post);
                    if matching_posts.len() >= limit {
                        break;
                    }
                }
            }
        }
    }
    
    // 作成日時の降順でソート
    matching_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(matching_posts)
}

// Automergeドキュメントを他のノードとマージ
pub fn merge_remote_changes(changes: &[u8]) -> Result<(), String> {
    let mut doc = DOCUMENT.lock().unwrap();
    
    doc.load_changes(changes)
        .map_err(|e| format!("Failed to merge remote changes: {}", e))?;
    
    save_document(&doc)
}

// 最後のマージ以降の変更を取得
pub fn get_changes_since(heads: Vec<String>) -> Result<Vec<u8>, String> {
    let doc = DOCUMENT.lock().unwrap();
    
    // 実際の実装では、Automergeのバージョンに合わせて適切に変換する必要がある
    // ここではモック実装として空のバイト列を返す
    Ok(Vec::new())
}

// ドキュメントの現在のヘッドハッシュを取得
pub fn get_heads() -> Result<Vec<String>, String> {
    let doc = DOCUMENT.lock().unwrap();
    
    // 実際の実装では、Automergeのバージョンに合わせて適切に変換する必要がある
    // ここではモック実装として空の配列を返す
    Ok(Vec::new())
}