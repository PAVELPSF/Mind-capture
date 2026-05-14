import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useQuery } from "@tanstack/react-query";
import { Loader2, AlertCircle, InboxIcon } from "lucide-react";
import { FilterBar } from "../components/FilterBar";
import { TabCard } from "../components/TabCard";
import { Pagination } from "../components/Pagination";

interface Tab {
  id: number;
  url: string;
  title: string;
  favicon: string | null;
  browser: string;
  imported_at: string;
  status: string;
}

interface GetTabsResult {
  tabs: Tab[];
  total: number;
  page: number;
  per_page: number;
}

const PER_PAGE = 50;

async function fetchTabs(
  browser: string,
  status: string,
  page: number,
): Promise<GetTabsResult> {
  return invoke<GetTabsResult>("get_tabs", {
    params: {
      browser: browser || null,
      status: status || null,
      page,
      per_page: PER_PAGE,
    },
  });
}

export function Inbox() {
  const [browser, setBrowser] = useState("");
  const [status, setStatus] = useState("");
  const [page, setPage] = useState(1);

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["tabs", browser, status, page],
    queryFn: () => fetchTabs(browser, status, page),
    placeholderData: (prev) => prev,
  });

  const handleBrowserChange = (value: string) => {
    setBrowser(value);
    setPage(1);
  };

  const handleStatusChange = (value: string) => {
    setStatus(value);
    setPage(1);
  };

  const totalPages = data ? Math.max(1, Math.ceil(data.total / data.per_page)) : 0;

  return (
    <div>
      <h2 className="text-2xl font-bold mb-4">Входящие</h2>

      <FilterBar
        browser={browser}
        onBrowserChange={handleBrowserChange}
        status={status}
        onStatusChange={handleStatusChange}
      />

      {data && (
        <p className="text-sm text-muted-foreground mt-3">
          Показано {data.tabs.length} из {data.total} вкладок
        </p>
      )}

      <div className="mt-3">
        {isLoading && (
          <div className="flex items-center gap-2 text-muted-foreground py-12 justify-center">
            <Loader2 className="w-5 h-5 animate-spin" />
            <span>Загрузка вкладок...</span>
          </div>
        )}

        {isError && (
          <div className="flex items-center gap-2 text-destructive py-12 justify-center">
            <AlertCircle className="w-5 h-5" />
            <span>{error instanceof Error ? error.message : "Не удалось загрузить вкладки"}</span>
          </div>
        )}

        {data && data.tabs.length === 0 && (
          <div className="flex flex-col items-center gap-2 text-muted-foreground py-12">
            <InboxIcon className="w-8 h-8" />
            <p>Вкладок пока нет. Отправьте их из расширения браузера.</p>
          </div>
        )}

        {data && data.tabs.length > 0 && (
          <div className="space-y-2">
            {data.tabs.map((tab) => (
              <TabCard
                key={tab.id}
                url={tab.url}
                title={tab.title}
                favicon={tab.favicon}
                browser={tab.browser}
                status={tab.status}
                importedAt={tab.imported_at}
              />
            ))}
          </div>
        )}
      </div>

      <Pagination page={page} totalPages={totalPages} onPageChange={setPage} />
    </div>
  );
}
