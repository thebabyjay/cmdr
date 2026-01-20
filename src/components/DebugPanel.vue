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
  width: 400px;
  height: 100vh;
  background: #1a1a2e;
  border-left: 1px solid #333;
  display: flex;
  flex-direction: column;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 12px;
}

.debug-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #16213e;
  border-bottom: 1px solid #333;
}

.debug-header h3 {
  margin: 0;
  font-size: 14px;
  color: #00d9ff;
}

.close-btn {
  background: none;
  border: none;
  color: #888;
  font-size: 20px;
  cursor: pointer;
  padding: 0 4px;
  line-height: 1;
}

.close-btn:hover {
  color: #fff;
}

.debug-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.debug-section {
  margin-bottom: 20px;
}

.debug-section h4 {
  margin: 0 0 8px 0;
  font-size: 12px;
  color: #888;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.json-display {
  background: #0f0f1a;
  border: 1px solid #333;
  border-radius: 4px;
  padding: 12px;
  margin: 0;
  overflow-x: auto;
  color: #a8ff60;
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 400px;
  overflow-y: auto;
}

.debug-footer {
  padding: 8px 16px;
  background: #16213e;
  border-top: 1px solid #333;
  text-align: center;
}

.hint {
  color: #666;
  font-size: 11px;
}

kbd {
  background: #333;
  border: 1px solid #444;
  border-radius: 3px;
  padding: 1px 5px;
  font-size: 10px;
  color: #aaa;
}
</style>
