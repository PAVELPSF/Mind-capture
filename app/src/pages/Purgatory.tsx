import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useQuery, useMutation } from "@tanstack/react-query";
import { Loader2, Check, Trash2, Clock, Play, InboxIcon } from "lucide-react";
import { Badge } from "../components/ui/badge";
import { Card } from "../components/ui/card";

interface Tab {
  id: number;
  url: string;
  title: string;
  favicon: string | null;
  browser: string;
  imported_at: string;
  status: string;
}

interface Decision {
  tabId: number;
  decision: string;
}

async function fetchBatch(): Promise<Tab[]> {
  return invoke<Tab[]>("get_purgatory_batch", { params: { limit: null } });
}

async function submitDecision(params: { tabId: number; decision: string }) {
  return invoke("submit_review", { params });
}

export function Purgatory() {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [decisions, setDecisions] = useState<Decision[]>([]);
  const [phase, setPhase] = useState<"idle" | "reviewing" | "done">("idle");

  const { data: batch, isLoading, isError, error, refetch } = useQuery({
    queryKey: ["purgatory-batch"],
    queryFn: fetchBatch,
    enabled: false,
  });

  const submitMutation = useMutation({
    mutationFn: submitDecision,
    onSuccess: (_, vars) => {
      setDecisions((prev) => [...prev, { tabId: vars.tabId, decision: vars.decision }]);
    },
  });

  const startReview = useCallback(() => {
    setCurrentIndex(0);
    setDecisions([]);
    setPhase("reviewing");
    refetch();
  }, [refetch]);

  const handleDecision = useCallback(
    (decision: string) => {
      if (!batch || currentIndex >= batch.length) return;
      const tab = batch[currentIndex];
      submitMutation.mutate({ tabId: tab.id, decision });
      if (currentIndex + 1 >= batch.length) {
        setPhase("done");
      }
      setCurrentIndex((i) => i + 1);
    },
    [batch, currentIndex, submitMutation],
  );

  const keepCount = decisions.filter((d) => d.decision === "keep").length;
  const deleteCount = decisions.filter((d) => d.decision === "delete").length;
  const laterCount = decisions.filter((d) => d.decision === "later").length;

  if (phase === "idle") {
    return (
      <div className="max-w-lg">
        <h2 className="text-2xl font-bold mb-4">Чистилище</h2>
        <Card className="p-6 text-center space-y-4">
          <InboxIcon className="w-12 h-12 mx-auto text-muted-foreground" />
          <p className="text-muted-foreground">
            Просмотрите сохранённые вкладки и решите, что оставить, удалить или отложить.
          </p>
          <button
            onClick={startReview}
            className="inline-flex items-center gap-2 px-6 py-2.5 bg-accent text-white rounded-md font-medium hover:opacity-90 transition-opacity"
          >
            <Play className="w-4 h-4" />
            Начать просмотр
          </button>
        </Card>
      </div>
    );
  }

  if (isLoading || !batch) {
    return (
      <div className="flex items-center gap-2 text-muted-foreground py-12 justify-center">
        <Loader2 className="w-5 h-5 animate-spin" />
        <span>Загрузка партии для просмотра...</span>
      </div>
    );
  }

  if (isError) {
    return (
      <div className="max-w-lg">
        <h2 className="text-2xl font-bold mb-4">Чистилище</h2>
        <p className="text-destructive">
          {error instanceof Error ? error.message : "Не удалось загрузить партию"}
        </p>
      </div>
    );
  }

  if (batch.length === 0) {
    return (
      <div className="max-w-lg">
        <h2 className="text-2xl font-bold mb-4">Чистилище</h2>
        <Card className="p-6 text-center space-y-4">
          <InboxIcon className="w-12 h-12 mx-auto text-muted-foreground" />
          <p className="text-muted-foreground">
            Нет вкладок для просмотра. Сначала импортируйте вкладки.
          </p>
        </Card>
      </div>
    );
  }

  if (phase === "done") {
    return (
      <div className="max-w-lg">
        <h2 className="text-2xl font-bold mb-4">Чистилище</h2>
        <Card className="p-6 text-center space-y-4">
          <Check className="w-12 h-12 mx-auto text-green-500" />
          <p className="font-medium">
            Сессия завершена! Просмотрено {batch.length}:
          </p>
          <div className="flex justify-center gap-4 text-sm">
            <span className="text-green-600">{keepCount} сохранено</span>
            <span className="text-destructive">{deleteCount} удалено</span>
            <span className="text-muted-foreground">{laterCount} пропущено</span>
          </div>
          <button
            onClick={startReview}
            className="inline-flex items-center gap-2 px-6 py-2.5 bg-accent text-white rounded-md font-medium hover:opacity-90 transition-opacity"
          >
            <Play className="w-4 h-4" />
            Новая сессия
          </button>
        </Card>
      </div>
    );
  }

  const tab = batch[currentIndex];
  if (!tab) return null;

  return (
    <div className="max-w-lg">
      <h2 className="text-2xl font-bold mb-2">Purgatory</h2>
      <p className="text-sm text-muted-foreground mb-4">
        Прогресс: {currentIndex + 1} из {batch.length}
      </p>

      <Card className="p-5 mb-4">
        <div className="flex items-start gap-3">
          {tab.favicon ? (
            <img
              src={tab.favicon}
              alt=""
              className="w-4 h-4 mt-0.5 flex-shrink-0"
              onError={(e) => {
                (e.target as HTMLImageElement).style.display = "none";
              }}
            />
          ) : null}
          <div className="min-w-0 flex-1">
            <div className="font-medium text-sm truncate">{tab.title}</div>
            <div className="text-xs text-muted-foreground truncate mb-2">{tab.url}</div>
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="text-[10px]">{tab.browser}</Badge>
              <span className="text-[10px] text-muted-foreground">{tab.status}</span>
              <span className="text-[10px] text-muted-foreground">
                {new Date(tab.imported_at + "Z").toLocaleDateString()}
              </span>
            </div>
          </div>
        </div>
      </Card>

      <div className="flex gap-3">
        <button
          onClick={() => handleDecision("keep")}
          disabled={submitMutation.isPending}
          className="flex-1 flex items-center justify-center gap-2 px-4 py-3 bg-green-600 text-white rounded-md font-medium hover:bg-green-700 transition-colors disabled:opacity-50"
        >
          <Check className="w-4 h-4" />
          Оставить
        </button>
        <button
          onClick={() => handleDecision("delete")}
          disabled={submitMutation.isPending}
          className="flex-1 flex items-center justify-center gap-2 px-4 py-3 bg-destructive text-white rounded-md font-medium hover:opacity-90 transition-opacity disabled:opacity-50"
        >
          <Trash2 className="w-4 h-4" />
          Удалить
        </button>
        <button
          onClick={() => handleDecision("later")}
          disabled={submitMutation.isPending}
          className="flex-1 flex items-center justify-center gap-2 px-4 py-3 bg-secondary text-secondary-foreground rounded-md font-medium hover:bg-accent/10 transition-colors disabled:opacity-50"
        >
          <Clock className="w-4 h-4" />
          Позже
        </button>
      </div>
    </div>
  );
}
