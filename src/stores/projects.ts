import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Project } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useProjectsStore = defineStore("projects", () => {
  const projects = ref<Project[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const recentProjects = computed(() => {
    return [...projects.value]
      .filter((p) => p.lastOpened)
      .sort(
        (a, b) =>
          new Date(b.lastOpened!).getTime() - new Date(a.lastOpened!).getTime()
      )
      .slice(0, 5);
  });

  const projectById = computed(() => {
    return (id: string) => projects.value.find((p) => p.id === id);
  });

  async function loadProjects() {
    console.log("[ProjectsStore] Loading projects...");
    loading.value = true;
    error.value = null;
    try {
      projects.value = await invoke<Project[]>("get_projects");
      console.log("[ProjectsStore] Loaded", projects.value.length, "projects");
    } catch (e) {
      error.value = String(e);
      console.error("[ProjectsStore] Failed to load projects:", e);
    } finally {
      loading.value = false;
    }
  }

  async function addProject(project: Omit<Project, "id" | "createdAt">) {
    console.log("[ProjectsStore] Adding project:", project.name);
    try {
      const newProject = await invoke<Project>("add_project", { project });
      projects.value.push(newProject);
      console.log("[ProjectsStore] Project added with ID:", newProject.id);
      return newProject;
    } catch (e) {
      error.value = String(e);
      console.error("[ProjectsStore] Failed to add project:", e);
      throw e;
    }
  }

  async function updateProject(id: string, updates: Partial<Project>) {
    console.log("[ProjectsStore] Updating project:", id, "with:", Object.keys(updates));
    try {
      const updated = await invoke<Project>("update_project", { id, updates });
      const index = projects.value.findIndex((p) => p.id === id);
      if (index !== -1) {
        projects.value[index] = updated;
      }
      console.log("[ProjectsStore] Project updated successfully");
      return updated;
    } catch (e) {
      error.value = String(e);
      console.error("[ProjectsStore] Failed to update project:", e);
      throw e;
    }
  }

  async function deleteProject(id: string) {
    console.log("[ProjectsStore] Deleting project:", id);
    try {
      await invoke("delete_project", { id });
      projects.value = projects.value.filter((p) => p.id !== id);
      console.log("[ProjectsStore] Project deleted successfully");
    } catch (e) {
      error.value = String(e);
      console.error("[ProjectsStore] Failed to delete project:", e);
      throw e;
    }
  }

  async function openProject(id: string) {
    console.log("[ProjectsStore] Opening project:", id);
    try {
      await invoke("open_project", { id });
      const project = projects.value.find((p) => p.id === id);
      if (project) {
        project.lastOpened = new Date().toISOString();
      }
      console.log("[ProjectsStore] Project opened successfully");
    } catch (e) {
      error.value = String(e);
      console.error("[ProjectsStore] Failed to open project:", e);
      throw e;
    }
  }

  return {
    projects,
    loading,
    error,
    recentProjects,
    projectById,
    loadProjects,
    addProject,
    updateProject,
    deleteProject,
    openProject,
  };
});
