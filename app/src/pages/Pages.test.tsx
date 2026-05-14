import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Inbox } from "./Inbox";
import { Library } from "./Library";
import { Neglected } from "./Neglected";
import { Purgatory } from "./Purgatory";
import { Settings } from "./Settings";

function renderPage(ui: React.ReactElement) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>{ui}</MemoryRouter>
    </QueryClientProvider>,
  );
}

describe("Inbox page", () => {
  it("renders heading", async () => {
    renderPage(<Inbox />);
    expect(await screen.findByRole("heading", { name: "Входящие" })).toBeInTheDocument();
  });
});

describe("Library page", () => {
  it("renders heading", () => {
    renderPage(<Library />);
    expect(screen.getByRole("heading", { name: "Библиотека" })).toBeInTheDocument();
  });
});

describe("Neglected page", () => {
  it("renders heading", () => {
    renderPage(<Neglected />);
    expect(screen.getByRole("heading", { name: "Заброшенные" })).toBeInTheDocument();
  });
});

describe("Purgatory page", () => {
  it("renders heading", () => {
    renderPage(<Purgatory />);
    expect(screen.getByRole("heading", { name: "Чистилище" })).toBeInTheDocument();
  });
});

describe("Settings page", () => {
  it("renders heading", async () => {
    renderPage(<Settings />);
    expect(await screen.findByRole("heading", { name: "Настройки" })).toBeInTheDocument();
  });
});
