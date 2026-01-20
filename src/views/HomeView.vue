<script setup lang="ts">
import { onMounted } from "vue";
import { useProjectsStore } from "@/stores/projects";
import ProjectCard from "@/components/ProjectCard.vue";
import QuickActions from "@/components/QuickActions.vue";

const projectsStore = useProjectsStore();

onMounted(() => {
  projectsStore.loadProjects();
});
</script>

<template>
  <div class="home">
    <header class="home-header">
      <h1>Welcome back</h1>
      <p class="subtitle">What would you like to work on today?</p>
    </header>

    <section class="quick-actions-section">
      <h2>Quick Actions</h2>
      <QuickActions />
    </section>

    <section class="recent-section">
      <div class="section-header">
        <h2>Recent Projects</h2>
        <router-link to="/projects" class="view-all">View all</router-link>
      </div>

      <div v-if="projectsStore.loading" class="loading">Loading projects...</div>

      <div
        v-else-if="projectsStore.recentProjects.length === 0"
        class="empty-state"
      >
        <i class="pi pi-folder-open"></i>
        <p>No recent projects</p>
        <router-link to="/projects" class="btn btn-primary">
          Add your first project
        </router-link>
      </div>

      <div v-else class="projects-grid">
        <ProjectCard
          v-for="project in projectsStore.recentProjects"
          :key="project.id"
          :project="project"
        />
      </div>
    </section>
  </div>
</template>

<style scoped>
.home {
  max-width: 1200px;
}

.home-header {
  margin-bottom: 32px;
}

.home-header h1 {
  font-size: 28px;
  font-weight: 600;
  margin-bottom: 8px;
}

.subtitle {
  color: var(--text-secondary);
  font-size: 16px;
}

section {
  margin-bottom: 40px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

h2 {
  font-size: 18px;
  font-weight: 600;
}

.view-all {
  color: var(--accent);
  text-decoration: none;
  font-size: 14px;
}

.view-all:hover {
  text-decoration: underline;
}

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.empty-state {
  text-align: center;
  padding: 48px;
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
  color: var(--text-secondary);
  padding: 24px;
  text-align: center;
}

.quick-actions-section h2 {
  margin-bottom: 16px;
}
</style>
