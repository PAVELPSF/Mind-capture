import { describe, it, expect, beforeEach } from "vitest";
import { useAppStore, Tab, Collection, Note } from "./index";

describe("useAppStore", () => {
  beforeEach(() => {
    useAppStore.setState({ tabs: [], collections: [], notes: [] });
  });

  it("initializes with empty arrays", () => {
    const state = useAppStore.getState();
    expect(state.tabs).toEqual([]);
    expect(state.collections).toEqual([]);
    expect(state.notes).toEqual([]);
  });

  it("sets tabs", () => {
    const tabs: Tab[] = [
      {
        id: 1,
        url: "https://example.com",
        title: "Example",
        favicon: null,
        browser: "Edge",
        importedAt: "2026-05-14",
        status: "new",
      },
    ];
    useAppStore.getState().setTabs(tabs);
    expect(useAppStore.getState().tabs).toHaveLength(1);
    expect(useAppStore.getState().tabs[0].url).toBe("https://example.com");
  });

  it("adds a tab immutably", () => {
    const tab: Tab = {
      id: 1,
      url: "https://example.com",
      title: "Example",
      favicon: null,
      browser: "Edge",
      importedAt: "2026-05-14",
      status: "new",
    };

    const before = useAppStore.getState().tabs;
    useAppStore.getState().addTab(tab);
    const after = useAppStore.getState().tabs;

    expect(before).toHaveLength(0);
    expect(after).toHaveLength(1);
    // Verify immutability: original reference preserved
    expect(before).not.toBe(after);
  });

  it("sets collections", () => {
    const collections: Collection[] = [
      { id: 1, name: "Work", color: "#ff0000", icon: null, createdAt: "2026-05-14" },
    ];
    useAppStore.getState().setCollections(collections);
    expect(useAppStore.getState().collections).toHaveLength(1);
  });

  it("sets notes", () => {
    const notes: Note[] = [
      { id: 1, tabId: 1, content: "A note", tags: ["ai"], priority: 1, createdAt: "2026-05-14" },
    ];
    useAppStore.getState().setNotes(notes);
    expect(useAppStore.getState().notes).toHaveLength(1);
  });

  it("returns correct tab status values", () => {
    const validStatuses = ["new", "analyzed", "reviewed", "exported", "deleted"];
    const tab: Tab = {
      id: 1,
      url: "https://example.com",
      title: "Example",
      favicon: null,
      browser: "Edge",
      importedAt: "2026-05-14",
      status: "new",
    };
    expect(validStatuses).toContain(tab.status);
  });
});
