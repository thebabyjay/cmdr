mod schema;

pub use schema::*;

use crate::error::{CmdrError, Result};
use std::fs;
use std::path::PathBuf;

/// Get the config directory path (~/.config/cmdr)
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or(CmdrError::ConfigDirNotFound)?
        .join("cmdr");

    if !config_dir.exists() {
        log::info!("[Config] Creating config directory: {:?}", config_dir);
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}

/// Get the projects directory path (~/.config/cmdr/projects)
pub fn get_projects_dir() -> Result<PathBuf> {
    let projects_dir = get_config_dir()?.join("projects");

    if !projects_dir.exists() {
        log::info!("[Config] Creating projects directory: {:?}", projects_dir);
        fs::create_dir_all(&projects_dir)?;
    }

    Ok(projects_dir)
}

/// Get the main config file path
pub fn get_config_file() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.toml"))
}

/// Load settings from config file
pub fn load_settings() -> Result<AppSettings> {
    let config_file = get_config_file()?;
    log::info!("[Config] Loading settings from: {:?}", config_file);

    if !config_file.exists() {
        log::info!("[Config] Config file not found, creating defaults");
        let default_settings = AppSettings::default();
        save_settings(&default_settings)?;
        return Ok(default_settings);
    }

    let content = fs::read_to_string(&config_file)?;
    let settings: AppSettings = toml::from_str(&content)?;
    log::info!("[Config] Settings loaded: terminal={}, theme={}, behavior={}",
        settings.default_terminal, settings.theme, settings.terminal_behavior);
    Ok(settings)
}

/// Save settings to config file
pub fn save_settings(settings: &AppSettings) -> Result<()> {
    let config_file = get_config_file()?;
    log::info!("[Config] Saving settings to: {:?}", config_file);
    let content = toml::to_string_pretty(settings)?;
    fs::write(config_file, content)?;
    log::info!("[Config] Settings saved successfully");
    Ok(())
}

/// Load all projects
pub fn load_projects() -> Result<Vec<Project>> {
    let projects_dir = get_projects_dir()?;
    log::info!("[Config] Loading projects from: {:?}", projects_dir);
    let mut projects = Vec::new();

    for entry in fs::read_dir(&projects_dir)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "toml") {
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    log::error!("[Config] Failed to read project file {:?}: {}", path, e);
                    continue;
                }
            };
            match toml::from_str::<Project>(&content) {
                Ok(project) => {
                    log::debug!("[Config] Loaded project: {} ({})", project.name, project.id);
                    projects.push(project);
                }
                Err(e) => {
                    log::error!("[Config] Failed to parse project file {:?}: {}", path, e);
                    continue;
                }
            }
        }
    }

    // Sort by last opened (most recent first)
    projects.sort_by(|a, b| {
        let a_time = a.last_opened.as_ref().map(|s| s.as_str()).unwrap_or("");
        let b_time = b.last_opened.as_ref().map(|s| s.as_str()).unwrap_or("");
        b_time.cmp(a_time)
    });

    log::info!("[Config] Loaded {} projects", projects.len());
    Ok(projects)
}

/// Save a project
pub fn save_project(project: &Project) -> Result<()> {
    let projects_dir = get_projects_dir()?;
    let file_path = projects_dir.join(format!("{}.toml", project.id));
    log::info!("[Config] Saving project {} to: {:?}", project.name, file_path);
    let content = toml::to_string_pretty(project)?;
    fs::write(file_path, content)?;
    log::info!("[Config] Project saved successfully");
    Ok(())
}

/// Delete a project file
pub fn delete_project_file(id: &str) -> Result<()> {
    let projects_dir = get_projects_dir()?;
    let file_path = projects_dir.join(format!("{}.toml", id));
    log::info!("[Config] Deleting project file: {:?}", file_path);
    if file_path.exists() {
        fs::remove_file(file_path)?;
        log::info!("[Config] Project file deleted successfully");
    } else {
        log::warn!("[Config] Project file not found: {:?}", file_path);
    }
    Ok(())
}
