mod commands;
mod models;
pub mod network;

use tokio::runtime::Runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Tokioランタイムの作成
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    // ネットワークの初期化
    runtime.block_on(async {
        // ネットワークの初期化
        if let Err(e) = network::iroh::initialize_network().await {
            eprintln!("Failed to initialize network: {}", e);
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // 認証コマンド
            commands::auth::create_user,
            commands::auth::sign_in,
            commands::auth::list_users,
            // 投稿コマンド
            commands::post::create_post,
            commands::post::get_posts,
            commands::post::get_user_posts,
            commands::post::search_posts,
            // プロフィールコマンド
            commands::profile::get_profile,
            commands::profile::update_profile,
            commands::profile::follow_user,
            commands::profile::unfollow_user,
            // 設定コマンド
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
