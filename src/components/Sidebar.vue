<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";

const router = useRouter();
const route = useRoute();

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
      <span class="version">v0.1.0</span>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 220px;
  background: var(--bg-secondary);
  display: flex;
  flex-direction: column;
  border-right: 1px solid rgba(255, 255, 255, 0.1);
}

.logo {
  padding: 24px 20px;
  display: flex;
  align-items: center;
  gap: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo-icon {
  font-size: 24px;
}

.logo-text {
  font-size: 20px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.nav {
  flex: 1;
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-radius: 8px;
  color: var(--text-secondary);
  text-decoration: none;
  transition: all 0.2s;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-primary);
}

.nav-item.active {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent);
}

.nav-item i {
  font-size: 18px;
}

.sidebar-footer {
  padding: 16px 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.version {
  color: var(--text-secondary);
  font-size: 12px;
}
</style>
