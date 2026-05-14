import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, FileDown, Bookmark, CircleCheck } from "lucide-react";
import { Card } from "../components/ui/card";

interface ExportStatus {
  ready_count: number;
  last_export: string | null;
}

interface ExportHtmlResult {
  path: string;
  total: number;
}

async function fetchStatus(): Promise<ExportStatus> {
  return invoke<ExportStatus>("get_export_status");
}

async function exportHtml(): Promise<ExportHtmlResult> {
  return invoke<ExportHtmlResult>("export_html");
}

export function Library() {
  const queryClient = useQueryClient();

  const { data: status, isLoading } = useQuery({
    queryKey: ["export-status"],
    queryFn: fetchStatus,
  });

  const htmlMutation = useMutation({
    mutationFn: exportHtml,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["export-status"] });
    },
  });

  return (
    <div className="max-w-2xl">
      <h2 className="text-2xl font-bold mb-6">Библиотека</h2>

      {isLoading ? (
        <div className="flex items-center gap-2 text-muted-foreground py-4">
          <Loader2 className="w-4 h-4 animate-spin" />
          <span>Загрузка...</span>
        </div>
      ) : (
        <>
          <Card className="p-6 space-y-4 mb-6">
            <div className="flex items-center gap-2">
              <Bookmark className="w-5 h-5 text-accent" />
              <h3 className="font-semibold">Экспорт закладок</h3>
            </div>

            <p className="text-sm text-muted-foreground">
              Экспортируйте просмотренные вкладки обратно в браузер. Вкладки, отмеченные как «оставить»
              в Чистилище, группируются по коллекциям и экспортируются как закладки.
            </p>

            <div className="flex items-center gap-2 text-sm">
              <span className="text-muted-foreground">Готово к экспорту:</span>
              <span className="font-semibold">{status?.ready_count ?? 0} вкладок</span>
            </div>

            {status?.last_export && (
              <p className="text-xs text-muted-foreground">
                Последний экспорт: {new Date(Number(status.last_export) * 1000).toLocaleString()}
              </p>
            )}

            <div className="flex gap-3 pt-2">
              <button
                onClick={() => htmlMutation.mutate()}
                disabled={htmlMutation.isPending || (status?.ready_count ?? 0) === 0}
                className="inline-flex items-center gap-2 px-4 py-2 bg-accent text-white rounded-md font-medium hover:opacity-90 disabled:opacity-50 transition-opacity"
              >
                <FileDown className="w-4 h-4" />
                {htmlMutation.isPending ? "Экспорт..." : "Экспорт HTML"}
              </button>

              <button
                onClick={() => {
                  window.open("http://127.0.0.1:1422/export-payload", "_blank");
                }}
                disabled={(status?.ready_count ?? 0) === 0}
                className="inline-flex items-center gap-2 px-4 py-2 border border-border rounded-md font-medium hover:bg-accent/10 disabled:opacity-50 transition-colors"
              >
                <Bookmark className="w-4 h-4" />
                Экспорт в браузер
              </button>
            </div>

            {htmlMutation.isSuccess && (
              <div className="flex items-start gap-2 p-3 bg-green-50 dark:bg-green-950 rounded-md text-sm">
                <CircleCheck className="w-4 h-4 text-green-600 mt-0.5" />
                <div>
                  <p className="font-medium text-green-700 dark:text-green-300">
                    Экспортировано {htmlMutation.data.total} закладок
                  </p>
                  <p className="text-green-600 dark:text-green-400 text-xs break-all">
                    {htmlMutation.data.path}
                  </p>
                </div>
              </div>
            )}

            {htmlMutation.isError && (
              <p className="text-sm text-destructive">
                {htmlMutation.error instanceof Error
                  ? htmlMutation.error.message
                  : "Экспорт не удался"}
              </p>
            )}
          </Card>

          <p className="text-sm text-muted-foreground">
            Организованные коллекции и AI-заметки появятся здесь в будущих обновлениях.
          </p>
        </>
      )}
    </div>
  );
}
