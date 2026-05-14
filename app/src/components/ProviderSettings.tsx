import { Input } from "./ui/input";
import { Badge } from "./ui/badge";

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
  claude: ProviderConfig;
  openai: ProviderConfig;
  ollama: ProviderConfig;
}

interface ProviderSettingsProps {
  providers: ProviderInfo[];
  config: AppConfig | null;
  onToggle: (id: string, enabled: boolean) => void;
  onKeyChange: (id: string, key: string) => void;
  onModelChange: (id: string, model: string) => void;
  onSetActive: (id: string) => void;
}

function getConfig(cfg: AppConfig | null, id: string): ProviderConfig {
  if (!cfg) return { api_key: "", model: "", enabled: false };
  const map: Record<string, ProviderConfig> = {
    claude: cfg.claude,
    openai: cfg.openai,
    ollama: cfg.ollama,
  };
  return map[id] ?? { api_key: "", model: "", enabled: false };
}

export function ProviderSettings({
  providers,
  config,
  onToggle,
  onKeyChange,
  onModelChange,
  onSetActive,
}: ProviderSettingsProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">AI-провайдеры</h3>
      <p className="text-sm text-muted-foreground">
        Настройте хотя бы одного провайдера для автоматического анализа вкладок.
        Ключи API хранятся локально и никогда не отправляются никуда, кроме API провайдера.
      </p>

      <div className="space-y-3">
        {providers.map((p) => {
          const cfg = getConfig(config, p.id);
          const isActive = config?.active_provider === p.id;
          const label = p.id === "ollama" ? "URL эндпоинта" : "API-ключ";

          return (
            <div
              key={p.id}
              className={`p-4 border rounded-lg space-y-3 ${
                isActive ? "border-accent bg-accent/5" : "border-border"
              }`}
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span className="font-medium">{p.name}</span>
                  {p.available && <Badge variant="outline" className="text-[10px]">Готов</Badge>}
                  {isActive && <Badge className="text-[10px]">Активен</Badge>}
                </div>
                <div className="flex items-center gap-2">
                  <label className="text-xs text-muted-foreground">Включить</label>
                  <input
                    type="checkbox"
                    checked={cfg.enabled}
                    onChange={(e) => onToggle(p.id, e.target.checked)}
                    className="w-4 h-4"
                  />
                </div>
              </div>

              {cfg.enabled && (
                <>
                  <div>
                    <label className="text-xs text-muted-foreground block mb-1">{label}</label>
                    <Input
                      type="password"
                      placeholder={p.id === "ollama" ? "http://localhost:11434" : "sk-..."}
                      value={cfg.api_key}
                      onChange={(e) => onKeyChange(p.id, e.target.value)}
                      className="text-sm"
                    />
                  </div>

                  <div>
                    <label className="text-xs text-muted-foreground block mb-1">Модель</label>
                    <Input
                      type="text"
                      value={cfg.model}
                      onChange={(e) => onModelChange(p.id, e.target.value)}
                      className="text-sm"
                    />
                  </div>

                  {!isActive && (
                    <button
                      onClick={() => onSetActive(p.id)}
                      className="text-xs text-accent hover:underline"
                    >
                      Сделать активным провайдером
                    </button>
                  )}
                </>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
