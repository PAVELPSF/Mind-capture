const BROWSERS = ["All", "Edge", "Chrome", "Firefox"];
const STATUSES = ["All", "new", "analyzed", "reviewed", "exported", "deleted"];
const STATUS_LABELS: Record<string, string> = {
  All: "Все",
  new: "Новые",
  analyzed: "Проанализированные",
  reviewed: "Просмотренные",
  exported: "Экспортированные",
  deleted: "Удалённые",
};

interface FilterBarProps {
  browser: string;
  onBrowserChange: (browser: string) => void;
  status: string;
  onStatusChange: (status: string) => void;
}

export function FilterBar({ browser, status, onBrowserChange, onStatusChange }: FilterBarProps) {
  return (
    <div className="flex gap-3">
      <select
        value={browser}
        onChange={(e) => onBrowserChange(e.target.value)}
        className="px-3 py-1.5 text-sm border border-border rounded-md bg-background text-foreground"
      >
        {BROWSERS.map((b) => (
          <option key={b} value={b === "All" ? "" : b}>
            {b === "All" ? "Все браузеры" : b}
          </option>
        ))}
      </select>
      <select
        value={status}
        onChange={(e) => onStatusChange(e.target.value)}
        className="px-3 py-1.5 text-sm border border-border rounded-md bg-background text-foreground"
      >
        {STATUSES.map((s) => (
          <option key={s} value={s === "All" ? "" : s}>
            {STATUS_LABELS[s] ?? s}
          </option>
        ))}
      </select>
    </div>
  );
}
