<script setup lang="ts">
const props = defineProps<{
  visible: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  danger?: boolean;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  confirm: [];
  cancel: [];
}>();

const close = () => {
  emit("update:visible", false);
  emit("cancel");
};

const confirm = () => {
  emit("update:visible", false);
  emit("confirm");
};
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="close">
      <div class="modal">
        <div class="modal-header">
          <h2>{{ title }}</h2>
          <button class="close-btn" @click="close">
            <i class="pi pi-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <p>{{ message }}</p>
        </div>

        <div class="modal-footer">
          <button class="btn btn-secondary" @click="close">
            {{ cancelText || "Cancel" }}
          </button>
          <button
            :class="['btn', danger ? 'btn-danger' : 'btn-primary']"
            @click="confirm"
          >
            {{ confirmText || "Confirm" }}
          </button>
        </div>
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
  max-width: 400px;
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

.modal-body {
  padding: 24px;
}

.modal-body p {
  color: var(--text-secondary);
  line-height: 1.5;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.btn-danger {
  background: var(--danger);
  color: white;
}

.btn-danger:hover {
  background: #ff3333;
}
</style>
