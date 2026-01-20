use crate::config::{self, NewProject, Project, Environment, Workspace, Command};
use crate::error::{CmdrError, Result};
use std::collections::HashMap;

#[tauri::command]
pub fn get_projects() -> Result<Vec<Project>> {
    log::info!("[Projects] Getting all projects");
    let projects = config::load_projects()?;
    log::info!("[Projects] Returning {} projects", projects.len());
    Ok(projects)
}

#[tauri::command]
pub fn add_project(project: NewProject) -> Result<Project> {
    log::info!("[Projects] Adding new project: {}", project.name);
    let project = project.into_project();
    config::save_project(&project)?;
    log::info!("[Projects] Project added with ID: {}", project.id);
    Ok(project)
}

#[tauri::command]
pub fn update_project(id: String, updates: HashMap<String, serde_json::Value>) -> Result<Project> {
    log::info!("[Projects] Updating project: {}", id);
    log::debug!("[Projects] Update fields: {:?}", updates.keys().collect::<Vec<_>>());

    let projects = config::load_projects()?;
    let mut project = projects
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| {
            log::error!("[Projects] Project not found: {}", id);
            CmdrError::ProjectNotFound(id.clone())
        })?;

    // Apply updates
    if let Some(name) = updates.get("name").and_then(|v| v.as_str()) {
        log::debug!("[Projects] Updating name: {} -> {}", project.name, name);
        project.name = name.to_string();
    }
    if let Some(path) = updates.get("path").and_then(|v| v.as_str()) {
        log::debug!("[Projects] Updating path: {} -> {}", project.path, path);
        project.path = path.to_string();
    }
    if let Some(description) = updates.get("description") {
        log::debug!("[Projects] Updating description");
        project.description = description.as_str().map(|s| s.to_string());
    }
    if let Some(tags) = updates.get("tags").and_then(|v| v.as_array()) {
        log::debug!("[Projects] Updating tags");
        project.tags = tags
            .iter()
            .filter_map(|t| t.as_str().map(|s| s.to_string()))
            .collect();
    }
    if let Some(last_opened) = updates.get("lastOpened").and_then(|v| v.as_str()) {
        log::debug!("[Projects] Updating lastOpened");
        project.last_opened = Some(last_opened.to_string());
    }
    if let Some(environments) = updates.get("environments") {
        log::debug!("[Projects] Updating environments");
        match serde_json::from_value::<HashMap<String, Environment>>(environments.clone()) {
            Ok(envs) => project.environments = envs,
            Err(e) => log::error!("[Projects] Failed to parse environments: {} - value: {:?}", e, environments),
        }
    }
    if let Some(workspaces) = updates.get("workspaces") {
        log::debug!("[Projects] Updating workspaces");
        match serde_json::from_value::<Vec<Workspace>>(workspaces.clone()) {
            Ok(ws) => project.workspaces = ws,
            Err(e) => log::error!("[Projects] Failed to parse workspaces: {} - value: {:?}", e, workspaces),
        }
    }
    if let Some(commands) = updates.get("commands") {
        log::debug!("[Projects] Updating commands");
        match serde_json::from_value::<Vec<Command>>(commands.clone()) {
            Ok(cmds) => project.commands = cmds,
            Err(e) => log::error!("[Projects] Failed to parse commands: {} - value: {:?}", e, commands),
        }
    }

    config::save_project(&project)?;
    log::info!("[Projects] Project updated successfully: {}", project.name);
    Ok(project)
}

#[tauri::command]
pub fn delete_project(id: String) -> Result<()> {
    log::info!("[Projects] Deleting project: {}", id);
    config::delete_project_file(&id)?;
    log::info!("[Projects] Project deleted successfully");
    Ok(())
}

#[tauri::command]
pub fn open_project(id: String) -> Result<()> {
    log::info!("[Projects] Opening project: {}", id);

    let projects = config::load_projects()?;
    let mut project = projects
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| {
            log::error!("[Projects] Project not found: {}", id);
            CmdrError::ProjectNotFound(id.clone())
        })?;

    log::info!("[Projects] Found project: {} at path: {}", project.name, project.path);

    // Update last opened timestamp
    project.last_opened = Some(chrono::Utc::now().to_rfc3339());
    config::save_project(&project)?;

    // Open in default file manager/IDE
    // For now, just open the folder
    #[cfg(target_os = "macos")]
    {
        log::info!("[Projects] Opening folder in Finder: {}", project.path);
        match std::process::Command::new("open")
            .arg(&project.path)
            .spawn() {
            Ok(_) => log::info!("[Projects] Folder opened successfully"),
            Err(e) => log::error!("[Projects] Failed to open folder: {}", e),
        }
    }

    #[cfg(target_os = "windows")]
    {
        log::info!("[Projects] Opening folder in Explorer: {}", project.path);
        match std::process::Command::new("explorer")
            .arg(&project.path)
            .spawn() {
            Ok(_) => log::info!("[Projects] Folder opened successfully"),
            Err(e) => log::error!("[Projects] Failed to open folder: {}", e),
        }
    }

    #[cfg(target_os = "linux")]
    {
        log::info!("[Projects] Opening folder with xdg-open: {}", project.path);
        match std::process::Command::new("xdg-open")
            .arg(&project.path)
            .spawn() {
            Ok(_) => log::info!("[Projects] Folder opened successfully"),
            Err(e) => log::error!("[Projects] Failed to open folder: {}", e),
        }
    }

    Ok(())
}
