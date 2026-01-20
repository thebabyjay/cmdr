use crate::config::{Project, Workspace};
use crate::error::{CmdrError, Result};
use std::process::Command;

/// Generate AppleScript to launch a workspace in iTerm2
pub fn launch_workspace(project: &Project, workspace: &Workspace) -> Result<()> {
    log::info!("[iTerm] Generating AppleScript for workspace: {}", workspace.name);
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

    // Calculate total panes needed based on layout
    // Layout: rows x columns (e.g., 2 rows with [2, 3] columns = 2 on top, 3 on bottom = 5 panes)
    let total_panes: u32 = layout.columns.iter().sum();

    log::debug!("[iTerm] Generating script for {} panes across {} rows", total_panes, layout.rows);
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

    // Strategy: Create all sessions first using splits, then configure each one.
    // We use session indices to reference sessions after creation.
    //
    // For a layout like [2, 3] (2 on top, 3 on bottom):
    // Session 1: (0,0) - created with window
    // Session 2: (0,1) - split vertically from session 1
    // Session 3: (1,0) - split horizontally from session 1
    // Session 4: (1,1) - split vertically from session 3
    // Session 5: (1,2) - split vertically from session 4
    //
    // The key is: for each new row, we split horizontally from the leftmost session
    // of the previous row. For columns within a row, we split vertically from the
    // previous session in that row.

    // Track which session index corresponds to which (row, col)
    // sessions[row][col] = session_index (1-based for AppleScript)
    let mut sessions: Vec<Vec<u32>> = Vec::new();
    let mut current_session: u32 = 1;

    // First row: session 1 is created with the window
    let first_row_cols = layout.columns.first().copied().unwrap_or(1);
    let mut first_row_sessions: Vec<u32> = vec![1];

    // Create additional columns in first row via vertical splits
    for col in 1..first_row_cols {
        current_session += 1;
        first_row_sessions.push(current_session);
        script.push_str(&format!(
            r#"
        -- Create session {} (row 0, col {})
        tell session {} of theTab
            split vertically with default profile
        end tell
"#,
            current_session, col, current_session - 1
        ));
    }
    sessions.push(first_row_sessions);

    // Create additional rows
    for (row_idx, &cols_in_row) in layout.columns.iter().enumerate().skip(1) {
        let mut row_sessions: Vec<u32> = Vec::new();

        // First column of new row: split horizontally from leftmost session of previous row
        let prev_row_first_session = sessions[row_idx - 1][0];
        current_session += 1;
        row_sessions.push(current_session);
        script.push_str(&format!(
            r#"
        -- Create session {} (row {}, col 0)
        tell session {} of theTab
            split horizontally with default profile
        end tell
"#,
            current_session, row_idx, prev_row_first_session
        ));

        // Additional columns in this row: split vertically from previous column's session
        for col in 1..cols_in_row {
            let prev_col_session = row_sessions[col as usize - 1];
            current_session += 1;
            row_sessions.push(current_session);
            script.push_str(&format!(
                r#"
        -- Create session {} (row {}, col {})
        tell session {} of theTab
            split vertically with default profile
        end tell
"#,
                current_session, row_idx, col, prev_col_session
            ));
        }

        sessions.push(row_sessions);
    }

    // Now configure each session with its directory and command
    script.push_str(
        r#"
        -- Small delay to ensure all sessions are ready
        delay 0.1
"#,
    );

    // Get pane for each (row, col) position and configure the corresponding session
    for (row_idx, row_sessions) in sessions.iter().enumerate() {
        for (col_idx, &session_num) in row_sessions.iter().enumerate() {
            // Find pane with this position
            let pane = panes.iter().find(|p| {
                p.position[0] as usize == row_idx && p.position[1] as usize == col_idx
            });

            let dir = if let Some(p) = pane {
                resolve_directory(&project.path, &p.directory)
            } else {
                project.path.clone()
            };

            script.push_str(&format!(
                r#"
        -- Configure session {} (row {}, col {})
        tell session {} of theTab
            write text "cd '{}'"
"#,
                session_num, row_idx, col_idx, session_num,
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
