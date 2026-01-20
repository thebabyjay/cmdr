<script setup lang="ts">
import { ref, watch, computed } from "vue";
import type { Workspace, Pane, Command } from "@/types";
import { open } from "@tauri-apps/plugin-dialog";

const props = defineProps<{
  visible: boolean;
  projectPath: string;
  commands: Command[];
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  add: [workspace: Omit<Workspace, "id">];
}>();

interface PaneConfig {
  directory: string;
  commandType: "none" | "preset" | "custom";
  selectedCommandId: string;
  customCommand: string;
}

const name = ref("");
const rows = ref<{ columns: number; panes: PaneConfig[] }[]>([
  { columns: 1, panes: [{ directory: "", commandType: "none", selectedCommandId: "", customCommand: "" }] },
]);
const error = ref<string | null>(null);

const createEmptyPane = (): PaneConfig => ({
  directory: "",
  commandType: "none",
  selectedCommandId: "",
  customCommand: "",
});

watch(
  () => props.visible,
  (val) => {
    if (val) {
      name.value = "";
      rows.value = [
        { columns: 1, panes: [createEmptyPane()] },
      ];
      error.value = null;
    }
  }
);

const totalPanes = computed(() => {
  return rows.value.reduce((sum, row) => sum + row.columns, 0);
});

const addRow = () => {
  rows.value.push({
    columns: 1,
    panes: [createEmptyPane()],
  });
};

const removeRow = (index: number) => {
  if (rows.value.length > 1) {
    rows.value.splice(index, 1);
  }
};

const updateRowColumns = (rowIndex: number, newColumns: number) => {
  const row = rows.value[rowIndex];
  const oldColumns = row.columns;

  if (newColumns > oldColumns) {
    // Add panes
    for (let i = oldColumns; i < newColumns; i++) {
      row.panes.push(createEmptyPane());
    }
  } else if (newColumns < oldColumns) {
    // Remove panes
    row.panes.splice(newColumns);
  }

  row.columns = newColumns;
};

const browseDirectory = async (pane: PaneConfig) => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: pane.directory || props.projectPath || undefined,
    });
    if (selected) {
      pane.directory = selected as string;
    }
  } catch (e) {
    console.error("Failed to open folder picker:", e);
  }
};

const getCommandForPane = (pane: PaneConfig): string | undefined => {
  if (pane.commandType === "none") {
    return undefined;
  } else if (pane.commandType === "preset" && pane.selectedCommandId) {
    const cmd = props.commands.find(c => c.id === pane.selectedCommandId);
    return cmd?.command;
  } else if (pane.commandType === "custom") {
    return pane.customCommand || undefined;
  }
  return undefined;
};

const handleSubmit = () => {
  if (!name.value.trim()) {
    error.value = "Workspace name is required";
    return;
  }

  const workspacePanes: Pane[] = [];
  rows.value.forEach((row, rowIndex) => {
    row.panes.forEach((pane, colIndex) => {
      workspacePanes.push({
        position: [rowIndex, colIndex] as [number, number],
        directory: pane.directory || ".",
        command: getCommandForPane(pane),
      });
    });
  });

  emit("add", {
    name: name.value.trim(),
    layout: {
      rows: rows.value.length,
      columns: rows.value.map((r) => r.columns),
    },
    panes: workspacePanes,
  });
  emit("update:visible", false);
};

const close = () => {
  emit("update:visible", false);
};
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="close">
      <div class="modal">
        <div class="modal-header">
          <h2>Add Workspace</h2>
          <button class="close-btn" @click="close">
            <i class="pi pi-times"></i>
          </button>
        </div>

        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label>Workspace Name *</label>
            <input
              v-model="name"
              type="text"
              placeholder="e.g., Development, Testing"
            />
          </div>

          <div class="form-group">
            <div class="section-header">
              <label>Layout ({{ totalPanes }} panes)</label>
              <button type="button" class="btn btn-secondary btn-sm" @click="addRow">
                <i class="pi pi-plus"></i>
                Add Row
              </button>
            </div>

            <div class="rows-list">
              <div v-for="(row, rowIndex) in rows" :key="rowIndex" class="row-config">
                <div class="row-header">
                  <span class="row-label">Row {{ rowIndex + 1 }}</span>
                  <div class="row-controls">
                    <label class="column-label">Panes:</label>
                    <select
                      :value="row.columns"
                      @change="updateRowColumns(rowIndex, parseInt(($event.target as HTMLSelectElement).value))"
                    >
                      <option v-for="n in 6" :key="n" :value="n">{{ n }}</option>
                    </select>
                    <button
                      v-if="rows.length > 1"
                      type="button"
                      class="remove-row-btn"
                      @click="removeRow(rowIndex)"
                      title="Remove row"
                    >
                      <i class="pi pi-trash"></i>
                    </button>
                  </div>
                </div>

                <div class="panes-grid" :style="{ gridTemplateColumns: `repeat(${row.columns}, 1fr)` }">
                  <div
                    v-for="(pane, paneIndex) in row.panes"
                    :key="paneIndex"
                    class="pane-config"
                  >
                    <div class="pane-header">Pane {{ paneIndex + 1 }}</div>

                    <div class="pane-field">
                      <label>Directory</label>
                      <div class="input-with-button">
                        <input
                          v-model="pane.directory"
                          type="text"
                          placeholder="Use project default"
                          class="pane-input"
                        />
                        <button
                          type="button"
                          class="browse-btn"
                          @click="browseDirectory(pane)"
                          title="Browse"
                        >
                          <i class="pi pi-folder-open"></i>
                        </button>
                      </div>
                    </div>

                    <div class="pane-field">
                      <label>Command</label>
                      <select v-model="pane.commandType" class="pane-select">
                        <option value="none">None</option>
                        <option value="preset" :disabled="commands.length === 0">From project commands</option>
                        <option value="custom">Custom command</option>
                      </select>

                      <select
                        v-if="pane.commandType === 'preset'"
                        v-model="pane.selectedCommandId"
                        class="pane-select"
                      >
                        <option value="">Select a command...</option>
                        <option v-for="cmd in commands" :key="cmd.id" :value="cmd.id">
                          {{ cmd.name }}
                        </option>
                      </select>

                      <input
                        v-if="pane.commandType === 'custom'"
                        v-model="pane.customCommand"
                        type="text"
                        placeholder="Enter command..."
                        class="pane-input"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="layout-preview">
            <label>Preview</label>
            <div class="preview-grid">
              <div
                v-for="(row, rowIndex) in rows"
                :key="rowIndex"
                class="preview-row"
                :style="{ gridTemplateColumns: `repeat(${row.columns}, 1fr)` }"
              >
                <div
                  v-for="paneIndex in row.columns"
                  :key="paneIndex"
                  class="preview-pane"
                ></div>
              </div>
            </div>
          </div>

          <div v-if="error" class="error-message">
            {{ error }}
          </div>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="close">
              Cancel
            </button>
            <button type="submit" class="btn btn-primary">
              Add Workspace
            </button>
          </div>
        </form>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
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
  border-radius: 16px;
  width: 100%;
  max-width: 900px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.modal-header h2 {
  font-size: 18px;
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
}

.close-btn:hover {
  color: var(--text-primary);
}

form {
  padding: 24px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group > label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 12px;
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

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-header label {
  font-size: 14px;
  font-weight: 500;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 13px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.rows-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.row-config {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 12px;
}

.row-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.row-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.row-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.column-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.row-controls select {
  padding: 6px 10px;
  background: var(--bg-secondary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
}

.remove-row-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 6px;
}

.remove-row-btn:hover {
  color: var(--danger);
}

.panes-grid {
  display: grid;
  gap: 8px;
}

.pane-config {
  background: var(--bg-secondary);
  border-radius: 6px;
  padding: 10px;
}

.pane-header {
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: 6px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.pane-input {
  margin-bottom: 6px;
  padding: 8px !important;
  font-size: 12px !important;
}

.pane-field {
  margin-bottom: 8px;
}

.pane-field:last-child {
  margin-bottom: 0;
}

.pane-field label {
  display: block;
  font-size: 10px;
  color: var(--text-secondary);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.input-with-button {
  display: flex;
  gap: 4px;
}

.input-with-button input {
  flex: 1;
}

.browse-btn {
  background: var(--bg-primary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-secondary);
  padding: 0 10px;
  cursor: pointer;
  transition: all 0.2s;
}

.browse-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.pane-select {
  width: 100%;
  padding: 8px;
  background: var(--bg-primary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 12px;
  margin-bottom: 6px;
}

.pane-select:last-child {
  margin-bottom: 0;
}

.pane-select:focus {
  outline: none;
  border-color: var(--accent);
}

.layout-preview {
  margin-bottom: 20px;
}

.layout-preview label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 8px;
}

.preview-grid {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.preview-row {
  display: grid;
  gap: 4px;
}

.preview-pane {
  height: 32px;
  background: var(--accent);
  opacity: 0.3;
  border-radius: 4px;
}

.error-message {
  background: rgba(255, 82, 82, 0.1);
  border: 1px solid var(--danger);
  color: var(--danger);
  padding: 12px;
  border-radius: 8px;
  margin-bottom: 20px;
  font-size: 14px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}
</style>
