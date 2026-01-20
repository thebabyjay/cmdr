<script setup lang="ts">
import { ref, watch } from "vue";
import type { Command } from "@/types";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  add: [command: Omit<Command, "id">];
}>();

const name = ref("");
const command = ref("");
const description = ref("");
const error = ref<string | null>(null);

watch(
  () => props.visible,
  (val) => {
    if (val) {
      name.value = "";
      command.value = "";
      description.value = "";
      error.value = null;
    }
  }
);

const handleSubmit = () => {
  if (!name.value.trim()) {
    error.value = "Command name is required";
    return;
  }
  if (!command.value.trim()) {
    error.value = "Command is required";
    return;
  }

  emit("add", {
    name: name.value.trim(),
    command: command.value.trim(),
    description: description.value.trim() || undefined,
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
          <h2>Add Command</h2>
          <button class="close-btn" @click="close">
            <i class="pi pi-times"></i>
          </button>
        </div>

        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label>Command Name *</label>
            <input
              v-model="name"
              type="text"
              placeholder="e.g., Start Dev Server"
            />
          </div>

          <div class="form-group">
            <label>Command *</label>
            <input
              v-model="command"
              type="text"
              placeholder="e.g., npm run dev"
              class="mono"
            />
          </div>

          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="description"
              placeholder="What does this command do?"
              rows="2"
            ></textarea>
          </div>

          <div v-if="error" class="error-message">
            {{ error }}
          </div>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="close">
              Cancel
            </button>
            <button type="submit" class="btn btn-primary">
              Add Command
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
  max-width: 500px;
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

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 500;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 12px;
  background: var(--bg-primary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 14px;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--accent);
}

.form-group textarea {
  resize: vertical;
  font-family: inherit;
}

.mono {
  font-family: monospace;
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
