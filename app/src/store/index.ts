import { create } from "zustand";

export interface Tab {
  id: number;
  url: string;
  title: string;
  favicon: string | null;
  browser: string;
  importedAt: string;
  status: "new" | "analyzed" | "reviewed" | "exported" | "deleted";
}

export interface Collection {
  id: number;
  name: string;
  color: string | null;
  icon: string | null;
  createdAt: string;
}

export interface Note {
  id: number;
  tabId: number;
  content: string;
  tags: string[];
  priority: number;
  createdAt: string;
}

export interface ImportResult {
  imported: number;
  duplicates: number;
}

export interface Review {
  id: number;
  tabId: number;
  decision: string;
  reviewedAt: string;
}

interface AppState {
  tabs: Tab[];
  collections: Collection[];
  notes: Note[];
  setTabs: (tabs: Tab[]) => void;
  addTab: (tab: Tab) => void;
  addTabs: (tabs: Tab[]) => void;
  setCollections: (collections: Collection[]) => void;
  setNotes: (notes: Note[]) => void;
}

export const useAppStore = create<AppState>((set) => ({
  tabs: [],
  collections: [],
  notes: [],
  setTabs: (tabs) => set({ tabs }),
  addTab: (tab) => set((state) => ({ tabs: [...state.tabs, tab] })),
  addTabs: (newTabs) =>
    set((state) => ({
      tabs: [
        ...state.tabs,
        ...newTabs.filter(
          (nt) => !state.tabs.some((t) => t.url === nt.url),
        ),
      ],
    })),
  setCollections: (collections) => set({ collections }),
  setNotes: (notes) => set({ notes }),
}));
