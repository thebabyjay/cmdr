<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { AppSettings, Command } from "@/types";

const settings = ref<AppSettings>({
  defaultTerminal: "iterm2",
  theme: "dark",
  defaultProjectsPath: "",
  terminalBehavior: "new_window",
  globalCommands: [],
});

const saving = ref(false);
const saved = ref(false);

// Global commands editing
const showAddCommand = ref(false);
const showEditCommand = ref(false);
const editingCommand = ref<Command | null>(null);
const newCommandName = ref("");
const newCommandValue = ref("");
const newCommandDescription = ref("");

onMounted(async () => {
  console.log("[SettingsView] Loading settings");
  try {
    settings.value = await invoke<AppSettings>("get_settings");
    // Ensure globalCommands is initialized
    if (!settings.value.globalCommands) {
      settings.value.globalCommands = [];
    }
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

// Global command management
const openAddCommand = () => {
  newCommandName.value = "";
  newCommandValue.value = "";
  newCommandDescription.value = "";
  showAddCommand.value = true;
};

const addGlobalCommand = async () => {
  if (!newCommandName.value.trim() || !newCommandValue.value.trim()) return;

  const newCmd: Command = {
    id: crypto.randomUUID(),
    name: newCommandName.value.trim(),
    command: newCommandValue.value.trim(),
    description: newCommandDescription.value.trim() || undefined,
  };

  settings.value.globalCommands.push(newCmd);
  showAddCommand.value = false;
  await saveSettings();
};

const openEditCommand = (cmd: Command) => {
  editingCommand.value = { ...cmd };
  showEditCommand.value = true;
};

const saveEditedCommand = async () => {
  if (!editingCommand.value) return;

  const idx = settings.value.globalCommands.findIndex(c => c.id === editingCommand.value!.id);
  if (idx !== -1) {
    settings.value.globalCommands[idx] = editingCommand.value;
  }

  showEditCommand.value = false;
  editingCommand.value = null;
  await saveSettings();
};

const deleteGlobalCommand = async (cmdId: string) => {
  settings.value.globalCommands = settings.value.globalCommands.filter(c => c.id !== cmdId);
  await saveSettings();
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
      <div class="section-header">
        <h2>Global Commands</h2>
        <button class="btn btn-secondary btn-sm" @click="openAddCommand">
          <i class="pi pi-plus"></i>
          Add Command
        </button>
      </div>
      <p class="section-description">
        Commands available in all projects. Use these for common tools like Claude Code, package managers, etc.
      </p>

      <div v-if="settings.globalCommands.length === 0" class="empty-commands">
        <p>No global commands configured</p>
        <button class="btn btn-primary btn-sm" @click="openAddCommand">
          Add Your First Command
        </button>
      </div>

      <div v-else class="commands-list">
        <div v-for="cmd in settings.globalCommands" :key="cmd.id" class="command-item">
          <div class="command-info">
            <span class="command-name">{{ cmd.name }}</span>
            <code class="command-value">{{ cmd.command }}</code>
            <span v-if="cmd.description" class="command-desc">{{ cmd.description }}</span>
          </div>
          <div class="command-actions">
            <button class="btn btn-icon" @click="openEditCommand(cmd)" title="Edit">
              <i class="pi pi-pencil"></i>
            </button>
            <button class="btn btn-icon btn-danger" @click="deleteGlobalCommand(cmd.id)" title="Delete">
              <i class="pi pi-trash"></i>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Command Modal -->
    <div v-if="showAddCommand" class="modal-overlay" @click.self="showAddCommand = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add Global Command</h3>
          <button class="btn btn-icon" @click="showAddCommand = false">
            <i class="pi pi-times"></i>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newCommandName" type="text" placeholder="e.g., Claude Code, pnpm dev" />
          </div>
          <div class="form-group">
            <label>Command *</label>
            <input v-model="newCommandValue" type="text" placeholder="e.g., claude, pnpm run dev" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <input v-model="newCommandDescription" type="text" placeholder="Optional description" />
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showAddCommand = false">Cancel</button>
          <button class="btn btn-primary" @click="addGlobalCommand" :disabled="!newCommandName.trim() || !newCommandValue.trim()">
            Add Command
          </button>
        </div>
      </div>
    </div>

    <!-- Edit Command Modal -->
    <div v-if="showEditCommand && editingCommand" class="modal-overlay" @click.self="showEditCommand = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Edit Global Command</h3>
          <button class="btn btn-icon" @click="showEditCommand = false">
            <i class="pi pi-times"></i>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="editingCommand.name" type="text" placeholder="e.g., Claude Code" />
          </div>
          <div class="form-group">
            <label>Command *</label>
            <input v-model="editingCommand.command" type="text" placeholder="e.g., claude" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <input v-model="editingCommand.description" type="text" placeholder="Optional description" />
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showEditCommand = false">Cancel</button>
          <button class="btn btn-primary" @click="saveEditedCommand">Save</button>
        </div>
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

/* Global Commands Section */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.section-header h2 {
  margin: 0;
  padding: 0;
  border: none;
}

.section-description {
  color: var(--text-secondary);
  font-size: 13px;
  margin-bottom: 16px;
}

.empty-commands {
  text-align: center;
  padding: 24px;
  background: var(--bg-primary);
  border-radius: 8px;
}

.empty-commands p {
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.commands-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.command-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-primary);
  border-radius: 8px;
}

.command-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.command-name {
  font-weight: 500;
}

.command-value {
  font-family: monospace;
  font-size: 13px;
  color: var(--accent);
  background: rgba(0, 217, 255, 0.1);
  padding: 2px 8px;
  border-radius: 4px;
}

.command-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

.command-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  width: 32px;
  height: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.btn-icon.btn-danger:hover {
  background: rgba(255, 82, 82, 0.15);
  border-color: rgba(255, 82, 82, 0.3);
  color: #ff5252;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 13px;
  display: flex;
  align-items: center;
  gap: 6px;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-secondary);
  border-radius: 12px;
  width: 100%;
  max-width: 450px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.modal-header h3 {
  margin: 0;
  font-size: 16px;
}

.modal-body {
  padding: 20px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.form-group {
  margin-bottom: 16px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 6px;
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-primary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 14px;
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent);
}
</style>
