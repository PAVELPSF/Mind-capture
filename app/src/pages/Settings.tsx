import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, CheckCircle } from "lucide-react";
import { ProviderSettings } from "../components/ProviderSettings";
import { Input } from "../components/ui/input";

interface ProviderInfo {
  id: string;
  name: string;
  available: boolean;
  enabled: boolean;
}

interface ProviderConfig {
  api_key: string;
  model: string;
  enabled: boolean;
}

interface AppConfig {
  active_provider: string;
  purgatory_batch_size: number;
  claude: ProviderConfig;
  openai: ProviderConfig;
  ollama: ProviderConfig;
}

interface PurgatoryConfig {
  batch_size: number;
}

async function fetchProviders(): Promise<ProviderInfo[]> {
  return invoke<ProviderInfo[]>("get_providers");
}

async function fetchConfig(): Promise<AppConfig> {
  return invoke<AppConfig>("get_config");
}

async function fetchPurgatoryConfig(): Promise<PurgatoryConfig> {
  return invoke<PurgatoryConfig>("get_purgatory_config");
}

async function setProvider(params: {
  providerId: string;
  apiKey?: string;
  model?: string;
  enabled?: boolean;
}): Promise<AppConfig> {
  return invoke<AppConfig>("set_provider", { params });
}

async function setActiveProvider(providerId: string): Promise<AppConfig> {
  return invoke<AppConfig>("set_active_provider", {
    params: { providerId },
  });
}

async function setPurgatoryConfig(batchSize: number): Promise<PurgatoryConfig> {
  return invoke<PurgatoryConfig>("set_purgatory_config", {
    params: { batchSize },
  });
}

export function Settings() {
  const queryClient = useQueryClient();
  const [batchSizeInput, setBatchSizeInput] = useState<string | null>(null);

  const { data: providers, isLoading: pLoading } = useQuery({
    queryKey: ["providers"],
    queryFn: fetchProviders,
  });

  const { data: config, isLoading: cLoading } = useQuery({
    queryKey: ["config"],
    queryFn: fetchConfig,
  });

  const { data: purgatoryConfig } = useQuery({
    queryKey: ["purgatory-config"],
    queryFn: fetchPurgatoryConfig,
  });

  const setMutation = useMutation({
    mutationFn: setProvider,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["providers"] });
      queryClient.invalidateQueries({ queryKey: ["config"] });
    },
  });

  const activeMutation = useMutation({
    mutationFn: setActiveProvider,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["config"] });
    },
  });

  const purgatoryMutation = useMutation({
    mutationFn: setPurgatoryConfig,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["purgatory-config"] });
    },
  });

  if (pLoading || cLoading) {
    return (
      <div className="flex items-center gap-2 text-muted-foreground py-12 justify-center">
        <Loader2 className="w-5 h-5 animate-spin" />
        <span>Загрузка настроек...</span>
      </div>
    );
  }

  return (
    <div className="max-w-2xl">
      <h2 className="text-2xl font-bold mb-6">Настройки</h2>

      <div className="space-y-6">
        {/* Purgatory section */}
        <div className="space-y-3">
          <h3 className="text-lg font-semibold">Чистилище</h3>
          <p className="text-sm text-muted-foreground">
            Настройте еженедельную сессию просмотра.
          </p>
          <div className="p-4 border border-border rounded-lg space-y-2">
            <label className="text-sm font-medium block">
              Вкладок за сессию
            </label>
            <p className="text-xs text-muted-foreground">
              Сколько вкладок просматривать за одну сессию (5-50).
            </p>
            <div className="flex items-center gap-2">
              <Input
                type="number"
                min={5}
                max={50}
                value={
                  batchSizeInput ??
                  purgatoryConfig?.batch_size ??
                  15
                }
                onChange={(e) => setBatchSizeInput(e.target.value)}
                className="w-24 text-sm"
              />
              <button
                onClick={() => {
                  const n = Number(batchSizeInput ?? purgatoryConfig?.batch_size ?? 15);
                  purgatoryMutation.mutate(n);
                  setBatchSizeInput(null);
                }}
                disabled={purgatoryMutation.isPending}
                className="px-3 py-1.5 text-sm bg-accent text-white rounded-md hover:opacity-90 disabled:opacity-50"
              >
                {purgatoryMutation.isPending ? "Сохранение..." : "Сохранить"}
              </button>
              {purgatoryMutation.isSuccess && (
                <CheckCircle className="w-4 h-4 text-green-600" />
              )}
            </div>
          </div>
        </div>

        {/* Provider section */}
        <ProviderSettings
          providers={providers ?? []}
          config={config ?? null}
          onToggle={(id, enabled) =>
            setMutation.mutate({ providerId: id, enabled })
          }
          onKeyChange={(id, apiKey) =>
            setMutation.mutate({ providerId: id, apiKey })
          }
          onModelChange={(id, model) =>
            setMutation.mutate({ providerId: id, model })
          }
          onSetActive={(id) => activeMutation.mutate(id)}
        />
      </div>
    </div>
  );
}
