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
  max-width: 700px;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h1 {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 4px;
}

.subtitle {
  color: var(--text-secondary);
  font-size: 14px;
}

.settings-section {
  margin-bottom: 20px;
  background: var(--bg-card);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 16px;
}

.settings-section h2 {
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 16px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--border-subtle);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-secondary);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-subtle);
}

.setting-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.setting-item:first-of-type {
  padding-top: 0;
}

.setting-info label {
  font-weight: 500;
  margin-bottom: 2px;
  display: block;
  font-size: 14px;
}

.setting-info p {
  font-size: 12px;
  color: var(--text-muted);
}

.setting-item select,
.setting-item input {
  padding: 8px 12px;
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  min-width: 200px;
  font-size: 13px;
}

.setting-item select:focus,
.setting-item input:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-muted);
}

.about-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.about-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.about-row span:first-child {
  color: var(--text-secondary);
}

.about-row code {
  font-family: "SF Mono", monospace;
  background: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-primary);
}

.settings-footer {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 12px;
}

.saved-message {
  color: var(--success);
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
}

/* Global Commands Section */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--border-subtle);
}

.section-header h2 {
  margin: 0;
  padding: 0;
  border: none;
}

.section-description {
  color: var(--text-muted);
  font-size: 12px;
  margin-bottom: 16px;
}

.empty-commands {
  text-align: center;
  padding: 24px;
  background: var(--bg-tertiary);
  border-radius: 6px;
  border: 1px solid var(--border-subtle);
}

.empty-commands p {
  color: var(--text-secondary);
  margin-bottom: 12px;
  font-size: 13px;
}

.commands-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.command-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
}

.command-info {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.command-name {
  font-weight: 500;
  font-size: 13px;
}

.command-value {
  font-family: "SF Mono", monospace;
  font-size: 12px;
  color: var(--accent);
  background: var(--accent-muted);
  padding: 2px 6px;
  border-radius: 4px;
}

.command-desc {
  font-size: 11px;
  color: var(--text-muted);
}

.command-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  width: 28px;
  height: 28px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-icon:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border-color: var(--border-accent);
}

.btn-icon.btn-danger:hover {
  background: var(--danger-muted);
  border-color: var(--danger);
  color: var(--danger);
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-card);
  border-radius: 8px;
  width: 100%;
  max-width: 420px;
  border: 1px solid var(--border-primary);
  box-shadow: var(--shadow-lg);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
  border-bottom: 1px solid var(--border-primary);
}

.modal-header h3 {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
}

.modal-body {
  padding: 16px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 16px;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
  border-radius: 0 0 8px 8px;
}

.form-group {
  margin-bottom: 14px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 6px;
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-input);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-muted);
}
</style>
