<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useProjectsStore } from "@/stores/projects";
import { invoke } from "@tauri-apps/api/core";
import QuickActions from "@/components/QuickActions.vue";

const projectsStore = useProjectsStore();

onMounted(() => {
  projectsStore.loadProjects();
});

const recentProjects = computed(() => {
  return [...projectsStore.projects]
    .sort((a, b) => (b.lastOpened || "").localeCompare(a.lastOpened || ""))
    .slice(0, 5);
});

// Get all workspaces flattened with project info for "recent" display
const recentWorkspaces = computed(() => {
  const workspaces: { projectId: string; projectName: string; projectPath: string; workspace: any }[] = [];

  projectsStore.projects.forEach(project => {
    project.workspaces?.forEach(ws => {
      workspaces.push({
        projectId: project.id,
        projectName: project.name,
        projectPath: project.path,
        workspace: ws
      });
    });
  });

  return workspaces.slice(0, 6);
});

const launchWorkspace = async (projectId: string, workspaceId: string) => {
  try {
    await invoke("launch_workspace", { projectId, workspaceId });
  } catch (e) {
    console.error("[Dashboard] Failed to launch workspace:", e);
    alert("Failed to launch workspace: " + e);
  }
};

const getPaneCount = (workspace: any) => {
  return workspace.layout?.columns?.reduce((sum: number, cols: number) => sum + cols, 0) || 0;
};

const getLayoutDisplay = (workspace: any) => {
  if (!workspace.layout?.columns) return "";
  return workspace.layout.columns.join(" × ");
};

const lastOpenedFormatted = (dateStr?: string) => {
  if (!dateStr) return "Never";
  const date = new Date(dateStr);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  if (minutes < 1) return "Just now";
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  if (days < 7) return `${days}d ago`;
  return date.toLocaleDateString();
};
</script>

<template>
  <div class="home">
    <header class="home-header">
      <h1>Dashboard</h1>
      <p class="subtitle">Launch workspaces and manage your projects</p>
    </header>

    <section class="quick-actions-section">
      <h2>Quick Actions</h2>
      <QuickActions />
    </section>

    <!-- Recent Workspaces - Quick Launch -->
    <section class="workspaces-section" v-if="recentWorkspaces.length > 0">
      <div class="section-header">
        <h2>Quick Launch</h2>
        <router-link to="/workspaces" class="view-all">All workspaces</router-link>
      </div>

      <div class="workspace-list">
        <div
          v-for="item in recentWorkspaces"
          :key="`${item.projectId}-${item.workspace.id}`"
          class="workspace-row"
        >
          <div class="workspace-details">
            <span class="workspace-name">{{ item.workspace.name }}</span>
            <span class="workspace-project-badge">{{ item.projectName }}</span>
          </div>
          <div class="workspace-meta">
            <span class="pane-count">{{ getPaneCount(item.workspace) }} panes</span>
            <span class="layout-info">{{ getLayoutDisplay(item.workspace) }}</span>
          </div>
          <button
            class="btn btn-primary btn-sm"
            @click="launchWorkspace(item.projectId, item.workspace.id)"
          >
            <i class="pi pi-play"></i>
            Launch
          </button>
        </div>
      </div>
    </section>

    <!-- Recent Projects -->
    <section class="projects-section">
      <div class="section-header">
        <h2>Recent Projects</h2>
        <router-link to="/projects" class="view-all">All projects</router-link>
      </div>

      <div v-if="projectsStore.loading" class="loading">Loading...</div>

      <div v-else-if="projectsStore.projects.length === 0" class="empty-state">
        <i class="pi pi-folder-open"></i>
        <p>No projects yet</p>
        <router-link to="/projects?new=true" class="btn btn-primary">
          Add your first project
        </router-link>
      </div>

      <div v-else class="project-list">
        <router-link
          v-for="project in recentProjects"
          :key="project.id"
          :to="`/projects/${project.id}`"
          class="project-row"
        >
          <div class="project-info">
            <span class="project-name">{{ project.name }}</span>
            <span class="project-path">{{ project.path }}</span>
          </div>
          <span class="project-opened">{{ lastOpenedFormatted(project.lastOpened) }}</span>
        </router-link>
      </div>
    </section>
  </div>
</template>

<style scoped>
.home {
  max-width: 1100px;
}

.home-header {
  margin-bottom: 24px;
}

.home-header h1 {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 4px;
  color: var(--text-primary);
}

.subtitle {
  color: var(--text-secondary);
  font-size: 13px;
}

section {
  margin-bottom: 28px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

h2 {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-muted);
}

.view-all {
  color: var(--accent);
  text-decoration: none;
  font-size: 12px;
  font-weight: 500;
}

.view-all:hover {
  text-decoration: underline;
}

.quick-actions-section h2 {
  margin-bottom: 10px;
}

/* Workspace List - Quick Launch */
.workspace-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  background: var(--border-subtle);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  overflow: hidden;
}

.workspace-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 10px 14px;
  background: var(--bg-card);
  transition: background 0.1s ease;
}

.workspace-row:hover {
  background: var(--bg-secondary);
}

.workspace-details {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.workspace-name {
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary);
}

.workspace-project-badge {
  font-size: 11px;
  padding: 2px 8px;
  background: var(--accent-muted);
  color: var(--accent);
  border-radius: 4px;
  white-space: nowrap;
}

.workspace-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.layout-info {
  font-family: "SF Mono", monospace;
}

/* Project List */
.project-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  background: var(--border-subtle);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  overflow: hidden;
}

.project-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 10px 14px;
  background: var(--bg-card);
  transition: background 0.1s ease;
  text-decoration: none;
}

.project-row:hover {
  background: var(--bg-secondary);
}

.project-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.project-name {
  font-weight: 500;
  font-size: 14px;
  color: var(--text-primary);
}

.project-row:hover .project-name {
  color: var(--accent);
}

.project-path {
  font-size: 11px;
  color: var(--text-muted);
  font-family: "SF Mono", monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.project-opened {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
  white-space: nowrap;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 40px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
}

.empty-state i {
  font-size: 36px;
  color: var(--text-muted);
  margin-bottom: 12px;
}

.empty-state p {
  color: var(--text-secondary);
  margin-bottom: 16px;
  font-size: 13px;
}

.loading {
  color: var(--text-secondary);
  padding: 20px;
  text-align: center;
  font-size: 13px;
}
</style>
