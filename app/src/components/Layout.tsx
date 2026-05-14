import { NavLink } from "react-router-dom";
import { Inbox, Library, AlertCircle, Timer, Settings } from "lucide-react";

const NAV_ITEMS = [
  { to: "/", label: "Входящие", icon: Inbox },
  { to: "/library", label: "Библиотека", icon: Library },
  { to: "/neglected", label: "Заброшенные", icon: AlertCircle },
  { to: "/purgatory", label: "Чистилище", icon: Timer },
  { to: "/settings", label: "Настройки", icon: Settings },
];

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps) {
  return (
    <div className="flex h-screen bg-background">
      <aside className="flex w-56 flex-col gap-1 border-r border-sidebar-border bg-sidebar p-4">
        <h1 className="mb-4 px-3 py-2 text-lg font-bold text-sidebar-foreground">
          MindCapture
        </h1>
        {NAV_ITEMS.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              `flex items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors ${
                isActive
                  ? "bg-sidebar-primary text-sidebar-primary-foreground"
                  : "text-muted-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
              }`
            }
          >
            <Icon size={18} />
            {label}
          </NavLink>
        ))}
      </aside>
      <main className="flex-1 overflow-auto bg-background p-6">{children}</main>
    </div>
  );
}
