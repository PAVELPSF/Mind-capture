import { Globe } from "lucide-react";
import { Badge } from "./ui/badge";
import { Card } from "./ui/card";

interface TabCardProps {
  url: string;
  title: string;
  favicon: string | null;
  browser: string;
  status: string;
  importedAt: string;
}

const STATUS_VARIANT: Record<string, "default" | "secondary" | "outline" | "destructive"> = {
  new: "default",
  analyzed: "secondary",
  reviewed: "outline",
  exported: "outline",
  deleted: "destructive",
};

const STATUS_LABEL: Record<string, string> = {
  new: "Новая",
  analyzed: "Проанализирована",
  reviewed: "Просмотрена",
  exported: "Экспортирована",
  deleted: "Удалена",
};

export function TabCard({ url, title, favicon, browser, status, importedAt }: TabCardProps) {
  return (
    <Card className="p-3 flex gap-3 items-start hover:bg-accent/10 transition-colors">
      <img
        src={favicon ?? undefined}
        alt=""
        className="w-4 h-4 mt-0.5 flex-shrink-0"
        onError={(e) => {
          const target = e.currentTarget;
          target.style.display = "none";
          target.nextElementSibling?.classList.remove("hidden");
        }}
      />
      <Globe className="w-4 h-4 mt-0.5 flex-shrink-0 text-muted-foreground hidden" />
      <div className="flex-1 min-w-0">
        <div className="font-medium text-sm truncate">{title}</div>
        <div className="text-xs text-muted-foreground truncate">{url}</div>
        <div className="flex items-center gap-2 mt-1.5">
          <Badge variant={STATUS_VARIANT[status] ?? "default"} className="text-[10px]">
            {STATUS_LABEL[status] ?? status}
          </Badge>
          <span className="text-[10px] text-muted-foreground">{browser}</span>
          <span className="text-[10px] text-muted-foreground">
            {new Date(importedAt + "Z").toLocaleDateString()}
          </span>
        </div>
      </div>
    </Card>
  );
}
