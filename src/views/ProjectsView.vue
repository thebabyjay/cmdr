<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useProjectsStore } from "@/stores/projects";
import { invoke } from "@tauri-apps/api/core";
import AddProjectModal from "@/components/AddProjectModal.vue";

const route = useRoute();
const projectsStore = useProjectsStore();
const showAddModal = ref(false);
const searchQuery = ref("");
const expandedRows = ref<Set<string>>(new Set());

// Sorting
type SortColumn = "name" | "path" | "workspaces" | "lastOpened";
const sortColumn = ref<SortColumn>("name");
const sortDirection = ref<"asc" | "desc">("asc");

onMounted(() => {
  projectsStore.loadProjects();
  if (route.query.new === "true") {
    showAddModal.value = true;
  }
});

const toggleSort = (column: SortColumn) => {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === "asc" ? "desc" : "asc";
  } else {
    sortColumn.value = column;
    sortDirection.value = "asc";
  }
};

const sortIcon = (column: SortColumn) => {
  if (sortColumn.value !== column) return "pi pi-sort-alt";
  return sortDirection.value === "asc" ? "pi pi-sort-amount-up" : "pi pi-sort-amount-down";
};

const toggleExpand = (projectId: string) => {
  if (expandedRows.value.has(projectId)) {
    expandedRows.value.delete(projectId);
  } else {
    expandedRows.value.add(projectId);
  }
};

const filteredProjects = computed(() => {
  let projects = [...projectsStore.projects];

  // Filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    projects = projects.filter(
      (p) =>
        p.name.toLowerCase().includes(query) ||
        p.path.toLowerCase().includes(query) ||
        p.tags.some((t) => t.toLowerCase().includes(query))
    );
  }

  // Sort
  projects.sort((a, b) => {
    let cmp = 0;
    switch (sortColumn.value) {
      case "name":
        cmp = a.name.localeCompare(b.name);
        break;
      case "path":
        cmp = a.path.localeCompare(b.path);
        break;
      case "workspaces":
        cmp = (a.workspaces?.length || 0) - (b.workspaces?.length || 0);
        break;
      case "lastOpened":
        cmp = (a.lastOpened || "").localeCompare(b.lastOpened || "");
        break;
    }
    return sortDirection.value === "asc" ? cmp : -cmp;
  });

  return projects;
});

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

const launchWorkspace = async (projectId: string, workspaceId: string) => {
  try {
    await invoke("launch_workspace", { projectId, workspaceId });
  } catch (e) {
    console.error("Failed to launch workspace:", e);
    alert("Failed to launch workspace: " + e);
  }
};

const openProject = async (projectId: string) => {
  try {
    await projectsStore.openProject(projectId);
  } catch (e) {
    console.error("Failed to open project:", e);
  }
};
</script>

<template>
  <div class="projects-view">
    <header class="page-header">
      <div>
        <h1>Projects</h1>
        <p class="subtitle">Manage your development projects</p>
      </div>
      <button class="btn btn-primary" @click="showAddModal = true">
        <i class="pi pi-plus"></i>
        Add Project
      </button>
    </header>

    <div class="search-bar">
      <i class="pi pi-search"></i>
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search projects..."
      />
    </div>

    <div v-if="projectsStore.loading" class="loading">Loading...</div>

    <div v-else-if="filteredProjects.length === 0" class="empty-state">
      <i class="pi pi-folder-open"></i>
      <p v-if="searchQuery">No projects match your search</p>
      <p v-else>No projects yet</p>
      <button
        v-if="!searchQuery"
        class="btn btn-primary"
        @click="showAddModal = true"
      >
        Add your first project
      </button>
    </div>

    <table v-else class="data-table">
      <thead>
        <tr>
          <th style="width: 32px"></th>
          <th @click="toggleSort('name')">
            Name
            <i :class="['sort-icon', sortIcon('name'), { active: sortColumn === 'name' }]"></i>
          </th>
          <th @click="toggleSort('path')">
            Path
            <i :class="['sort-icon', sortIcon('path'), { active: sortColumn === 'path' }]"></i>
          </th>
          <th>Tags</th>
          <th @click="toggleSort('workspaces')">
            Workspaces
            <i :class="['sort-icon', sortIcon('workspaces'), { active: sortColumn === 'workspaces' }]"></i>
          </th>
          <th @click="toggleSort('lastOpened')">
            Last Opened
            <i :class="['sort-icon', sortIcon('lastOpened'), { active: sortColumn === 'lastOpened' }]"></i>
          </th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <template v-for="project in filteredProjects" :key="project.id">
          <tr>
            <td>
              <button
                class="expand-btn"
                @click="toggleExpand(project.id)"
                :disabled="!project.workspaces?.length"
                :title="project.workspaces?.length ? 'Show workspaces' : 'No workspaces'"
              >
                <i :class="[
                  'pi',
                  expandedRows.has(project.id) ? 'pi-chevron-down' : 'pi-chevron-right',
                ]" :style="{ opacity: project.workspaces?.length ? 1 : 0.3 }"></i>
              </button>
            </td>
            <td>
              <router-link :to="`/projects/${project.id}`" class="project-name-link">
                {{ project.name }}
              </router-link>
            </td>
            <td>
              <span class="project-path-cell">{{ project.path }}</span>
            </td>
            <td>
              <div class="tags-cell" v-if="project.tags.length">
                <span v-for="tag in project.tags" :key="tag" class="tag">{{ tag }}</span>
              </div>
              <span v-else class="text-muted">—</span>
            </td>
            <td>
              <span :class="project.workspaces?.length ? '' : 'text-muted'">
                {{ project.workspaces?.length || 0 }}
              </span>
            </td>
            <td>
              <span class="last-opened-cell">{{ lastOpenedFormatted(project.lastOpened) }}</span>
            </td>
            <td>
              <div class="action-group">
                <router-link :to="`/projects/${project.id}`" class="btn btn-secondary btn-sm">
                  <i class="pi pi-pencil"></i> Edit
                </router-link>
                <button class="btn btn-secondary btn-sm" @click="openProject(project.id)">
                  <i class="pi pi-folder-open"></i> Open
                </button>
              </div>
            </td>
          </tr>
          <tr v-if="expandedRows.has(project.id) && project.workspaces?.length" class="expand-row">
            <td :colspan="7">
              <div class="expand-content">
                <div class="expand-label">Workspaces</div>
                <div class="workspace-chips">
                  <button
                    v-for="ws in project.workspaces"
                    :key="ws.id"
                    class="workspace-launch-chip"
                    @click="launchWorkspace(project.id, ws.id)"
                  >
                    <i class="pi pi-play"></i>
                    {{ ws.name }}
                    <span class="chip-panes">{{ ws.panes.length }} panes</span>
                  </button>
                </div>
              </div>
            </td>
          </tr>
        </template>
      </tbody>
    </table>

    <AddProjectModal v-model:visible="showAddModal" />
  </div>
</template>

<style scoped>
.projects-view {
  max-width: 1200px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
}

.page-header h1 {
  font-size: 24px;
  margin-bottom: 4px;
}

.subtitle {
  color: var(--text-secondary);
}

.page-header .btn {
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border-radius: 10px;
  margin-bottom: 24px;
}

.search-bar i {
  color: var(--text-secondary);
}

.search-bar input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-size: 15px;
}

.search-bar input::placeholder {
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

.empty-state p {
  color: var(--text-secondary);
  margin-bottom: 24px;
}

.loading {
  text-align: center;
  padding: 48px;
  color: var(--text-secondary);
}

/* Table-specific */
.expand-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.15s ease;
}

.expand-btn:hover:not(:disabled) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.expand-btn:disabled {
  cursor: default;
}

.project-name-link {
  color: var(--text-primary);
  text-decoration: none;
  font-weight: 500;
}

.project-name-link:hover {
  color: var(--accent);
}

.project-path-cell {
  font-family: "SF Mono", "Fira Code", monospace;
  font-size: 12px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: block;
  max-width: 280px;
}

.tags-cell {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.last-opened-cell {
  color: var(--text-secondary);
  font-size: 12px;
  white-space: nowrap;
}

/* Expanded row */
.expand-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-muted);
  margin-bottom: 8px;
}

.workspace-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.workspace-launch-chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--accent-muted);
  color: var(--accent);
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.workspace-launch-chip:hover {
  background: var(--accent);
  color: var(--text-inverse);
}

.workspace-launch-chip i {
  font-size: 11px;
}

.chip-panes {
  font-size: 11px;
  opacity: 0.7;
  font-weight: 400;
}
</style>
