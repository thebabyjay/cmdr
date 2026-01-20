<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  add: [name: string, variables: Record<string, string>];
}>();

const name = ref("");
const variables = ref<{ key: string; value: string }[]>([{ key: "", value: "" }]);
const error = ref<string | null>(null);

watch(
  () => props.visible,
  (val) => {
    if (val) {
      name.value = "";
      variables.value = [{ key: "", value: "" }];
      error.value = null;
    }
  }
);

const addVariable = () => {
  variables.value.push({ key: "", value: "" });
};

const removeVariable = (index: number) => {
  variables.value.splice(index, 1);
  if (variables.value.length === 0) {
    variables.value = [{ key: "", value: "" }];
  }
};

const handleSubmit = () => {
  if (!name.value.trim()) {
    error.value = "Environment name is required";
    return;
  }

  const vars: Record<string, string> = {};
  for (const v of variables.value) {
    if (v.key.trim()) {
      vars[v.key.trim()] = v.value;
    }
  }

  emit("add", name.value.trim(), vars);
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
          <h2>Add Environment</h2>
          <button class="close-btn" @click="close">
            <i class="pi pi-times"></i>
          </button>
        </div>

        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label>Environment Name *</label>
            <input
              v-model="name"
              type="text"
              placeholder="e.g., development, production"
            />
          </div>

          <div class="form-group">
            <label>Environment Variables</label>
            <div class="variables-list">
              <div
                v-for="(variable, index) in variables"
                :key="index"
                class="variable-row"
              >
                <input
                  v-model="variable.key"
                  type="text"
                  placeholder="KEY"
                  class="key-input"
                />
                <input
                  v-model="variable.value"
                  type="text"
                  placeholder="value"
                  class="value-input"
                />
                <button
                  type="button"
                  class="remove-btn"
                  @click="removeVariable(index)"
                >
                  <i class="pi pi-times"></i>
                </button>
              </div>
            </div>
            <button type="button" class="btn btn-secondary btn-sm" @click="addVariable">
              <i class="pi pi-plus"></i>
              Add Variable
            </button>
          </div>

          <div v-if="error" class="error-message">
            {{ error }}
          </div>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="close">
              Cancel
            </button>
            <button type="submit" class="btn btn-primary">
              Add Environment
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

.variables-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.variable-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.key-input {
  width: 40% !important;
  font-family: monospace;
}

.value-input {
  flex: 1;
  font-family: monospace;
}

.remove-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 8px;
}

.remove-btn:hover {
  color: var(--danger);
}

.btn-sm {
  padding: 8px 12px;
  font-size: 13px;
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
