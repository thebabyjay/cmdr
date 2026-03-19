<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useProjectsStore } from "@/stores/projects";
import { useSettingsStore } from "@/stores/settings";
import { invoke } from "@tauri-apps/api/core";
import type { Workspace, Project, Pane } from "@/types";

const projectsStore = useProjectsStore();
const settingsStore = useSettingsStore();

const globalCommands = computed(() => settingsStore.settings.globalCommands || []);

onMounted(() => {
  projectsStore.loadProjects();
  settingsStore.loadSettings();
});

const allWorkspaces = computed(() => {
  const workspaces: Array<{ workspace: Workspace; project: Project }> = [];
  for (const project of projectsStore.projects) {
    for (const workspace of project.workspaces) {
      workspaces.push({ workspace, project });
    }
  }
  return workspaces;
});

const launchWorkspace = async (projectId: string, workspaceId: string) => {
  try {
    await invoke("launch_workspace", {
      projectId,
      workspaceId,
    });
  } catch (e) {
    console.error("[WorkspacesView] Failed to launch workspace:", e);
    alert("Failed to launch workspace: " + e);
  }
};

// Get pane for a workspace by position
const getWorkspacePaneAt = (workspace: Workspace, row: number, col: number): Pane | undefined => {
  return workspace.panes.find(p => p.position[0] === row && p.position[1] === col);
};

// Get display text for a pane's command (for preview)
const getCommandDisplayText = (pane: Pane | undefined, project: Project): string => {
  if (!pane?.command) return '';

  const globalCmd = globalCommands.value.find(c => c.command === pane.command);
  if (globalCmd) return globalCmd.name;

  const projectCmd = project.commands.find(c => c.command === pane.command);
  if (projectCmd) return projectCmd.name;

  return pane.command.length > 20 ? pane.command.slice(0, 20) + '...' : pane.command;
};

// Get directory display for a pane
const getPaneDirectoryDisplay = (pane: Pane | undefined): string => {
  if (!pane) return './';
  if (!pane.directory || pane.directory === '.') return './';
  return pane.directory.length > 20 ? '.../' + pane.directory.split('/').pop() : pane.directory;
};

const getPaneCount = (workspace: Workspace) => {
  return workspace.layout?.columns?.reduce((sum: number, cols: number) => sum + cols, 0) || 0;
};

const getLayoutDisplay = (workspace: Workspace) => {
  if (!workspace.layout?.columns) return "";
  const rows = workspace.layout.columns.length;
  const totalPanes = getPaneCount(workspace);
  return `${rows} row${rows > 1 ? 's' : ''}, ${totalPanes} pane${totalPanes > 1 ? 's' : ''}`;
};
</script>

<template>
  <div class="workspaces-view">
    <header class="page-header">
      <div>
        <h1>Workspaces</h1>
        <p class="subtitle">Terminal layouts for your projects</p>
      </div>
    </header>

    <div v-if="projectsStore.loading" class="loading">Loading...</div>

    <div v-else-if="allWorkspaces.length === 0" class="empty-state">
      <i class="pi pi-th-large"></i>
      <h3>No workspaces yet</h3>
      <p>
        Workspaces let you define terminal layouts with multiple panes, each
        with their own directory and startup commands.
      </p>
      <p>Add a workspace from a project's detail page.</p>
    </div>

    <div v-else class="workspaces-list">
      <div
        v-for="{ workspace, project } in allWorkspaces"
        :key="`${project.id}-${workspace.id}`"
        class="workspace-card"
      >
        <div class="card-main">
          <div class="card-info">
            <div class="card-title-row">
              <h3>{{ workspace.name }}</h3>
              <span class="project-badge">
                <i class="pi pi-folder"></i>
                {{ project.name }}
              </span>
            </div>
            <div class="card-meta">
              <span class="layout-detail">
                <i class="pi pi-th-large"></i>
                {{ getLayoutDisplay(workspace) }}
              </span>
            </div>
          </div>

          <div class="card-preview">
            <div class="layout-preview">
              <div
                v-for="(cols, row) in workspace.layout.columns"
                :key="row"
                class="layout-row"
                :style="{ flex: 1 }"
              >
                <div
                  v-for="col in cols"
                  :key="col"
                  class="layout-cell"
                  :style="{ flex: 1 }"
                >
                  <span class="cell-command">{{ getCommandDisplayText(getWorkspacePaneAt(workspace, row, col - 1), project) }}</span>
                  <span class="cell-dir">{{ getPaneDirectoryDisplay(getWorkspacePaneAt(workspace, row, col - 1)) }}</span>
                </div>
              </div>
            </div>
          </div>

          <div class="card-actions">
            <button
              class="btn btn-primary"
              @click="launchWorkspace(project.id, workspace.id)"
            >
              <i class="pi pi-play"></i>
              Launch
            </button>
            <router-link
              :to="`/projects/${project.id}`"
              class="btn btn-secondary"
            >
              <i class="pi pi-pencil"></i>
              Edit
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.workspaces-view {
  max-width: 1200px;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h1 {
  font-size: 24px;
  margin-bottom: 4px;
}

.subtitle {
  color: var(--text-secondary);
}

.empty-state {
  text-align: center;
  padding: 64px;
  background: var(--bg-secondary);
  border-radius: 12px;
}

.empty-state i {
  font-size: 48px;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.empty-state h3 {
  margin-bottom: 12px;
}

.empty-state p {
  color: var(--text-secondary);
  max-width: 400px;
  margin: 0 auto 8px;
}

/* Stacked vertical list */
.workspaces-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.workspace-card {
  background: var(--bg-card);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 16px 20px;
  transition: border-color 0.15s ease;
}

.workspace-card:hover {
  border-color: var(--border-accent);
}

.card-main {
  display: flex;
  align-items: center;
  gap: 24px;
}

.card-info {
  flex: 1;
  min-width: 0;
}

.card-title-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 6px;
}

.card-title-row h3 {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
}

.project-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  padding: 2px 8px;
  background: var(--accent-muted);
  color: var(--accent);
  border-radius: 4px;
  white-space: nowrap;
}

.project-badge i {
  font-size: 10px;
}

.card-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--text-muted);
  font-size: 12px;
}

.layout-detail {
  display: flex;
  align-items: center;
  gap: 4px;
}

.layout-detail i {
  font-size: 11px;
}

/* Layout preview */
.card-preview {
  width: 220px;
  flex-shrink: 0;
}

.layout-preview {
  height: 80px;
  display: flex;
  flex-direction: column;
  gap: 3px;
  background: var(--bg-primary);
  border-radius: 6px;
  padding: 6px;
}

.layout-row {
  display: flex;
  gap: 3px;
}

.layout-cell {
  background: var(--bg-secondary);
  border-radius: 3px;
  border: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  padding: 2px 4px;
  gap: 1px;
}

.cell-command {
  font-size: 9px;
  color: var(--text-secondary);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
  font-family: "SF Mono", monospace;
  font-weight: 500;
}

.cell-dir {
  font-size: 7px;
  color: var(--text-muted);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

/* Actions */
.card-actions {
  display: flex;
  flex-direction: column;
  gap: 6px;
  flex-shrink: 0;
}

.card-actions .btn {
  min-width: 100px;
  justify-content: center;
}

.loading {
  text-align: center;
  padding: 48px;
  color: var(--text-secondary);
}
</style>
