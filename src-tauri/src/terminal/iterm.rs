use crate::config::{Project, Workspace};
use crate::error::{CmdrError, Result};
use std::process::Command;

/// Generate AppleScript to launch a workspace in iTerm2
pub fn launch_workspace(project: &Project, workspace: &Workspace) -> Result<()> {
    log::info!(
        "[iTerm] Generating AppleScript for workspace: {}",
        workspace.name
    );
    log::debug!("[iTerm] Project path: {}", project.path);
    log::debug!("[iTerm] Workspace layout: {} rows", workspace.layout.rows);

    let script = generate_applescript(project, workspace);

    log::debug!("[iTerm] Executing AppleScript ({} chars)", script.len());

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| {
            log::error!("[iTerm] Failed to execute AppleScript: {}", e);
            CmdrError::Terminal(format!("Failed to execute AppleScript: {}", e))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("[iTerm] AppleScript error: {}", stderr);
        return Err(CmdrError::Terminal(format!(
            "AppleScript error: {}",
            stderr
        )));
    }

    log::info!("[iTerm] Workspace launched successfully in iTerm2");
    Ok(())
}

fn generate_applescript(project: &Project, workspace: &Workspace) -> String {
    let layout = &workspace.layout;
    let panes = &workspace.panes;

    let total_panes: u32 = layout.columns.iter().sum();
    let num_rows = layout.columns.len();

    log::debug!(
        "[iTerm] Generating script for {} panes across {} rows",
        total_panes,
        num_rows
    );
    log::debug!("[iTerm] Layout columns: {:?}", layout.columns);

    let mut script = String::new();

    // Start iTerm2 and create a new window
    script.push_str(
        r#"
tell application "iTerm"
    activate

    -- Create a new window
    create window with default profile

    tell current window
        set theTab to current tab
"#,
    );

    // Strategy: Use AppleScript variables to track session references
    // This is more reliable than assuming session numbers are sequential
    //
    // 1. Store reference to first session as sess_0_0
    // 2. Create all rows by splitting first session horizontally, storing refs
    // 3. For each row, split vertically to create columns, storing refs
    // 4. Configure each session using the stored variable references

    // Helper to generate variable name for a session at (row, col)
    let sess_var = |row: usize, col: usize| format!("sess_{}_{}", row, col);

    // Store reference to the first session (row 0, col 0)
    script.push_str(&format!(
        r#"
        -- Store reference to first session
        set {} to current session of theTab
"#,
        sess_var(0, 0)
    ));

    // Step 1: Create all rows by splitting horizontally
    // Each new row is created by splitting the previous row's first pane
    // This ensures rows are added at the bottom in correct order
    for row_idx in 1..num_rows {
        let prev_row = row_idx - 1;
        script.push_str(&format!(
            r#"
        -- Create row {} by splitting row {} horizontally
        tell {}
            set {} to (split horizontally with default profile)
        end tell
"#,
            row_idx,
            prev_row,
            sess_var(prev_row, 0),
            sess_var(row_idx, 0)
        ));
    }

    // Step 2: Create columns in each row by splitting vertically
    // We split the rightmost pane in each row to add columns to the right
    for row_idx in 0..num_rows {
        let cols_in_row = layout.columns[row_idx] as usize;

        for col_idx in 1..cols_in_row {
            let prev_col = col_idx - 1;
            script.push_str(&format!(
                r#"
        -- Create row {} col {} by splitting vertically
        tell {}
            set {} to (split vertically with default profile)
        end tell
"#,
                row_idx,
                col_idx,
                sess_var(row_idx, prev_col),
                sess_var(row_idx, col_idx)
            ));
        }
    }

    // Small delay to ensure all sessions are ready
    script.push_str(
        r#"
        -- Small delay to ensure all sessions are ready
        delay 0.3
"#,
    );

    // Configure each session with its directory and command
    for row_idx in 0..num_rows {
        let cols_in_row = layout.columns[row_idx] as usize;

        for col_idx in 0..cols_in_row {
            // Find pane with this position
            let pane = panes
                .iter()
                .find(|p| p.position.0 as usize == row_idx && p.position.1 as usize == col_idx);

            let dir = if let Some(p) = pane {
                resolve_directory(&project.path, &p.directory)
            } else {
                project.path.clone()
            };

            script.push_str(&format!(
                r#"
        -- Configure session at row {}, col {}
        tell {}
            write text "cd '{}'"
"#,
                row_idx,
                col_idx,
                sess_var(row_idx, col_idx),
                escape_applescript_string(&dir)
            ));

            if let Some(p) = pane {
                if let Some(cmd) = &p.command {
                    script.push_str(&format!(
                        r#"            write text "{}"
"#,
                        escape_applescript_string(cmd)
                    ));
                }
            }

            script.push_str(
                r#"        end tell
"#,
            );
        }
    }

    script.push_str(
        r#"    end tell
end tell
"#,
    );

    log::debug!("[iTerm] Generated AppleScript:\n{}", script);

    script
}

fn resolve_directory(project_path: &str, pane_dir: &str) -> String {
    if pane_dir == "." || pane_dir.is_empty() {
        project_path.to_string()
    } else if pane_dir.starts_with('/') {
        pane_dir.to_string()
    } else if pane_dir.starts_with("./") {
        format!("{}/{}", project_path, &pane_dir[2..])
    } else {
        format!("{}/{}", project_path, pane_dir)
    }
}

fn escape_applescript_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_directory() {
        assert_eq!(
            resolve_directory("/home/user/project", "."),
            "/home/user/project"
        );
        assert_eq!(
            resolve_directory("/home/user/project", "./src"),
            "/home/user/project/src"
        );
        assert_eq!(
            resolve_directory("/home/user/project", "backend"),
            "/home/user/project/backend"
        );
        assert_eq!(
            resolve_directory("/home/user/project", "/absolute/path"),
            "/absolute/path"
        );
    }

    #[test]
    fn test_escape_applescript() {
        assert_eq!(escape_applescript_string("hello"), "hello");
        assert_eq!(escape_applescript_string("he\"llo"), "he\\\"llo");
        assert_eq!(escape_applescript_string("he\\llo"), "he\\\\llo");
    }
}
