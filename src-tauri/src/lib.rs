mod commands;
mod config;
mod error;
mod terminal;

use commands::{projects, settings};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::info!("[App] Starting cmdr application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // Project commands
            projects::get_projects,
            projects::add_project,
            projects::update_project,
            projects::delete_project,
            projects::open_project,
            // Settings commands
            settings::get_settings,
            settings::save_settings,
            // Terminal commands
            terminal::launch_workspace,
            terminal::open_terminal,
            terminal::run_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
