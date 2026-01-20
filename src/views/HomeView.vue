<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useProjectsStore } from "@/stores/projects";
import { invoke } from "@tauri-apps/api/core";
import QuickActions from "@/components/QuickActions.vue";

const projectsStore = useProjectsStore();

onMounted(() => {
  projectsStore.loadProjects();
});

// Get projects that have workspaces, sorted by last opened
const projectsWithWorkspaces = computed(() => {
  return projectsStore.projects
    .filter(p => p.workspaces && p.workspaces.length > 0)
    .sort((a, b) => {
      const aTime = a.lastOpened ? new Date(a.lastOpened).getTime() : 0;
      const bTime = b.lastOpened ? new Date(b.lastOpened).getTime() : 0;
      return bTime - aTime;
    });
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

  return workspaces.slice(0, 6); // Show up to 6 recent workspaces
});

const launchWorkspace = async (projectId: string, workspaceId: string) => {
  console.log("[Dashboard] Launching workspace:", workspaceId, "for project:", projectId);
  try {
    await invoke("launch_workspace", { projectId, workspaceId });
    console.log("[Dashboard] Workspace launched successfully");
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

      <div class="workspace-grid">
        <div
          v-for="item in recentWorkspaces"
          :key="`${item.projectId}-${item.workspace.id}`"
          class="workspace-card"
        >
          <div class="workspace-info">
            <div class="workspace-name">{{ item.workspace.name }}</div>
            <div class="workspace-project">
              <i class="pi pi-folder"></i>
              {{ item.projectName }}
            </div>
            <div class="workspace-meta">
              <span class="pane-count">{{ getPaneCount(item.workspace) }} panes</span>
              <span class="layout-info">{{ getLayoutDisplay(item.workspace) }}</span>
            </div>
          </div>
          <button
            class="btn btn-primary btn-launch"
            @click="launchWorkspace(item.projectId, item.workspace.id)"
          >
            <i class="pi pi-play"></i>
            Launch
          </button>
        </div>
      </div>
    </section>

    <!-- Projects with Workspaces -->
    <section class="projects-section">
      <div class="section-header">
        <h2>Projects</h2>
        <router-link to="/projects" class="view-all">Manage projects</router-link>
      </div>

      <div v-if="projectsStore.loading" class="loading">Loading...</div>

      <div v-else-if="projectsStore.projects.length === 0" class="empty-state">
        <i class="pi pi-folder-open"></i>
        <p>No projects yet</p>
        <router-link to="/projects" class="btn btn-primary">
          Add your first project
        </router-link>
      </div>

      <div v-else class="project-list">
        <div
          v-for="project in projectsStore.projects"
          :key="project.id"
          class="project-row"
        >
          <div class="project-info">
            <router-link :to="`/projects/${project.id}`" class="project-name">
              {{ project.name }}
            </router-link>
            <div class="project-path">{{ project.path }}</div>
          </div>

          <div class="project-workspaces" v-if="project.workspaces?.length">
            <button
              v-for="ws in project.workspaces.slice(0, 3)"
              :key="ws.id"
              class="workspace-chip"
              @click="launchWorkspace(project.id, ws.id)"
              :title="`Launch ${ws.name}`"
            >
              <i class="pi pi-play"></i>
              {{ ws.name }}
            </button>
            <span v-if="project.workspaces.length > 3" class="more-workspaces">
              +{{ project.workspaces.length - 3 }} more
            </span>
          </div>
          <div v-else class="no-workspaces">
            <router-link :to="`/projects/${project.id}?tab=workspaces`" class="add-workspace-link">
              <i class="pi pi-plus"></i>
              Add workspace
            </router-link>
          </div>

          <router-link :to="`/projects/${project.id}`" class="project-settings-btn" title="Project settings">
            <i class="pi pi-cog"></i>
          </router-link>
        </div>
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

/* Workspace Grid - Quick Launch */
.workspace-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 10px;
}

.workspace-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  background: var(--bg-card);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  transition: border-color 0.15s ease;
}

.workspace-card:hover {
  border-color: var(--border-accent);
}

.workspace-info {
  flex: 1;
  min-width: 0;
}

.workspace-name {
  font-weight: 600;
  font-size: 14px;
  margin-bottom: 2px;
  color: var(--text-primary);
}

.workspace-project {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.workspace-project i {
  font-size: 11px;
  color: var(--text-muted);
}

.workspace-meta {
  display: flex;
  gap: 8px;
  font-size: 11px;
  color: var(--text-muted);
}

.layout-info {
  font-family: "SF Mono", monospace;
}

.btn-launch {
  padding: 6px 12px;
  font-size: 12px;
  flex-shrink: 0;
  margin-left: 12px;
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
  padding: 12px 14px;
  background: var(--bg-card);
  transition: background 0.1s ease;
}

.project-row:hover {
  background: var(--bg-secondary);
}

.project-info {
  flex: 1;
  min-width: 0;
}

.project-name {
  font-weight: 500;
  font-size: 14px;
  color: var(--text-primary);
  text-decoration: none;
  display: block;
  margin-bottom: 2px;
}

.project-name:hover {
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

.project-workspaces {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.workspace-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  background: var(--accent-muted);
  color: var(--accent);
  border: 1px solid transparent;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.workspace-chip:hover {
  background: var(--accent);
  color: var(--text-inverse);
}

.workspace-chip i {
  font-size: 10px;
}

.more-workspaces {
  font-size: 11px;
  color: var(--text-muted);
  padding-left: 4px;
}

.no-workspaces {
  flex-shrink: 0;
}

.add-workspace-link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-muted);
  text-decoration: none;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.15s ease;
}

.add-workspace-link:hover {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
}

.add-workspace-link i {
  font-size: 10px;
}

.project-settings-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  color: var(--text-muted);
  border-radius: 4px;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.project-settings-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
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
