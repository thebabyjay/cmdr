<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import { useProjectsStore } from "@/stores/projects";
import { useSettingsStore } from "@/stores/settings";

const route = useRoute();
const projectsStore = useProjectsStore();
const settingsStore = useSettingsStore();

defineEmits<{
  close: [];
}>();

const currentProject = computed(() => {
  if (route.name === "project-detail" && route.params.id) {
    return projectsStore.projectById(route.params.id as string);
  }
  return null;
});

const debugData = computed(() => {
  const data: Record<string, unknown> = {
    route: {
      name: route.name,
      path: route.path,
      params: route.params,
      query: route.query,
    },
    settings: settingsStore.settings,
    projectsStore: {
      loading: projectsStore.loading,
      error: projectsStore.error,
      projectCount: projectsStore.projects.length,
      recentProjectsCount: projectsStore.recentProjects.length,
    },
  };

  if (currentProject.value) {
    data.currentProject = currentProject.value;
  }

  return data;
});

const allProjects = computed(() => projectsStore.projects);

function formatJson(obj: unknown): string {
  return JSON.stringify(obj, null, 2);
}
</script>

<template>
  <div class="debug-panel">
    <div class="debug-header">
      <h3>Debug Panel</h3>
      <button class="close-btn" @click="$emit('close')" title="Close (Ctrl+Shift+D)">
        &times;
      </button>
    </div>

    <div class="debug-content">
      <section class="debug-section">
        <h4>Current Context</h4>
        <pre class="json-display">{{ formatJson(debugData) }}</pre>
      </section>

      <section class="debug-section">
        <h4>All Projects</h4>
        <pre class="json-display">{{ formatJson(allProjects) }}</pre>
      </section>
    </div>

    <div class="debug-footer">
      <span class="hint">Press <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>D</kbd> to toggle</span>
    </div>
  </div>
</template>

<style scoped>
.debug-panel {
  width: 380px;
  height: 100vh;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  font-family: "SF Mono", "Fira Code", Menlo, Monaco, monospace;
  font-size: 12px;
}

.debug-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-primary);
}

.debug-header h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--accent);
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
  line-height: 1;
}

.close-btn:hover {
  color: var(--text-primary);
}

.debug-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.debug-section {
  margin-bottom: 16px;
}

.debug-section h4 {
  margin: 0 0 8px 0;
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 600;
}

.json-display {
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  padding: 10px;
  margin: 0;
  overflow-x: auto;
  color: var(--success);
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 350px;
  overflow-y: auto;
  font-size: 11px;
  line-height: 1.5;
}

.debug-footer {
  padding: 10px 16px;
  background: var(--bg-tertiary);
  border-top: 1px solid var(--border-primary);
  text-align: center;
}

.hint {
  color: var(--text-muted);
  font-size: 11px;
}

kbd {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: 3px;
  padding: 1px 5px;
  font-size: 10px;
  color: var(--text-secondary);
}
</style>
