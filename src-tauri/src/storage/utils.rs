use std::fs;
use std::path::PathBuf;

/// ストレージパスの取得
///
/// iroh-docsのストレージパスを取得します。
pub fn get_storage_path() -> Result<PathBuf, String> {
    // テスト用の簡易実装
    let app_dir = std::env::temp_dir();
    let storage_dir = app_dir.join("iroh-docs-mock");

    fs::create_dir_all(&storage_dir)
        .map_err(|e| format!("Failed to create storage directory: {}", e))?;

    Ok(storage_dir)
}
