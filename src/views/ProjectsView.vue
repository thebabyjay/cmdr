<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useProjectsStore } from "@/stores/projects";
import ProjectCard from "@/components/ProjectCard.vue";
import AddProjectModal from "@/components/AddProjectModal.vue";

const route = useRoute();
const projectsStore = useProjectsStore();
const showAddModal = ref(false);
const searchQuery = ref("");

onMounted(() => {
  projectsStore.loadProjects();
  if (route.query.new === "true") {
    showAddModal.value = true;
  }
});

const filteredProjects = () => {
  if (!searchQuery.value) return projectsStore.projects;
  const query = searchQuery.value.toLowerCase();
  return projectsStore.projects.filter(
    (p) =>
      p.name.toLowerCase().includes(query) ||
      p.path.toLowerCase().includes(query) ||
      p.tags.some((t) => t.toLowerCase().includes(query))
  );
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

    <div v-else-if="filteredProjects().length === 0" class="empty-state">
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

    <div v-else class="projects-grid">
      <ProjectCard
        v-for="project in filteredProjects()"
        :key="project.id"
        :project="project"
      />
    </div>

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

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
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
</style>
