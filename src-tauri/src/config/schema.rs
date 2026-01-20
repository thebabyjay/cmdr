use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default = "default_terminal", alias = "default_terminal")]
    pub default_terminal: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default, alias = "default_projects_path")]
    pub default_projects_path: Option<String>,
    #[serde(default = "default_terminal_behavior", alias = "terminal_behavior")]
    pub terminal_behavior: String, // "new_window" or "use_existing"
    #[serde(default, alias = "global_commands")]
    pub global_commands: Vec<Command>,
}

fn default_terminal() -> String {
    "iterm2".to_string()
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_terminal_behavior() -> String {
    "new_window".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_terminal: default_terminal(),
            theme: default_theme(),
            default_projects_path: None,
            terminal_behavior: default_terminal_behavior(),
            global_commands: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub environments: HashMap<String, Environment>,
    #[serde(default)]
    pub workspaces: Vec<Workspace>,
    #[serde(default)]
    pub commands: Vec<Command>,
    #[serde(default, alias = "last_opened")]
    pub last_opened: Option<String>,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    #[serde(default)]
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub layout: WorkspaceLayout,
    #[serde(default)]
    pub panes: Vec<Pane>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceLayout {
    pub rows: u32,
    pub columns: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pane {
    pub position: (u32, u32),
    pub directory: String,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default, alias = "environment_variables")]
    pub environment_variables: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

/// Input type for adding a new project (without id and created_at)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub environments: HashMap<String, Environment>,
    #[serde(default)]
    pub workspaces: Vec<Workspace>,
    #[serde(default)]
    pub commands: Vec<Command>,
}

impl NewProject {
    pub fn into_project(self) -> Project {
        Project {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.name,
            path: self.path,
            description: self.description,
            tags: self.tags,
            environments: self.environments,
            workspaces: self.workspaces,
            commands: self.commands,
            last_opened: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
