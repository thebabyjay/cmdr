<script setup lang="ts">
import { RouterView } from "vue-router";
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <div class="app-layout">
    <Sidebar />
    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}
</style>
