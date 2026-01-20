mod iterm;

use crate::error::{CmdrError, Result};

#[tauri::command]
pub fn launch_workspace(project_id: String, workspace_id: String) -> Result<()> {
    log::info!("[Terminal] Launching workspace {} for project {}", workspace_id, project_id);

    let projects = crate::config::load_projects()?;
    let project = projects
        .into_iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| CmdrError::ProjectNotFound(project_id.clone()))?;

    let workspace = project
        .workspaces
        .iter()
        .find(|w| w.id == workspace_id)
        .ok_or_else(|| CmdrError::ProjectNotFound(format!("workspace:{}", workspace_id)))?;

    log::info!("[Terminal] Found workspace: {} with {} panes", workspace.name, workspace.panes.len());

    // Launch using iTerm2 (macOS only for MVP)
    #[cfg(target_os = "macos")]
    {
        iterm::launch_workspace(&project, workspace)?;
        log::info!("[Terminal] Workspace launched successfully");
    }

    #[cfg(not(target_os = "macos"))]
    {
        return Err(CmdrError::Terminal(
            "Terminal automation not yet supported on this platform".to_string(),
        ));
    }

    Ok(())
}

#[tauri::command]
pub fn open_terminal() -> Result<()> {
    log::info!("[Terminal] Opening new terminal window");

    #[cfg(target_os = "macos")]
    {
        // Load settings to check which terminal to use
        let settings = crate::config::load_settings()?;
        log::info!("[Terminal] Using terminal: {}", settings.default_terminal);

        let script = match settings.default_terminal.as_str() {
            "terminal" => {
                r#"
                tell application "Terminal"
                    activate
                    do script ""
                end tell
                "#
            }
            _ => {
                // Default to iTerm2
                r#"
                tell application "iTerm"
                    activate
                    create window with default profile
                end tell
                "#
            }
        };

        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| CmdrError::Terminal(format!("Failed to execute AppleScript: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("[Terminal] AppleScript error: {}", stderr);
            return Err(CmdrError::Terminal(format!("AppleScript error: {}", stderr)));
        }

        log::info!("[Terminal] Terminal opened successfully");
    }

    #[cfg(not(target_os = "macos"))]
    {
        return Err(CmdrError::Terminal(
            "Terminal automation not yet supported on this platform".to_string(),
        ));
    }

    Ok(())
}

#[tauri::command]
pub fn run_command(project_id: String, command: String) -> Result<()> {
    log::info!("[Terminal] Running command '{}' for project {}", command, project_id);

    let projects = crate::config::load_projects()?;
    let project = projects
        .into_iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| CmdrError::ProjectNotFound(project_id.clone()))?;

    log::info!("[Terminal] Found project at path: {}", project.path);

    #[cfg(target_os = "macos")]
    {
        // Load settings to check which terminal to use
        let settings = crate::config::load_settings()?;
        log::info!("[Terminal] Using terminal: {}, behavior: {}", settings.default_terminal, settings.terminal_behavior);

        let escaped_path = project.path.replace("'", "'\\''");
        let escaped_command = command.replace("'", "'\\''");

        let use_existing = settings.terminal_behavior == "use_existing";

        let script = match settings.default_terminal.as_str() {
            "terminal" => {
                if use_existing {
                    format!(
                        r#"
                        tell application "Terminal"
                            activate
                            if (count of windows) > 0 then
                                do script "cd '{}' && {}" in front window
                            else
                                do script "cd '{}' && {}"
                            end if
                        end tell
                        "#,
                        escaped_path, escaped_command, escaped_path, escaped_command
                    )
                } else {
                    format!(
                        r#"
                        tell application "Terminal"
                            activate
                            do script "cd '{}' && {}"
                        end tell
                        "#,
                        escaped_path, escaped_command
                    )
                }
            }
            _ => {
                // Default to iTerm2
                if use_existing {
                    format!(
                        r#"
                        tell application "iTerm"
                            activate
                            if (count of windows) > 0 then
                                tell current window
                                    create tab with default profile
                                    tell current session
                                        write text "cd '{}'"
                                        write text "{}"
                                    end tell
                                end tell
                            else
                                create window with default profile
                                tell current window
                                    tell current session
                                        write text "cd '{}'"
                                        write text "{}"
                                    end tell
                                end tell
                            end if
                        end tell
                        "#,
                        escaped_path, escaped_command, escaped_path, escaped_command
                    )
                } else {
                    format!(
                        r#"
                        tell application "iTerm"
                            activate
                            create window with default profile
                            tell current window
                                tell current session
                                    write text "cd '{}'"
                                    write text "{}"
                                end tell
                            end tell
                        end tell
                        "#,
                        escaped_path, escaped_command
                    )
                }
            }
        };

        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| CmdrError::Terminal(format!("Failed to execute AppleScript: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("[Terminal] AppleScript error: {}", stderr);
            return Err(CmdrError::Terminal(format!("AppleScript error: {}", stderr)));
        }

        log::info!("[Terminal] Command executed successfully");
    }

    #[cfg(not(target_os = "macos"))]
    {
        return Err(CmdrError::Terminal(
            "Terminal automation not yet supported on this platform".to_string(),
        ));
    }

    Ok(())
}
