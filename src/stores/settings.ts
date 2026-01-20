import { defineStore } from "pinia";
import { ref } from "vue";
import type { AppSettings } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<AppSettings>({
    defaultTerminal: "iterm2",
    theme: "dark",
    terminalBehavior: "new_window",
    globalCommands: [],
  });
  const loading = ref(false);

  async function loadSettings() {
    loading.value = true;
    try {
      settings.value = await invoke<AppSettings>("get_settings");
    } catch (e) {
      console.error("Failed to load settings:", e);
    } finally {
      loading.value = false;
    }
  }

  async function saveSettings(newSettings: AppSettings) {
    try {
      await invoke("save_settings", { settings: newSettings });
      settings.value = newSettings;
    } catch (e) {
      console.error("Failed to save settings:", e);
      throw e;
    }
  }

  return {
    settings,
    loading,
    loadSettings,
    saveSettings,
  };
});
