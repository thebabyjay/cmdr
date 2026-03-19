<script setup lang="ts">
import { useRoute } from "vue-router";
import { computed } from "vue";
import { useSettingsStore } from "@/stores/settings";

const route = useRoute();
const settingsStore = useSettingsStore();

const isDark = computed(() => settingsStore.settings.theme !== "light");

const toggleTheme = async () => {
  const newTheme = isDark.value ? "light" : "dark";
  const newSettings = { ...settingsStore.settings, theme: newTheme };
  try {
    await settingsStore.saveSettings(newSettings);
  } catch (e) {
    console.error("Failed to save theme:", e);
  }
};

const menuItems = [
  { name: "Dashboard", path: "/", icon: "pi-home" },
  { name: "Projects", path: "/projects", icon: "pi-folder" },
  { name: "Workspaces", path: "/workspaces", icon: "pi-th-large" },
  { name: "Settings", path: "/settings", icon: "pi-cog" },
];

const isActive = (path: string) => {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
};
</script>

<template>
  <aside class="sidebar">
    <div class="logo">
      <span class="logo-icon">⚡</span>
      <span class="logo-text">cmdr</span>
    </div>

    <nav class="nav">
      <router-link
        v-for="item in menuItems"
        :key="item.path"
        :to="item.path"
        :class="['nav-item', { active: isActive(item.path) }]"
      >
        <i :class="['pi', item.icon]"></i>
        <span>{{ item.name }}</span>
      </router-link>
    </nav>

    <div class="sidebar-footer">
      <div class="footer-left">
        <span class="version">v0.1.0</span>
      </div>
      <button class="theme-toggle" @click="toggleTheme" :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'">
        <i :class="['pi', isDark ? 'pi-sun' : 'pi-moon']"></i>
      </button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 240px;
  background: var(--bg-secondary);
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border-primary);
}

.logo {
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 10px;
  border-bottom: 1px solid var(--border-primary);
}

.logo-icon {
  font-size: 20px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--gradient-accent);
  border-radius: 6px;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  letter-spacing: -0.5px;
  background: var(--gradient-accent);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.nav {
  flex: 1;
  padding: 12px 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.15s ease;
  border: 1px solid transparent;
}

.nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--accent-muted);
  color: var(--accent);
  border-color: var(--accent-subtle);
}

.nav-item i {
  font-size: 16px;
  width: 20px;
  text-align: center;
}

.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border-primary);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.version {
  color: var(--text-muted);
  font-size: 11px;
  font-family: "SF Mono", monospace;
}

.theme-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  border: 1px solid var(--border-primary);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.theme-toggle:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-muted);
}

.theme-toggle i {
  font-size: 14px;
}
</style>
