<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "@/types";

const settings = ref<AppSettings>({
  defaultTerminal: "iterm2",
  theme: "dark",
  defaultProjectsPath: "",
  terminalBehavior: "new_window",
});

const saving = ref(false);
const saved = ref(false);

onMounted(async () => {
  console.log("[SettingsView] Loading settings");
  try {
    settings.value = await invoke<AppSettings>("get_settings");
    console.log("[SettingsView] Settings loaded:", settings.value);
  } catch (e) {
    console.error("[SettingsView] Failed to load settings:", e);
  }
});

const saveSettings = async () => {
  console.log("[SettingsView] Saving settings:", settings.value);
  saving.value = true;
  saved.value = false;
  try {
    await invoke("save_settings", { settings: settings.value });
    console.log("[SettingsView] Settings saved successfully");
    saved.value = true;
    setTimeout(() => {
      saved.value = false;
    }, 2000);
  } catch (e) {
    console.error("[SettingsView] Failed to save settings:", e);
  } finally {
    saving.value = false;
  }
};
</script>

<template>
  <div class="settings-view">
    <header class="page-header">
      <h1>Settings</h1>
      <p class="subtitle">Configure cmdr preferences</p>
    </header>

    <div class="settings-section card">
      <h2>General</h2>

      <div class="setting-item">
        <div class="setting-info">
          <label>Default Terminal</label>
          <p>Terminal application for launching workspaces</p>
        </div>
        <select v-model="settings.defaultTerminal">
          <option value="iterm2">iTerm2</option>
          <option value="terminal">Terminal.app</option>
        </select>
      </div>

      <div class="setting-item">
        <div class="setting-info">
          <label>Terminal Behavior</label>
          <p>How to handle new terminal sessions</p>
        </div>
        <select v-model="settings.terminalBehavior">
          <option value="new_window">Always open new window</option>
          <option value="use_existing">Use existing window if available</option>
        </select>
      </div>

      <div class="setting-item">
        <div class="setting-info">
          <label>Theme</label>
          <p>Application color scheme</p>
        </div>
        <select v-model="settings.theme">
          <option value="dark">Dark</option>
          <option value="light">Light</option>
        </select>
      </div>

      <div class="setting-item">
        <div class="setting-info">
          <label>Default Projects Path</label>
          <p>Default location when adding new projects</p>
        </div>
        <input
          v-model="settings.defaultProjectsPath"
          type="text"
          placeholder="/Users/you/code"
        />
      </div>
    </div>

    <div class="settings-section card">
      <h2>About</h2>

      <div class="about-info">
        <div class="about-row">
          <span>Version</span>
          <span>0.1.0</span>
        </div>
        <div class="about-row">
          <span>Config Location</span>
          <code>~/.config/cmdr</code>
        </div>
      </div>
    </div>

    <div class="settings-footer">
      <span v-if="saved" class="saved-message">
        <i class="pi pi-check"></i>
        Settings saved
      </span>
      <button class="btn btn-primary" @click="saveSettings" :disabled="saving">
        {{ saving ? "Saving..." : "Save Settings" }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  max-width: 800px;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h1 {
  font-size: 24px;
  margin-bottom: 4px;
}

.subtitle {
  color: var(--text-secondary);
}

.settings-section {
  margin-bottom: 24px;
}

.settings-section h2 {
  font-size: 16px;
  margin-bottom: 20px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info label {
  font-weight: 500;
  margin-bottom: 4px;
  display: block;
}

.setting-info p {
  font-size: 13px;
  color: var(--text-secondary);
}

.setting-item select,
.setting-item input {
  padding: 10px 14px;
  background: var(--bg-primary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: var(--text-primary);
  min-width: 200px;
}

.setting-item select:focus,
.setting-item input:focus {
  outline: none;
  border-color: var(--accent);
}

.about-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.about-row {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
}

.about-row span:first-child {
  color: var(--text-secondary);
}

.about-row code {
  font-family: monospace;
  background: var(--bg-primary);
  padding: 4px 8px;
  border-radius: 4px;
}

.settings-footer {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 16px;
}

.saved-message {
  color: var(--success);
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
}
</style>
