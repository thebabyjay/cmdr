<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useProjectsStore } from "@/stores/projects";
import { useSettingsStore } from "@/stores/settings";
import type { Workspace, Command, Environment, Pane } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import ConfirmDialog from "@/components/ConfirmDialog.vue";
import AddEnvironmentModal from "@/components/AddEnvironmentModal.vue";
import AddWorkspaceModal from "@/components/AddWorkspaceModal.vue";
import AddCommandModal from "@/components/AddCommandModal.vue";

const route = useRoute();
const router = useRouter();
const projectsStore = useProjectsStore();
const settingsStore = useSettingsStore();

const globalCommands = computed(() => settingsStore.settings.globalCommands || []);

const showDeleteConfirm = ref(false);
const showAddEnvironment = ref(false);
const showAddWorkspace = ref(false);
const showAddCommand = ref(false);

// Editable project fields
const editName = ref("");
const editPath = ref("");
const editDescription = ref("");
const editTags = ref("");
const isEditing = ref(false);
const saveError = ref<string | null>(null);

// Edit modals for environments, workspaces, commands
const showEditEnvironment = ref(false);
const editingEnvironment = ref<{ name: string; env: Environment } | null>(null);
const showEditWorkspace = ref(false);
const editingWorkspace = ref<Workspace | null>(null);
const showEditCommand = ref(false);
const editingCommand = ref<Command | null>(null);

// Track command type overrides during workspace editing (key: "row,col", value: type)
const paneCommandTypeOverrides = ref<Map<string, 'none' | 'preset' | 'custom'>>(new Map());

// Delete confirmations
const showDeleteEnvironmentConfirm = ref(false);
const deletingEnvironmentName = ref<string | null>(null);
const showDeleteWorkspaceConfirm = ref(false);
const deletingWorkspace = ref<Workspace | null>(null);
const showDeleteCommandConfirm = ref(false);
const deletingCommand = ref<Command | null>(null);

const project = computed(() => {
  return projectsStore.projectById(route.params.id as string);
});

// Initialize edit fields when project loads
watch(project, (newProject) => {
  if (newProject && !isEditing.value) {
    editName.value = newProject.name;
    editPath.value = newProject.path;
    editDescription.value = newProject.description || "";
    editTags.value = newProject.tags.join(", ");
  }
}, { immediate: true });

onMounted(() => {
  if (!projectsStore.projects.length) {
    projectsStore.loadProjects();
  }
  settingsStore.loadSettings();
});

const openProject = async () => {
  if (project.value) {
    try {
      await projectsStore.openProject(project.value.id);
    } catch (e) {
      console.error("[ProjectDetail] Failed to open project:", e);
    }
  }
};

const deleteProject = async () => {
  if (project.value) {
    await projectsStore.deleteProject(project.value.id);
    router.push("/projects");
  }
};

const browsePath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: editPath.value || undefined,
    });
    if (selected) {
      editPath.value = selected as string;
    }
  } catch (e) {
    console.error("[ProjectDetail] Failed to open folder picker:", e);
  }
};

const saveProjectSettings = async () => {
  if (!project.value) return;

  saveError.value = null;
  try {
    const tags = editTags.value
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t.length > 0);

    await projectsStore.updateProject(project.value.id, {
      name: editName.value,
      path: editPath.value,
      description: editDescription.value || undefined,
      tags,
    });
    isEditing.value = false;
  } catch (e) {
    console.error("[ProjectDetail] Failed to save project settings:", e);
    saveError.value = String(e);
  }
};

const cancelEdit = () => {
  if (project.value) {
    editName.value = project.value.name;
    editPath.value = project.value.path;
    editDescription.value = project.value.description || "";
    editTags.value = project.value.tags.join(", ");
  }
  isEditing.value = false;
  saveError.value = null;
};

const addEnvironment = async (name: string, variables: Record<string, string>) => {
  if (!project.value) return;

  const updatedEnvironments = {
    ...project.value.environments,
    [name]: { name, variables },
  };

  try {
    await projectsStore.updateProject(project.value.id, {
      environments: updatedEnvironments,
    });
  } catch (e) {
    console.error("Failed to add environment:", e);
    alert("Failed to add environment: " + e);
  }
};

const addWorkspace = async (workspace: Omit<Workspace, "id">) => {
  if (!project.value) return;

  const newWorkspace: Workspace = {
    ...workspace,
    id: crypto.randomUUID(),
  };

  try {
    await projectsStore.updateProject(project.value.id, {
      workspaces: [...project.value.workspaces, newWorkspace],
    });
  } catch (e) {
    console.error("Failed to add workspace:", e);
    alert("Failed to add workspace: " + e);
  }
};

const addCommand = async (command: Omit<Command, "id">) => {
  if (!project.value) return;

  const newCommand: Command = {
    ...command,
    id: crypto.randomUUID(),
  };

  try {
    await projectsStore.updateProject(project.value.id, {
      commands: [...project.value.commands, newCommand],
    });
  } catch (e) {
    console.error("[ProjectDetail] Failed to add command:", e);
    alert("Failed to add command: " + e);
  }
};

// Environment edit/delete
const startEditEnvironment = (name: string, env: Environment) => {
  editingEnvironment.value = { name, env: { ...env, variables: { ...env.variables } } };
  showEditEnvironment.value = true;
};

const saveEnvironment = async (originalName: string, name: string, variables: Record<string, string>) => {
  if (!project.value) return;

  try {
    const updatedEnvironments = { ...project.value.environments };
    if (originalName !== name) {
      delete updatedEnvironments[originalName];
    }
    updatedEnvironments[name] = { name, variables };

    await projectsStore.updateProject(project.value.id, {
      environments: updatedEnvironments,
    });
    showEditEnvironment.value = false;
    editingEnvironment.value = null;
  } catch (e) {
    console.error("[ProjectDetail] Failed to save environment:", e);
    alert("Failed to save environment: " + e);
  }
};

const confirmDeleteEnvironment = (name: string) => {
  deletingEnvironmentName.value = name;
  showDeleteEnvironmentConfirm.value = true;
};

const deleteEnvironment = async () => {
  if (!project.value || !deletingEnvironmentName.value) return;

  try {
    const updatedEnvironments = { ...project.value.environments };
    delete updatedEnvironments[deletingEnvironmentName.value];

    await projectsStore.updateProject(project.value.id, {
      environments: updatedEnvironments,
    });
    showDeleteEnvironmentConfirm.value = false;
    deletingEnvironmentName.value = null;
  } catch (e) {
    console.error("[ProjectDetail] Failed to delete environment:", e);
    alert("Failed to delete environment: " + e);
  }
};

// Workspace edit/delete/launch
const startEditWorkspace = (workspace: Workspace) => {
  editingWorkspace.value = JSON.parse(JSON.stringify(workspace));
  paneCommandTypeOverrides.value.clear();
  showEditWorkspace.value = true;
};

// Helper functions for editing workspace layout
const getPaneForPosition = (row: number, col: number): Pane | undefined => {
  if (!editingWorkspace.value) return undefined;
  return editingWorkspace.value.panes.find(
    (p) => p.position[0] === row && p.position[1] === col
  );
};

const updatePaneDirectory = (row: number, col: number, value: string) => {
  if (!editingWorkspace.value) return;
  const pane = editingWorkspace.value.panes.find(
    (p) => p.position[0] === row && p.position[1] === col
  );
  if (pane) {
    pane.directory = value;
  }
};

const getPaneCommandType = (row: number, col: number): 'none' | 'preset' | 'custom' => {
  const key = `${row},${col}`;
  const override = paneCommandTypeOverrides.value.get(key);
  if (override !== undefined) {
    return override;
  }

  const pane = getPaneForPosition(row, col);
  if (!pane?.command) return 'none';

  const globalCmd = globalCommands.value.find(c => c.command === pane.command);
  if (globalCmd) return 'preset';

  const projectCmd = project.value?.commands.find(c => c.command === pane.command);
  if (projectCmd) return 'preset';

  return 'custom';
};

const getPaneSelectedCommandId = (row: number, col: number): string => {
  const pane = getPaneForPosition(row, col);
  if (!pane?.command) return '';

  const globalCmd = globalCommands.value.find(c => c.command === pane.command);
  if (globalCmd) return globalCmd.id;

  const projectCmd = project.value?.commands.find(c => c.command === pane.command);
  if (projectCmd) return projectCmd.id;

  return '';
};

const getPaneCustomCommand = (row: number, col: number): string => {
  const pane = getPaneForPosition(row, col);
  if (!pane?.command) return '';

  if (getPaneCommandType(row, col) === 'custom') {
    return pane.command;
  }
  return '';
};

const allCommands = computed(() => {
  const cmds: { id: string; name: string; command: string; isGlobal: boolean }[] = [];
  globalCommands.value.forEach(cmd => {
    cmds.push({ ...cmd, isGlobal: true });
  });
  project.value?.commands.forEach(cmd => {
    cmds.push({ ...cmd, isGlobal: false });
  });
  return cmds;
});

const updatePaneCommandType = (row: number, col: number, type: 'none' | 'preset' | 'custom') => {
  if (!editingWorkspace.value) return;
  const pane = editingWorkspace.value.panes.find(
    (p) => p.position[0] === row && p.position[1] === col
  );
  if (!pane) return;

  const key = `${row},${col}`;
  paneCommandTypeOverrides.value.set(key, type);

  if (type === 'none') {
    pane.command = undefined;
  } else if (type === 'custom') {
    pane.command = '';
  } else if (type === 'preset') {
    pane.command = undefined;
  }
};

const updatePaneSelectedCommand = (row: number, col: number, commandId: string) => {
  if (!editingWorkspace.value) return;
  const pane = editingWorkspace.value.panes.find(
    (p) => p.position[0] === row && p.position[1] === col
  );
  if (!pane) return;

  const cmd = allCommands.value.find(c => c.id === commandId);
  if (cmd) {
    pane.command = cmd.command;
    const key = `${row},${col}`;
    paneCommandTypeOverrides.value.delete(key);
  }
};

const updatePaneCustomCommand = (row: number, col: number, value: string) => {
  if (!editingWorkspace.value) return;
  const pane = editingWorkspace.value.panes.find(
    (p) => p.position[0] === row && p.position[1] === col
  );
  if (pane) {
    pane.command = value || undefined;
  }
};

const getCommandDisplayText = (pane: Pane | undefined): string => {
  if (!pane?.command) return '';

  const globalCmd = globalCommands.value.find(c => c.command === pane.command);
  if (globalCmd) return globalCmd.name;

  const projectCmd = project.value?.commands.find(c => c.command === pane.command);
  if (projectCmd) return projectCmd.name;

  return pane.command.length > 20 ? pane.command.slice(0, 20) + '...' : pane.command;
};

const getWorkspacePaneAt = (workspace: Workspace, row: number, col: number): Pane | undefined => {
  return workspace.panes.find(p => p.position[0] === row && p.position[1] === col);
};

const toRelativePath = (absolutePath: string, projectRoot: string): string => {
  if (!projectRoot) return absolutePath;

  const normalizedProject = projectRoot.endsWith('/') ? projectRoot.slice(0, -1) : projectRoot;
  const normalizedPath = absolutePath.endsWith('/') ? absolutePath.slice(0, -1) : absolutePath;

  if (normalizedPath === normalizedProject) {
    return '.';
  }

  if (normalizedPath.startsWith(normalizedProject + '/')) {
    return normalizedPath.slice(normalizedProject.length + 1);
  }

  return absolutePath;
};

const toAbsolutePath = (path: string, projectRoot: string): string => {
  if (!path || path === '.') {
    return projectRoot;
  }
  if (path.startsWith('/')) {
    return path;
  }
  return `${projectRoot}/${path}`;
};

const browsePaneDirectory = async (row: number, col: number) => {
  if (!editingWorkspace.value || !project.value) return;
  const pane = editingWorkspace.value.panes.find(
    (p) => p.position[0] === row && p.position[1] === col
  );
  if (!pane) return;

  try {
    const defaultPath = pane.directory
      ? toAbsolutePath(pane.directory, project.value.path)
      : project.value.path;

    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: defaultPath || undefined,
    });
    if (selected) {
      pane.directory = toRelativePath(selected as string, project.value.path);
    }
  } catch (e) {
    console.error("Failed to open folder picker:", e);
  }
};

const addWorkspaceRow = () => {
  if (!editingWorkspace.value || !project.value) return;
  const newRowIndex = editingWorkspace.value.layout.columns.length;
  editingWorkspace.value.layout.columns.push(1);
  editingWorkspace.value.layout.rows = editingWorkspace.value.layout.columns.length;
  editingWorkspace.value.panes.push({
    position: [newRowIndex, 0] as [number, number],
    directory: "",
    command: undefined,
  });
};

const removeWorkspaceRow = (rowIndex: number) => {
  if (!editingWorkspace.value) return;
  if (editingWorkspace.value.layout.columns.length <= 1) return;

  editingWorkspace.value.layout.columns.splice(rowIndex, 1);
  editingWorkspace.value.layout.rows = editingWorkspace.value.layout.columns.length;

  editingWorkspace.value.panes = editingWorkspace.value.panes.filter(
    (p) => p.position[0] !== rowIndex
  );

  editingWorkspace.value.panes.forEach((p) => {
    if (p.position[0] > rowIndex) {
      p.position = [p.position[0] - 1, p.position[1]];
    }
  });
};

const updateWorkspaceRowColumns = (rowIndex: number, newCols: number) => {
  if (!editingWorkspace.value || !project.value) return;
  const oldCols = editingWorkspace.value.layout.columns[rowIndex];

  if (newCols > oldCols) {
    for (let i = oldCols; i < newCols; i++) {
      editingWorkspace.value.panes.push({
        position: [rowIndex, i] as [number, number],
        directory: "",
        command: undefined,
      });
    }
  } else if (newCols < oldCols) {
    editingWorkspace.value.panes = editingWorkspace.value.panes.filter(
      (p) => !(p.position[0] === rowIndex && p.position[1] >= newCols)
    );
  }

  editingWorkspace.value.layout.columns[rowIndex] = newCols;
};

const saveWorkspace = async (workspace: Workspace) => {
  if (!project.value) return;

  try {
    const updatedWorkspaces = project.value.workspaces.map((w) =>
      w.id === workspace.id ? workspace : w
    );

    await projectsStore.updateProject(project.value.id, {
      workspaces: updatedWorkspaces,
    });
    showEditWorkspace.value = false;
    editingWorkspace.value = null;
  } catch (e) {
    console.error("[ProjectDetail] Failed to save workspace:", e);
    alert("Failed to save workspace: " + e);
  }
};

const confirmDeleteWorkspace = (workspace: Workspace) => {
  deletingWorkspace.value = workspace;
  showDeleteWorkspaceConfirm.value = true;
};

const deleteWorkspace = async () => {
  if (!project.value || !deletingWorkspace.value) return;

  try {
    const updatedWorkspaces = project.value.workspaces.filter(
      (w) => w.id !== deletingWorkspace.value!.id
    );

    await projectsStore.updateProject(project.value.id, {
      workspaces: updatedWorkspaces,
    });
    showDeleteWorkspaceConfirm.value = false;
    deletingWorkspace.value = null;
  } catch (e) {
    console.error("[ProjectDetail] Failed to delete workspace:", e);
    alert("Failed to delete workspace: " + e);
  }
};

const launchWorkspace = async (workspaceId: string) => {
  if (!project.value) return;

  try {
    await invoke("launch_workspace", {
      projectId: project.value.id,
      workspaceId,
    });
  } catch (e) {
    console.error("[ProjectDetail] Failed to launch workspace:", e);
    alert("Failed to launch workspace: " + e);
  }
};

// Command edit/delete/run
const startEditCommand = (command: Command) => {
  editingCommand.value = { ...command };
  showEditCommand.value = true;
};

const saveCommand = async (command: Command) => {
  if (!project.value) return;

  try {
    const updatedCommands = project.value.commands.map((c) =>
      c.id === command.id ? command : c
    );

    await projectsStore.updateProject(project.value.id, {
      commands: updatedCommands,
    });
    showEditCommand.value = false;
    editingCommand.value = null;
  } catch (e) {
    console.error("[ProjectDetail] Failed to save command:", e);
    alert("Failed to save command: " + e);
  }
};

const confirmDeleteCommand = (command: Command) => {
  deletingCommand.value = command;
  showDeleteCommandConfirm.value = true;
};

const deleteCommand = async () => {
  if (!project.value || !deletingCommand.value) return;

  try {
    const updatedCommands = project.value.commands.filter(
      (c) => c.id !== deletingCommand.value!.id
    );

    await projectsStore.updateProject(project.value.id, {
      commands: updatedCommands,
    });
    showDeleteCommandConfirm.value = false;
    deletingCommand.value = null;
  } catch (e) {
    console.error("[ProjectDetail] Failed to delete command:", e);
    alert("Failed to delete command: " + e);
  }
};

const runCommand = async (command: Command) => {
  if (!project.value) return;

  try {
    await invoke("run_command", {
      projectId: project.value.id,
      command: command.command,
    });
  } catch (e) {
    console.error("[ProjectDetail] Failed to run command:", e);
    alert("Failed to run command: " + e);
  }
};
</script>

<template>
  <div class="project-detail" v-if="project">
    <header class="page-header">
      <div class="breadcrumb">
        <router-link to="/projects">Projects</router-link>
        <i class="pi pi-chevron-right"></i>
        <span>{{ project.name }}</span>
      </div>

      <div class="header-actions">
        <button class="btn btn-secondary" @click="showDeleteConfirm = true">
          <i class="pi pi-trash"></i>
          Delete
        </button>
        <button class="btn btn-primary" @click="openProject">
          <i class="pi pi-folder-open"></i>
          Open Project
        </button>
      </div>
    </header>

    <!-- Project Header Card -->
    <div class="project-header card">
      <div class="project-icon">
        <i class="pi pi-folder"></i>
      </div>
      <div class="project-info">
        <h1>{{ project.name }}</h1>
        <p class="project-path">{{ project.path }}</p>
        <p v-if="project.description" class="project-description">
          {{ project.description }}
        </p>
        <div v-if="project.tags.length" class="tags">
          <span v-for="tag in project.tags" :key="tag" class="tag">{{ tag }}</span>
        </div>
      </div>
    </div>

    <!-- Overview Section -->
    <section class="page-section">
      <div class="stats-grid">
        <div class="stat-card card-flat">
          <div class="stat-value">{{ Object.keys(project.environments).length }}</div>
          <div class="stat-label">Environments</div>
        </div>
        <div class="stat-card card-flat">
          <div class="stat-value">{{ project.workspaces.length }}</div>
          <div class="stat-label">Workspaces</div>
        </div>
        <div class="stat-card card-flat">
          <div class="stat-value">{{ project.commands.length }}</div>
          <div class="stat-label">Commands</div>
        </div>
      </div>

      <div class="project-settings card-flat">
        <div class="settings-header">
          <h3>Project Settings</h3>
          <button v-if="!isEditing" class="btn btn-secondary btn-sm" @click="isEditing = true">
            <i class="pi pi-pencil"></i>
            Edit
          </button>
        </div>

        <div v-if="saveError" class="error-message">
          {{ saveError }}
        </div>

        <div class="setting-item">
          <label>Name</label>
          <input v-if="isEditing" v-model="editName" type="text" placeholder="Project name" />
          <span v-else class="setting-value">{{ project.name }}</span>
        </div>

        <div class="setting-item">
          <label>Path</label>
          <div v-if="isEditing" class="path-input">
            <input v-model="editPath" type="text" placeholder="/path/to/project" />
            <button class="btn btn-secondary btn-sm" @click="browsePath">
              <i class="pi pi-folder-open"></i>
              Browse
            </button>
          </div>
          <span v-else class="setting-value monospace">{{ project.path }}</span>
        </div>

        <div class="setting-item">
          <label>Description</label>
          <textarea v-if="isEditing" v-model="editDescription" placeholder="Project description (optional)" rows="2"></textarea>
          <span v-else class="setting-value">{{ project.description || "(none)" }}</span>
        </div>

        <div class="setting-item">
          <label>Tags</label>
          <input v-if="isEditing" v-model="editTags" type="text" placeholder="Comma-separated tags" />
          <span v-else class="setting-value">{{ project.tags.join(", ") || "(none)" }}</span>
        </div>

        <div v-if="isEditing" class="settings-actions">
          <button class="btn btn-secondary" @click="cancelEdit">Cancel</button>
          <button class="btn btn-primary" @click="saveProjectSettings">Save Changes</button>
        </div>
      </div>
    </section>

    <!-- Environments Section -->
    <section class="page-section">
      <div class="page-section-header">
        <h2>Environments</h2>
        <button class="btn btn-primary btn-sm" @click="showAddEnvironment = true">
          <i class="pi pi-plus"></i>
          Add Environment
        </button>
      </div>

      <div v-if="Object.keys(project.environments).length === 0" class="empty-state">
        <p>No environments configured</p>
        <button class="btn btn-primary btn-sm" @click="showAddEnvironment = true">
          <i class="pi pi-plus"></i> Add Environment
        </button>
      </div>
      <div v-else class="env-list">
        <div v-for="(env, name) in project.environments" :key="name" class="env-card card-flat">
          <div class="card-header">
            <h3>{{ name }}</h3>
            <div class="action-group">
              <button class="btn btn-secondary btn-sm" @click="startEditEnvironment(String(name), env)">
                <i class="pi pi-pencil"></i> Edit
              </button>
              <button class="btn btn-secondary btn-sm btn-danger-ghost" @click="confirmDeleteEnvironment(String(name))">
                <i class="pi pi-trash"></i> Delete
              </button>
            </div>
          </div>
          <div class="env-vars">
            <div v-for="(value, key) in env.variables" :key="key" class="env-var">
              <span class="env-key">{{ key }}</span>
              <span class="env-value">{{ value }}</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Workspaces Section -->
    <section class="page-section">
      <div class="page-section-header">
        <h2>Workspaces</h2>
        <button class="btn btn-primary btn-sm" @click="showAddWorkspace = true">
          <i class="pi pi-plus"></i>
          Add Workspace
        </button>
      </div>

      <div v-if="project.workspaces.length === 0" class="empty-state">
        <p>No workspaces configured</p>
        <button class="btn btn-primary btn-sm" @click="showAddWorkspace = true">
          <i class="pi pi-plus"></i> Add Workspace
        </button>
      </div>
      <div v-else class="workspace-list">
        <div v-for="workspace in project.workspaces" :key="workspace.id" class="workspace-card card-flat">
          <div class="card-header">
            <h3>{{ workspace.name }}</h3>
            <div class="action-group">
              <button class="btn btn-primary btn-sm" @click="launchWorkspace(workspace.id)">
                <i class="pi pi-play"></i> Launch
              </button>
              <button class="btn btn-secondary btn-sm" @click="startEditWorkspace(workspace)">
                <i class="pi pi-pencil"></i> Edit
              </button>
              <button class="btn btn-secondary btn-sm btn-danger-ghost" @click="confirmDeleteWorkspace(workspace)">
                <i class="pi pi-trash"></i> Delete
              </button>
            </div>
          </div>

          <div class="workspace-preview">
            <div class="layout-preview">
              <div
                v-for="(cols, row) in workspace.layout.columns"
                :key="row"
                class="layout-row"
                :style="{ gridTemplateColumns: `repeat(${cols}, 1fr)` }"
              >
                <div v-for="col in cols" :key="col" class="layout-cell">
                  <span class="cell-command">{{ getCommandDisplayText(getWorkspacePaneAt(workspace, row, col - 1)) }}</span>
                </div>
              </div>
            </div>
          </div>

          <p class="pane-count">{{ workspace.panes.length }} panes</p>
        </div>
      </div>
    </section>

    <!-- Commands Section -->
    <section class="page-section">
      <div class="page-section-header">
        <h2>Commands</h2>
        <button class="btn btn-primary btn-sm" @click="showAddCommand = true">
          <i class="pi pi-plus"></i>
          Add Command
        </button>
      </div>

      <div v-if="project.commands.length === 0" class="empty-state">
        <p>No commands configured</p>
        <button class="btn btn-primary btn-sm" @click="showAddCommand = true">
          <i class="pi pi-plus"></i> Add Command
        </button>
      </div>
      <div v-else class="command-list">
        <div v-for="cmd in project.commands" :key="cmd.id" class="command-card card-flat">
          <div class="card-header">
            <h3>{{ cmd.name }}</h3>
            <div class="action-group">
              <button class="btn btn-primary btn-sm" @click="runCommand(cmd)">
                <i class="pi pi-play"></i> Run
              </button>
              <button class="btn btn-secondary btn-sm" @click="startEditCommand(cmd)">
                <i class="pi pi-pencil"></i> Edit
              </button>
              <button class="btn btn-secondary btn-sm btn-danger-ghost" @click="confirmDeleteCommand(cmd)">
                <i class="pi pi-trash"></i> Delete
              </button>
            </div>
          </div>
          <code>{{ cmd.command }}</code>
          <p v-if="cmd.description" class="command-description">{{ cmd.description }}</p>
        </div>
      </div>
    </section>

    <!-- Confirm Delete Dialog -->
    <ConfirmDialog
      v-model:visible="showDeleteConfirm"
      title="Delete Project"
      :message="`Are you sure you want to delete '${project.name}'? This action cannot be undone.`"
      confirm-text="Delete"
      :danger="true"
      @confirm="deleteProject"
    />

    <!-- Add Modals -->
    <AddEnvironmentModal
      v-model:visible="showAddEnvironment"
      @add="addEnvironment"
    />

    <AddWorkspaceModal
      v-model:visible="showAddWorkspace"
      :project-path="project.path"
      :commands="project.commands"
      :global-commands="globalCommands"
      @add="addWorkspace"
    />

    <AddCommandModal
      v-model:visible="showAddCommand"
      @add="addCommand"
    />

    <!-- Edit Environment Modal -->
    <div v-if="showEditEnvironment && editingEnvironment" class="modal-overlay" @click.self="showEditEnvironment = false">
      <div class="modal">
        <div class="modal-header">
          <h2>Edit Environment</h2>
          <button class="btn btn-ghost" @click="showEditEnvironment = false">
            <i class="pi pi-times"></i>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Environment Name</label>
            <input v-model="editingEnvironment.env.name" type="text" placeholder="e.g., development, production" />
          </div>
          <div class="form-group">
            <label>Variables</label>
            <div class="env-var-list">
              <div v-for="(_value, key) in editingEnvironment.env.variables" :key="key" class="env-var-row">
                <input :value="key" type="text" placeholder="KEY" @input="(e: Event) => {
                  const newKey = (e.target as HTMLInputElement).value;
                  const oldValue = editingEnvironment!.env.variables[key as string];
                  delete editingEnvironment!.env.variables[key as string];
                  editingEnvironment!.env.variables[newKey] = oldValue;
                }" />
                <input v-model="editingEnvironment.env.variables[key as string]" type="text" placeholder="value" />
                <button class="btn btn-secondary btn-sm btn-danger-ghost" @click="delete editingEnvironment!.env.variables[key as string]">
                  <i class="pi pi-trash"></i>
                </button>
              </div>
            </div>
            <button class="btn btn-secondary btn-sm" @click="editingEnvironment.env.variables[''] = ''">
              <i class="pi pi-plus"></i> Add Variable
            </button>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showEditEnvironment = false">Cancel</button>
          <button class="btn btn-primary" @click="saveEnvironment(editingEnvironment.name, editingEnvironment.env.name, editingEnvironment.env.variables)">Save</button>
        </div>
      </div>
    </div>

    <!-- Edit Command Modal -->
    <div v-if="showEditCommand && editingCommand" class="modal-overlay" @click.self="showEditCommand = false">
      <div class="modal">
        <div class="modal-header">
          <h2>Edit Command</h2>
          <button class="btn btn-ghost" @click="showEditCommand = false">
            <i class="pi pi-times"></i>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Command Name</label>
            <input v-model="editingCommand.name" type="text" placeholder="e.g., Start Server" />
          </div>
          <div class="form-group">
            <label>Command</label>
            <input v-model="editingCommand.command" type="text" placeholder="e.g., npm run dev" />
          </div>
          <div class="form-group">
            <label>Description (optional)</label>
            <textarea v-model="editingCommand.description" placeholder="What does this command do?" rows="2"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showEditCommand = false">Cancel</button>
          <button class="btn btn-primary" @click="saveCommand(editingCommand)">Save</button>
        </div>
      </div>
    </div>

    <!-- Edit Workspace Modal -->
    <div v-if="showEditWorkspace && editingWorkspace" class="modal-overlay" @click.self="showEditWorkspace = false">
      <div class="modal modal-large">
        <div class="modal-header">
          <h2>Edit Workspace</h2>
          <button class="btn btn-ghost" @click="showEditWorkspace = false">
            <i class="pi pi-times"></i>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Workspace Name</label>
            <input v-model="editingWorkspace.name" type="text" placeholder="e.g., Development" />
          </div>

          <div class="form-group">
            <div class="section-header">
              <label>Layout ({{ editingWorkspace.panes.length }} panes)</label>
              <button type="button" class="btn btn-secondary btn-sm" @click="addWorkspaceRow">
                <i class="pi pi-plus"></i>
                Add Row
              </button>
            </div>

            <div class="rows-list">
              <div v-for="(cols, rowIndex) in editingWorkspace.layout.columns" :key="rowIndex" class="row-config">
                <div class="row-header">
                  <span class="row-label">Row {{ rowIndex + 1 }}</span>
                  <div class="row-controls">
                    <label class="column-label">Panes:</label>
                    <select
                      :value="cols"
                      @change="updateWorkspaceRowColumns(rowIndex, parseInt(($event.target as HTMLSelectElement).value))"
                    >
                      <option v-for="n in 6" :key="n" :value="n">{{ n }}</option>
                    </select>
                    <button
                      v-if="editingWorkspace.layout.columns.length > 1"
                      type="button"
                      class="btn btn-secondary btn-sm btn-danger-ghost"
                      @click="removeWorkspaceRow(rowIndex)"
                    >
                      <i class="pi pi-trash"></i> Remove
                    </button>
                  </div>
                </div>

                <div class="panes-grid" :style="{ gridTemplateColumns: `repeat(${cols}, 1fr)` }">
                  <div v-for="colIndex in cols" :key="colIndex" class="pane-config">
                    <div class="pane-header">Pane {{ colIndex }}</div>

                    <div class="pane-field">
                      <label>Directory</label>
                      <div class="input-with-browse">
                        <input
                          :value="getPaneForPosition(rowIndex, colIndex - 1)?.directory || ''"
                          @input="updatePaneDirectory(rowIndex, colIndex - 1, ($event.target as HTMLInputElement).value)"
                          type="text"
                          placeholder="Use project default"
                          class="pane-input"
                        />
                        <button
                          type="button"
                          class="browse-btn"
                          @click="browsePaneDirectory(rowIndex, colIndex - 1)"
                          title="Browse"
                        >
                          <i class="pi pi-folder-open"></i>
                        </button>
                      </div>
                    </div>

                    <div class="pane-field">
                      <label>Command</label>
                      <select
                        class="pane-select"
                        :value="getPaneCommandType(rowIndex, colIndex - 1)"
                        @change="updatePaneCommandType(rowIndex, colIndex - 1, ($event.target as HTMLSelectElement).value as 'none' | 'preset' | 'custom')"
                      >
                        <option value="none">None</option>
                        <option value="preset" :disabled="allCommands.length === 0">Select command</option>
                        <option value="custom">Custom command</option>
                      </select>

                      <select
                        v-if="getPaneCommandType(rowIndex, colIndex - 1) === 'preset'"
                        class="pane-select"
                        :value="getPaneSelectedCommandId(rowIndex, colIndex - 1)"
                        @change="updatePaneSelectedCommand(rowIndex, colIndex - 1, ($event.target as HTMLSelectElement).value)"
                      >
                        <option value="">Select a command...</option>
                        <optgroup v-if="globalCommands.length > 0" label="Global Commands">
                          <option v-for="cmd in globalCommands" :key="cmd.id" :value="cmd.id">
                            {{ cmd.name }}
                          </option>
                        </optgroup>
                        <optgroup v-if="project?.commands.length" label="Project Commands">
                          <option v-for="cmd in project?.commands" :key="cmd.id" :value="cmd.id">
                            {{ cmd.name }}
                          </option>
                        </optgroup>
                      </select>

                      <input
                        v-if="getPaneCommandType(rowIndex, colIndex - 1) === 'custom'"
                        :value="getPaneCustomCommand(rowIndex, colIndex - 1)"
                        @input="updatePaneCustomCommand(rowIndex, colIndex - 1, ($event.target as HTMLInputElement).value)"
                        type="text"
                        placeholder="Enter command..."
                        class="pane-input"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="layout-preview-section">
            <label>Preview</label>
            <div class="preview-grid">
              <div
                v-for="(cols, rowIndex) in editingWorkspace.layout.columns"
                :key="rowIndex"
                class="preview-row"
                :style="{ gridTemplateColumns: `repeat(${cols}, 1fr)` }"
              >
                <div v-for="colIndex in cols" :key="colIndex" class="preview-pane">
                  <span class="preview-command">{{ getCommandDisplayText(getPaneForPosition(rowIndex, colIndex - 1)) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showEditWorkspace = false">Cancel</button>
          <button class="btn btn-primary" @click="saveWorkspace(editingWorkspace)">Save</button>
        </div>
      </div>
    </div>

    <!-- Delete Environment Confirm -->
    <ConfirmDialog
      v-model:visible="showDeleteEnvironmentConfirm"
      title="Delete Environment"
      :message="`Are you sure you want to delete the environment '${deletingEnvironmentName}'?`"
      confirm-text="Delete"
      :danger="true"
      @confirm="deleteEnvironment"
    />

    <!-- Delete Workspace Confirm -->
    <ConfirmDialog
      v-model:visible="showDeleteWorkspaceConfirm"
      title="Delete Workspace"
      :message="`Are you sure you want to delete the workspace '${deletingWorkspace?.name}'?`"
      confirm-text="Delete"
      :danger="true"
      @confirm="deleteWorkspace"
    />

    <!-- Delete Command Confirm -->
    <ConfirmDialog
      v-model:visible="showDeleteCommandConfirm"
      title="Delete Command"
      :message="`Are you sure you want to delete the command '${deletingCommand?.name}'?`"
      confirm-text="Delete"
      :danger="true"
      @confirm="deleteCommand"
    />
  </div>

  <div v-else class="loading">
    Loading project...
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary);
}

.breadcrumb a {
  color: var(--accent);
  text-decoration: none;
}

.breadcrumb a:hover {
  text-decoration: underline;
}

.breadcrumb i {
  font-size: 12px;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.header-actions .btn {
  display: flex;
  align-items: center;
  gap: 8px;
}

.project-header {
  display: flex;
  gap: 20px;
  margin-bottom: 24px;
}

.project-icon {
  width: 64px;
  height: 64px;
  background: var(--accent-muted);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent);
  font-size: 24px;
  flex-shrink: 0;
}

.project-info h1 {
  font-size: 24px;
  margin-bottom: 4px;
}

.project-path {
  font-family: "SF Mono", monospace;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.project-description {
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.tags {
  display: flex;
  gap: 6px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  text-align: center;
  padding: 16px;
}

.stat-value {
  font-size: 36px;
  font-weight: 700;
  color: var(--accent);
}

.stat-label {
  color: var(--text-secondary);
  font-size: 14px;
}

.empty-state {
  text-align: center;
  padding: 32px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.empty-state p {
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.env-list,
.workspace-list,
.command-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.env-card,
.workspace-card,
.command-card {
  padding: 16px;
}

.env-card h3,
.workspace-card h3,
.command-card h3 {
  font-size: 15px;
  margin: 0;
}

.env-vars {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.env-var {
  display: flex;
  gap: 12px;
  font-family: "SF Mono", monospace;
  font-size: 13px;
}

.env-key {
  color: var(--accent);
}

.env-value {
  color: var(--text-secondary);
}

.loading {
  text-align: center;
  padding: 48px;
  color: var(--text-secondary);
}

/* Project Settings */
.project-settings {
  padding: 16px;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.settings-header h3 {
  font-size: 16px;
  margin: 0;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-subtle);
}

.setting-item:last-of-type {
  border-bottom: none;
}

.setting-item label {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
}

.setting-item input,
.setting-item textarea {
  padding: 10px 14px;
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 14px;
  width: 100%;
}

.setting-item input:focus,
.setting-item textarea:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-muted);
}

.setting-value {
  font-size: 14px;
}

.setting-value.monospace {
  font-family: "SF Mono", monospace;
}

.path-input {
  display: flex;
  gap: 8px;
}

.path-input input {
  flex: 1;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-subtle);
}

.error-message {
  background: var(--danger-muted);
  border: 1px solid var(--danger);
  color: var(--danger);
  padding: 12px;
  border-radius: 8px;
  margin-bottom: 16px;
  font-size: 13px;
}

/* Card headers with actions */
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

/* Danger ghost button */
.btn-danger-ghost {
  color: var(--text-secondary);
}

.btn-danger-ghost:hover {
  background: var(--danger-muted) !important;
  border-color: var(--danger) !important;
  color: var(--danger) !important;
}

/* Workspace/Command actions */
.pane-count {
  color: var(--text-secondary);
  font-size: 13px;
  margin: 0;
}

.command-description {
  color: var(--text-secondary);
  font-size: 13px;
  margin: 8px 0 0 0;
}

code {
  font-family: "SF Mono", monospace;
  font-size: 13px;
  background: var(--bg-tertiary);
  padding: 4px 8px;
  border-radius: 4px;
  color: var(--accent);
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-card);
  border-radius: 12px;
  width: 100%;
  max-width: 500px;
  max-height: 80vh;
  overflow-y: auto;
  border: 1px solid var(--border-primary);
  box-shadow: var(--shadow-lg);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-primary);
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
  text-transform: none;
  letter-spacing: normal;
  color: var(--text-primary);
}

.modal-body {
  padding: 20px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 20px;
  border-top: 1px solid var(--border-primary);
}

.form-group {
  margin-bottom: 16px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
  font-weight: 500;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 14px;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-muted);
}

.env-var-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.env-var-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.env-var-row input:first-child {
  flex: 0 0 120px;
}

.env-var-row input:nth-child(2) {
  flex: 1;
}

/* Workspace preview in cards */
.workspace-preview {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 8px;
  margin: 12px 0;
  height: 80px;
}

.layout-preview {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.layout-row {
  display: grid;
  gap: 4px;
  flex: 1;
}

.layout-cell {
  background: var(--bg-secondary);
  border-radius: 4px;
  border: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  padding: 2px;
}

.cell-command {
  font-size: 8px;
  color: var(--text-muted);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
  font-family: "SF Mono", monospace;
}

/* Larger modal for workspace editor */
.modal-large {
  max-width: 1000px;
}

/* Workspace edit modal styles */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-header label {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 0;
}

.rows-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.row-config {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 12px;
}

.row-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.row-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.row-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.column-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.row-controls select {
  padding: 6px 10px;
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
}

.panes-grid {
  display: grid;
  gap: 8px;
}

.pane-config {
  background: var(--bg-tertiary);
  border-radius: 6px;
  padding: 10px;
}

.pane-header {
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: 6px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.pane-input {
  margin-bottom: 6px;
  padding: 8px !important;
  font-size: 12px !important;
}

.pane-field {
  margin-bottom: 8px;
}

.pane-field:last-child {
  margin-bottom: 0;
}

.pane-field > label {
  display: block;
  font-size: 10px;
  color: var(--text-secondary);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.input-with-browse {
  display: flex;
  gap: 4px;
}

.input-with-browse input {
  flex: 1;
}

.browse-btn {
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-secondary);
  padding: 0 10px;
  cursor: pointer;
  transition: all 0.2s;
}

.browse-btn:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
}

.pane-select {
  width: 100%;
  padding: 8px;
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 12px;
}

.pane-select:focus {
  outline: none;
  border-color: var(--accent);
}

.layout-preview-section {
  margin-top: 16px;
}

.layout-preview-section label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 8px;
  color: var(--text-secondary);
}

.preview-grid {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.preview-row {
  display: grid;
  gap: 4px;
}

.preview-pane {
  height: 32px;
  background: var(--bg-secondary);
  border-radius: 4px;
  border: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  padding: 2px;
}

.preview-command {
  font-size: 8px;
  color: var(--text-muted);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
  font-family: "SF Mono", monospace;
}
</style>
