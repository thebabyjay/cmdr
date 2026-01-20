<script setup lang="ts">
import { computed } from "vue";
import type { Project } from "@/types";
import { useProjectsStore } from "@/stores/projects";

const props = defineProps<{
  project: Project;
}>();

const projectsStore = useProjectsStore();

const lastOpenedFormatted = computed(() => {
  if (!props.project.lastOpened) return "Never";
  const date = new Date(props.project.lastOpened);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return "Just now";
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;
  return date.toLocaleDateString();
});

const handleOpen = async () => {
  try {
    await projectsStore.openProject(props.project.id);
  } catch (e) {
    console.error("Failed to open project:", e);
  }
};
</script>

<template>
  <div class="project-card card">
    <div class="card-header">
      <div class="project-icon">
        <i class="pi pi-folder"></i>
      </div>
      <div class="project-info">
        <h3>{{ project.name }}</h3>
        <p class="project-path">{{ project.path }}</p>
      </div>
    </div>

    <p v-if="project.description" class="project-description">
      {{ project.description }}
    </p>

    <div v-if="project.tags.length > 0" class="tags">
      <span v-for="tag in project.tags" :key="tag" class="tag">
        {{ tag }}
      </span>
    </div>

    <div class="card-footer">
      <span class="last-opened">
        <i class="pi pi-clock"></i>
        {{ lastOpenedFormatted }}
      </span>

      <div class="actions">
        <router-link
          :to="`/projects/${project.id}`"
          class="btn btn-secondary btn-sm"
        >
          <i class="pi pi-cog"></i>
        </router-link>
        <button class="btn btn-primary btn-sm" @click="handleOpen">
          <i class="pi pi-play"></i>
          Open
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.project-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.card-header {
  display: flex;
  gap: 12px;
  align-items: flex-start;
}

.project-icon {
  width: 40px;
  height: 40px;
  background: rgba(0, 217, 255, 0.15);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent);
}

.project-info h3 {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 4px;
}

.project-path {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 200px;
}

.project-description {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tag {
  font-size: 11px;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  color: var(--text-secondary);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: auto;
  padding-top: 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.last-opened {
  font-size: 12px;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.actions {
  display: flex;
  gap: 8px;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 13px;
  display: flex;
  align-items: center;
  gap: 6px;
}
</style>
