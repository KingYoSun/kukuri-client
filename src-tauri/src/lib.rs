mod commands;
mod models;
pub mod network;
pub mod storage;
// Tokio Runtime is usually managed by tauri::async_runtime

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone(); // Clone the handle
                                               // Spawn an async task to initialize the Iroh node
                                               // This prevents blocking the main thread during setup
            tauri::async_runtime::spawn(async move {
                println!("Initializing Iroh node...");
                // Use the initialize function from the storage state module
                if let Err(err) = crate::storage::state::initialize_iroh(&handle).await {
                    eprintln!("Failed to initialize Iroh node: {:?}", err);
                    // Consider more robust error handling, e.g., notifying the user or exiting
                } else {
                    println!("Iroh node initialized successfully.");
                    // You can now proceed with other setup tasks that depend on Iroh
                }
            });
            Ok(()) // Indicate successful setup hook execution
        })
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
