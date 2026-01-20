export interface Project {
  id: string;
  name: string;
  path: string;
  description?: string;
  tags: string[];
  environments: Record<string, Environment>;
  workspaces: Workspace[];
  commands: Command[];
  lastOpened?: string;
  createdAt: string;
}

export interface Environment {
  name: string;
  variables: Record<string, string>;
}

export interface Workspace {
  id: string;
  name: string;
  layout: WorkspaceLayout;
  panes: Pane[];
}

export interface WorkspaceLayout {
  rows: number;
  columns: number[];
}

export interface Pane {
  position: [number, number];
  directory: string;
  command?: string;
  environmentVariables?: Record<string, string>;
}

export interface Command {
  id: string;
  name: string;
  command: string;
  description?: string;
  icon?: string;
}

export interface AppSettings {
  defaultTerminal: "iterm2" | "terminal" | "windows_terminal" | "gnome";
  theme: "dark" | "light";
  defaultProjectsPath?: string;
  terminalBehavior: "new_window" | "use_existing";
  globalCommands: Command[];
}
