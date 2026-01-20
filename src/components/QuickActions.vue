<script setup lang="ts">
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useProjectsStore } from "@/stores/projects";

const router = useRouter();
const projectsStore = useProjectsStore();

const openFolderAction = async () => {
  console.log("[QuickActions] Opening folder picker");
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      console.log("[QuickActions] Selected folder:", selected);
      // Create a new project from the selected folder
      const folderPath = selected as string;
      const folderName = folderPath.split("/").pop() || "New Project";

      await projectsStore.addProject({
        name: folderName,
        path: folderPath,
        description: "",
        tags: [],
        environments: {},
        workspaces: [],
        commands: [],
      });
      console.log("[QuickActions] Project created from folder");
      router.push("/projects");
    }
  } catch (e) {
    console.error("[QuickActions] Failed to open folder:", e);
    alert("Failed to open folder: " + e);
  }
};

const openTerminalAction = async () => {
  console.log("[QuickActions] Opening terminal");
  try {
    await invoke("open_terminal", {});
    console.log("[QuickActions] Terminal opened successfully");
  } catch (e) {
    console.error("[QuickActions] Failed to open terminal:", e);
    alert("Failed to open terminal: " + e);
  }
};

const actions = [
  {
    id: "new-project",
    label: "New Project",
    icon: "pi-plus",
    color: "#00d9ff",
    action: () => router.push("/projects?new=true"),
  },
  {
    id: "open-folder",
    label: "Open Folder",
    icon: "pi-folder-open",
    color: "#ffc107",
    action: openFolderAction,
  },
  {
    id: "new-workspace",
    label: "New Workspace",
    icon: "pi-th-large",
    color: "#00c853",
    action: () => router.push("/workspaces?new=true"),
  },
  {
    id: "terminal",
    label: "Open Terminal",
    icon: "pi-desktop",
    color: "#ff5252",
    action: openTerminalAction,
  },
];
</script>

<template>
  <div class="quick-actions">
    <button
      v-for="action in actions"
      :key="action.id"
      class="quick-action-btn"
      @click="action.action"
    >
      <div class="action-icon" :style="{ background: action.color + '20' }">
        <i :class="['pi', action.icon]" :style="{ color: action.color }"></i>
      </div>
      <span>{{ action.label }}</span>
    </button>
  </div>
</template>

<style scoped>
.quick-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.quick-action-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  background: var(--bg-secondary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-primary);
}

.quick-action-btn:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.action-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-icon i {
  font-size: 16px;
}

.quick-action-btn span {
  font-weight: 500;
}
</style>
