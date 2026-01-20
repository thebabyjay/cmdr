<script setup lang="ts">
import { ref, watch } from "vue";
import { useProjectsStore } from "@/stores/projects";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
}>();

const projectsStore = useProjectsStore();

const form = ref({
  name: "",
  path: "",
  description: "",
  tags: "",
});

const saving = ref(false);
const error = ref<string | null>(null);

watch(
  () => props.visible,
  (val) => {
    if (val) {
      // Reset form
      form.value = { name: "", path: "", description: "", tags: "" };
      error.value = null;
    }
  }
);

const selectFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Project Folder",
    });
    if (selected) {
      form.value.path = selected as string;
      if (!form.value.name) {
        // Auto-fill name from folder name
        const parts = form.value.path.split("/");
        form.value.name = parts[parts.length - 1] || "";
      }
    }
  } catch (e) {
    console.error("Failed to open dialog:", e);
  }
};

const handleSubmit = async () => {
  if (!form.value.name || !form.value.path) {
    error.value = "Name and path are required";
    return;
  }

  saving.value = true;
  error.value = null;

  try {
    await projectsStore.addProject({
      name: form.value.name,
      path: form.value.path,
      description: form.value.description || undefined,
      tags: form.value.tags
        ? form.value.tags.split(",").map((t) => t.trim())
        : [],
      environments: {},
      workspaces: [],
      commands: [],
    });
    emit("update:visible", false);
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
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
          <h2>Add Project</h2>
          <button class="close-btn" @click="close">
            <i class="pi pi-times"></i>
          </button>
        </div>

        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label>Project Name *</label>
            <input
              v-model="form.name"
              type="text"
              placeholder="My Awesome Project"
            />
          </div>

          <div class="form-group">
            <label>Project Path *</label>
            <div class="path-input">
              <input
                v-model="form.path"
                type="text"
                placeholder="/path/to/project"
              />
              <button type="button" class="btn btn-secondary" @click="selectFolder">
                Browse
              </button>
            </div>
          </div>

          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="form.description"
              placeholder="What is this project about?"
              rows="3"
            ></textarea>
          </div>

          <div class="form-group">
            <label>Tags</label>
            <input
              v-model="form.tags"
              type="text"
              placeholder="web, typescript, api (comma separated)"
            />
          </div>

          <div v-if="error" class="error-message">
            {{ error }}
          </div>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="close">
              Cancel
            </button>
            <button type="submit" class="btn btn-primary" :disabled="saving">
              {{ saving ? "Saving..." : "Add Project" }}
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

.path-input {
  display: flex;
  gap: 8px;
}

.path-input input {
  flex: 1;
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
