# MindCapture — Project Specification for Claude Code

> Этот документ — единственный источник истины для разработки.
> Обновляется синхронно с работой над проектом.
> Язык разработки и комментарии в коде: английский. Язык общения с разработчиком: русский.

---

## Проект

**MindCapture** — локальное desktop-приложение (Windows) + браузерное расширение.
Захватывает открытые вкладки браузера, анализирует их через Claude API, создаёт
структурированные заметки, проводит еженедельный опрос для чистки и экспортирует
структуру обратно в браузер в виде папок закладок.

**Принцип**: все данные хранятся локально. Ничего не уходит во внешние сервисы кроме
запросов к Claude API (только URL и заголовок страницы).

---

## Расположение проекта

```
C:\Users\ppash\Desktop\My vaults\02 - Projects\MY_CLAUDE2026\MindCapture\
├── app/              ← Tauri + React приложение
├── extension/        ← Браузерное расширение (WebExtension MV3)
├── docs/
│   ├── claude.md     ← этот файл (живой документ)
│   ├── plan.html     ← визуальная дорожная карта
│   └── skills/
│       └── mindcapture-development/SKILL.md
```

---

## Технологический стек

### Frontend (app/src/)
- React 18 + TypeScript
- React Router v7
- Zustand — глобальное состояние
- TanStack Query — async/cache
- Tailwind CSS + shadcn/ui
- Lucide React — иконки

### Backend (app/src-tauri/)
- Tauri 2.x
- Rust: rusqlite, serde_json, reqwest, tokio

### Extension (extension/)
- WebExtension Manifest V3 (Edge, Chrome, Firefox)
- TypeScript, без фреймворка
- Native Messaging — связь с desktop app

### AI
- Claude API: `claude-sonnet-4-20250514`
- Ключ: Tauri secure store (никогда не в коде)

---

## Архитектура

```
[Браузер]
  └── [Расширение MV3]
        │  tabs + bookmarks API
        └──► native messaging ──► [Tauri Backend (Rust)]
                                        │
                               ┌────────┴────────┐
                               │                 │
                          [SQLite DB]      [Claude API]
                               │
                          [React UI]
                    Inbox / Library / Neglected
                    Purgatory / Settings
```

---

## База данных (SQLite)

```sql
tabs            (id, url, title, favicon, browser, imported_at, status)
notes           (id, tab_id, content, tags JSON, priority, created_at)
collections     (id, name, color, icon, created_at)
tab_collections (tab_id, collection_id)
reviews         (id, tab_id, decision, reviewed_at)
sync_log        (id, action, entity_id, timestamp)
```

**Статусы вкладки:** `new` → `analyzed` → `reviewed` → `exported` | `deleted`

---

## Правила разработки

1. **Локальность**: данные пользователя не покидают устройство
2. **Секреты**: API ключ только через Tauri secure store
3. **Валидация**: входящие данные из расширения валидировать на границе системы
4. **Изменения**: небольшие сфокусированные задачи, не расширять объём без запроса
5. **API**: для актуальных версий библиотек использовать документацию, не угадывать
6. **Язык**: идентификаторы и комментарии в коде — английский
7. **QA**: после каждого спринта — верификация по чек-листу из plan.html
8. **Cursor rules**: при наличии `.cursor/rules/` в репозитории следовать им;
   при конфликте — приоритет у явных инструкций пользователя в текущем чате

---

## Спринты

### Sprint 1 — Структура проекта и база данных
**Статус:** `DONE` (2026-05-14)
- Инициализировать Tauri 2.x проект
- Создать схему SQLite (все таблицы)
- Базовый React Router (5 страниц, пустые заглушки)
- Zustand store: начальное состояние
- Tailwind CSS + shadcn/ui

### Sprint 2 — Браузерное расширение
**Статус:** `DONE` (2026-05-14)
- Manifest V3, разрешения: tabs, storage, bookmarks
- Popup UI: кнопка "Отправить вкладки" + индикатор статуса
- Сбор данных: url, title, favicon, browser
- HTTP-сервер (tiny_http) на localhost:1422 — приём вкладок от расширения
- Валидация входных данных на границе системы
- `import_tabs` — запись в SQLite с проверкой дубликатов
- Примечание: вместо native messaging используется HTTP (проще, не требует настройки реестра)

### Sprint 3 — UI: Inbox
**Статус:** `TODO`
- Страница Inbox: список всех импортированных вкладок
- Фильтрация по браузеру, статусу
- Карточка вкладки: favicon, title, url, статус, дата
- Tauri command: `get_tabs` (чтение из SQLite с фильтрацией)

### Sprint 4 — AI-анализ (мульти-провайдер)
**Статус:** `DONE` (2026-05-14)
- Трейт `AiProvider` — общий интерфейс для всех AI-провайдеров
- **Claude API** — `ai/claude.rs`, модель `claude-sonnet-4-20250514`
- **OpenAI** — `ai/openai.rs`, модель `gpt-4.1-nano`, JSON mode
- **Ollama (локальный)** — `ai/ollama.rs`, модель `llama3.2`
- `analyze_tabs` — батчинг, запись в `notes`, обновление статуса на `analyzed`
- `config.rs` — хранение API ключей в JSON (app data dir)
- Settings UI: ProviderSettings — enable/disable, ключи, модели, активный провайдер

### Sprint 5 — Чистилище (еженедельный опрос)
**Статус:** `DONE` (2026-05-14)
- 5 Tauri commands: get_purgatory_batch, submit_review, get_review_history, get/set_purgatory_config
- Purgatory UI: idle → reviewing (по одной вкладке) → done (статистика)
- Кнопки: Keep (reviewed) / Delete (deleted) / Later (остаётся в пуле)
- Запись в reviews + обновление статуса tabs
- Settings: размер сессии (5-50, default 15)
- Сохранение purgatory_batch_size в AppConfig

### Sprint 6 — Экспорт в браузер
**Статус:** `TODO`
- Tauri command: `export_collections`
- Расширение: запись в Bookmarks API
- Структура: `MindCapture > {collection} > {title}`
- Delta export: только изменения с последнего экспорта
- HTML-файл закладок (универсальный формат)

---

## Запланировано в MVP, реализуется в V2+

| Фича | Версия |
|---|---|
| YouTube / RuTube / VK / Instagram API | V2 |
| Граф знаний (визуализация связей) | V2 |
| Edge Mobile Sync (мобильные вкладки) | V2 |
| Еженедельный AI-дайджест | V2 |
| Мобильное приложение (iOS/Android) | V3 |
| Поведенческий профиль пользователя | V3 |

---

## Dependencies

> Этот раздел обновляется автоматически при завершении каждого спринта.
> Команда для обновления: `cd app && npm list --depth=0`

```
app@0.1.0
├── @tanstack/react-query@5.100.10
├── @tauri-apps/api@2
├── @tauri-apps/plugin-opener@2
├── lucide-react@1.14.0
├── react@19.1.0
├── react-dom@19.1.0
├── react-router-dom@7.15.0
└── zustand@5.0.13
```

---

## Completed Sprints Log

> Автоматически пополняется при закрытии каждого спринта.

| Sprint | Завершён | Ключевые файлы |
|---|---|---|
| S1 | 2026-05-14 | `app/src-tauri/src/db/schema.rs`, `app/src-tauri/src/db/models.rs`, `app/src/store/index.ts`, `app/src/App.tsx` |
| S2 | 2026-05-14 | `app/src-tauri/src/server.rs`, `app/src-tauri/src/commands/import.rs`, `extension/src/popup/popup.ts`, `extension/manifest.json` |
| S3 | 2026-05-14 | `app/src-tauri/src/commands/mod.rs` (get_tabs), `app/src/pages/Inbox.tsx`, `app/src/components/TabCard.tsx`, `app/src/components/FilterBar.tsx` |
| S4 | 2026-05-14 | `app/src-tauri/src/ai/mod.rs`, `app/src-tauri/src/ai/claude.rs`, `app/src-tauri/src/ai/openai.rs`, `app/src-tauri/src/ai/ollama.rs`, `app/src-tauri/src/config.rs`, `app/src/components/ProviderSettings.tsx` |
| S5 | 2026-05-14 | `app/src-tauri/src/commands/purgatory.rs`, `app/src/pages/Purgatory.tsx`, `app/src/pages/Settings.tsx` |

---

## Ключевые файлы проекта

> Обновляется при завершении каждого спринта.

```
app/src-tauri/src/main.rs
app/src-tauri/src/lib.rs
app/src-tauri/src/db/mod.rs
app/src-tauri/src/db/schema.rs
app/src-tauri/src/db/models.rs
app/src-tauri/src/commands/mod.rs
app/src-tauri/src/commands/import.rs
app/src-tauri/src/server.rs
app/src/main.tsx
app/src/App.tsx
app/src/store/index.ts
app/src/components/Layout.tsx
app/src/components/ui/button.tsx
app/src/components/ui/card.tsx
app/src/components/ui/badge.tsx
app/src/components/ui/input.tsx
app/src/pages/Inbox.tsx
app/src/pages/Library.tsx
app/src/pages/Neglected.tsx
app/src/pages/Purgatory.tsx
app/src/pages/Settings.tsx
extension/manifest.json
extension/src/background.ts
extension/src/popup/popup.html
extension/src/popup/popup.ts
extension/build.mjs
```
