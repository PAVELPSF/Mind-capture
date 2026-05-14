/// <reference types="vitest/globals" />
import "@testing-library/jest-dom/vitest";

vi.mock("@tauri-apps/api/event", () => ({
  listen: () => Promise.resolve(() => {}),
}));

vi.mock("@tauri-apps/api/core", () => {
  const handlers: Record<string, unknown> = {
    get_tabs: { tabs: [], total: 0, page: 1, per_page: 50 },
    get_providers: [
      { id: "claude", name: "Claude API", available: false, enabled: false },
      { id: "openai", name: "OpenAI", available: false, enabled: false },
      { id: "ollama", name: "Ollama (local)", available: false, enabled: false },
    ],
    get_config: {
      active_provider: "claude",
      claude: { api_key: "", model: "claude-sonnet-4-20250514", enabled: false },
      openai: { api_key: "", model: "gpt-4.1-nano", enabled: false },
      ollama: { api_key: "", model: "llama3.2", enabled: false },
    },
    set_provider: {} as unknown,
    set_active_provider: {} as unknown,
    analyze_tabs: { analyzed: 0, failed: 0 },
    get_status: "ok — 0 tabs in database",
  };

  return {
    invoke: (cmd: string) => {
      if (cmd in handlers) return Promise.resolve(handlers[cmd]);
      return Promise.resolve();
    },
  };
});
