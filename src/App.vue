<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from "vue";
import { RouterView } from "vue-router";
import Sidebar from "./components/Sidebar.vue";
import DebugPanel from "./components/DebugPanel.vue";
import { useSettingsStore } from "./stores/settings";

const settingsStore = useSettingsStore();
const showDebugPanel = ref(false);

const currentTheme = computed(() => settingsStore.settings.theme || "dark");

// Apply theme to document
function applyTheme(theme: string) {
  document.documentElement.setAttribute("data-theme", theme);
}

// Watch for theme changes
watch(currentTheme, (newTheme) => {
  applyTheme(newTheme);
}, { immediate: true });

function handleKeydown(e: KeyboardEvent) {
  // Ctrl+Shift+D to toggle debug panel
  if (e.ctrlKey && e.shiftKey && e.key === "D") {
    e.preventDefault();
    showDebugPanel.value = !showDebugPanel.value;
  }
}

onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
  settingsStore.loadSettings();
  applyTheme(currentTheme.value);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
});
</script>

<template>
  <div class="app-layout">
    <Sidebar />
    <main class="main-content">
      <RouterView />
    </main>
    <DebugPanel v-if="showDebugPanel" @close="showDebugPanel = false" />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px 32px;
  background: var(--bg-primary);
}
</style>
