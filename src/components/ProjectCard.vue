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
          title="Project settings"
        >
          <i class="pi pi-cog"></i>
        </router-link>
        <button class="btn btn-secondary btn-sm" @click="handleOpen" title="Open in Finder">
          <i class="pi pi-folder-open"></i>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.project-card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  background: var(--bg-card);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 14px;
  transition: border-color 0.15s ease;
}

.project-card:hover {
  border-color: var(--border-accent);
}

.card-header {
  display: flex;
  gap: 10px;
  align-items: flex-start;
}

.project-icon {
  width: 36px;
  height: 36px;
  background: var(--accent-muted);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent);
  flex-shrink: 0;
}

.project-icon i {
  font-size: 16px;
}

.project-info {
  min-width: 0;
  flex: 1;
}

.project-info h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 2px;
  color: var(--text-primary);
}

.project-path {
  font-size: 11px;
  color: var(--text-muted);
  font-family: "SF Mono", monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.project-description {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tag {
  font-size: 11px;
  padding: 2px 6px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  color: var(--text-secondary);
  border: 1px solid var(--border-subtle);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: auto;
  padding-top: 10px;
  border-top: 1px solid var(--border-subtle);
}

.last-opened {
  font-size: 11px;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  gap: 4px;
}

.last-opened i {
  font-size: 12px;
}

.actions {
  display: flex;
  gap: 6px;
}
</style>
