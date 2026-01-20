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
    colorVar: "--accent",
    action: () => router.push("/projects?new=true"),
  },
  {
    id: "import-folder",
    label: "Import Folder",
    icon: "pi-folder-open",
    colorVar: "--warning",
    action: openFolderAction,
  },
  {
    id: "terminal",
    label: "Open Terminal",
    icon: "pi-desktop",
    colorVar: "--purple",
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
      :class="action.id"
      @click="action.action"
    >
      <div class="action-icon">
        <i :class="['pi', action.icon]"></i>
      </div>
      <span>{{ action.label }}</span>
    </button>
  </div>
</template>

<style scoped>
.quick-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.quick-action-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  color: var(--text-primary);
  font-size: 13px;
}

.quick-action-btn:hover {
  border-color: var(--border-accent);
  background: var(--bg-tertiary);
}

.action-icon {
  width: 28px;
  height: 28px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-icon i {
  font-size: 14px;
}

.quick-action-btn span {
  font-weight: 500;
}

/* Color variants */
.new-project .action-icon {
  background: var(--accent-muted);
  color: var(--accent);
}

.import-folder .action-icon {
  background: var(--warning-muted);
  color: var(--warning);
}

.terminal .action-icon {
  background: var(--purple-muted);
  color: var(--purple);
}
</style>
