import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import App from "./App";

function renderApp(initialRoute = "/") {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter initialEntries={[initialRoute]}>
        <App />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

describe("App routing", () => {
  it("renders Inbox on root path", () => {
    renderApp("/");
    expect(screen.getByRole("heading", { name: "Входящие" })).toBeInTheDocument();
  });

  it("renders Library on /library", () => {
    renderApp("/library");
    expect(screen.getByRole("heading", { name: "Библиотека" })).toBeInTheDocument();
  });

  it("renders Neglected on /neglected", () => {
    renderApp("/neglected");
    expect(screen.getByRole("heading", { name: "Заброшенные" })).toBeInTheDocument();
  });

  it("renders Purgatory on /purgatory", () => {
    renderApp("/purgatory");
    expect(screen.getByRole("heading", { name: "Чистилище" })).toBeInTheDocument();
  });

  it("renders Settings on /settings", async () => {
    renderApp("/settings");
    expect(await screen.findByRole("heading", { name: "Настройки" })).toBeInTheDocument();
  });

  it("renders navigation sidebar", () => {
    renderApp("/");
    expect(screen.getByText("MindCapture")).toBeInTheDocument();
    const navItems = ["Входящие", "Библиотека", "Заброшенные", "Чистилище", "Настройки"];
    for (const item of navItems) {
      const elements = screen.getAllByText(item);
      expect(elements.length).toBeGreaterThanOrEqual(1);
    }
  });
});
