use crate::config::{self, AppSettings};
use crate::error::Result;

#[tauri::command]
pub fn get_settings() -> Result<AppSettings> {
    config::load_settings()
}

#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<()> {
    config::save_settings(&settings)
}
