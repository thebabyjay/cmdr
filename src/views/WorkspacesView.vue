<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useProjectsStore } from "@/stores/projects";
import { invoke } from "@tauri-apps/api/core";
import type { Workspace, Project } from "@/types";

const projectsStore = useProjectsStore();

onMounted(() => {
  console.log("[WorkspacesView] Mounted, loading projects");
  projectsStore.loadProjects();
});

const allWorkspaces = computed(() => {
  const workspaces: Array<{ workspace: Workspace; project: Project }> = [];
  for (const project of projectsStore.projects) {
    for (const workspace of project.workspaces) {
      workspaces.push({ workspace, project });
    }
  }
  console.log("[WorkspacesView] Computed all workspaces:", workspaces.length);
  return workspaces;
});

const launchWorkspace = async (projectId: string, workspaceId: string) => {
  console.log("[WorkspacesView] Launching workspace:", workspaceId, "for project:", projectId);
  try {
    await invoke("launch_workspace", {
      projectId,
      workspaceId,
    });
    console.log("[WorkspacesView] Workspace launched successfully");
  } catch (e) {
    console.error("[WorkspacesView] Failed to launch workspace:", e);
    alert("Failed to launch workspace: " + e);
  }
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

    <div v-else class="workspaces-grid">
      <div
        v-for="{ workspace, project } in allWorkspaces"
        :key="`${project.id}-${workspace.id}`"
        class="workspace-card card"
      >
        <div class="workspace-header">
          <h3>{{ workspace.name }}</h3>
          <span class="project-badge">{{ project.name }}</span>
        </div>

        <div class="workspace-preview">
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
              ></div>
            </div>
          </div>
        </div>

        <div class="workspace-info">
          <span>
            <i class="pi pi-window-maximize"></i>
            {{ workspace.panes.length }} panes
          </span>
        </div>

        <button
          class="btn btn-primary"
          @click="launchWorkspace(project.id, workspace.id)"
        >
          <i class="pi pi-play"></i>
          Launch
        </button>
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

.workspaces-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.workspace-card {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.workspace-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.workspace-header h3 {
  font-size: 16px;
}

.project-badge {
  font-size: 11px;
  padding: 4px 8px;
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent);
  border-radius: 4px;
}

.workspace-preview {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 8px;
  height: 100px;
}

.layout-preview {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.layout-row {
  display: flex;
  gap: 4px;
}

.layout-cell {
  background: var(--bg-secondary);
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.workspace-info {
  color: var(--text-secondary);
  font-size: 13px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.workspace-card .btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.loading {
  text-align: center;
  padding: 48px;
  color: var(--text-secondary);
}
</style>
