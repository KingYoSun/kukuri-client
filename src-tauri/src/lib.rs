mod commands;
mod models;
mod network;
mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ネットワークの初期化
    let _ = network::iroh::initialize_network();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
