---
name: mindcapture-development
description: >
  Используй этот skill при работе над проектом MindCapture — desktop-приложение
  (Tauri + React) + браузерное расширение (WebExtension Manifest V3) для захвата,
  AI-анализа и организации вкладок браузера. Применяй всегда, когда задача касается:
  архитектуры компонентов, схемы БД, Rust-backend, Claude API интеграции, логики
  расширения, системы "Чистилища" (еженедельного опроса), экспорта в браузер.
---

# MindCapture — Development Skill

## Что такое MindCapture

Desktop-приложение (Windows-first) + браузерное расширение для управления "цифровым мозгом"
пользователя. Захватывает вкладки из Edge/Chrome/Firefox, анализирует их через Claude API,
создаёт структурированные заметки локально, проводит еженедельный опрос-чистилище и
экспортирует результат обратно в браузер в виде папок с закладками.

Принцип: всё локально, данные пользователя не покидают устройство.

---

## Архитектура

```
extension/          ← WebExtension Manifest V3 (Edge, Chrome, Firefox)
app/
  src-tauri/        ← Rust backend (Tauri 2.x)
    src/
      main.rs
      commands/     ← Tauri commands (import, export, db, ai)
      db/           ← SQLite через rusqlite
  src/              ← React + TypeScript frontend
    pages/          ← Inbox, Library, Neglected, Purgatory, Settings
    components/
    hooks/
    store/          ← Zustand (глобальное состояние)
    api/            ← Claude API клиент
docs/
  claude.md
  plan.html
  skills/
    mindcapture-development/SKILL.md
```

---

## Стек и зависимости

### App (Tauri + React)
- Tauri 2.x (Rust backend, webview frontend)
- React 18 + TypeScript
- Zustand (state management)
- TanStack Query (async state, кэш)
- React Router v6
- Tailwind CSS
- shadcn/ui (компоненты)
- Lucide React (иконки)

### Extension
- WebExtension Manifest V3
- Vanilla JS / TypeScript (без фреймворка — меньше bundle size)
- Chrome Storage API (локальное хранение в расширении)

### Backend (Rust / Tauri commands)
- rusqlite (SQLite)
- serde / serde_json
- reqwest (HTTP для Claude API)
- tokio (async runtime)

> Актуальный список npm-пакетов автоматически обновляется в claude.md
> при завершении каждого спринта (см. раздел "Dependencies" в claude.md).

---

## База данных (SQLite)

```sql
tabs            — url, title, favicon, browser, imported_at, status
notes           — tab_id, content, tags (JSON), priority, created_at
collections     — name, color, icon, created_at
tab_collections — tab_id, collection_id
reviews         — tab_id, decision (keep|delete|later), reviewed_at
sync_log        — action, entity_id, timestamp (история изменений)
```

Статусы вкладки: `new` → `analyzed` → `reviewed` → `exported` | `deleted`

---

## Claude API — правила интеграции

- Модель: `claude-sonnet-4-20250514`
- Ключ API хранить только в Tauri secure store (не в коде, не в .env в репозитории)
- Промпты держать в `src/api/prompts.ts` как именованные константы
- Каждый запрос: URL + заголовок страницы → тема, заметка (≤3 предложения), теги (≤5), приоритет
- Батчинг: обрабатывать вкладки группами по 10 для экономии токенов
- Fallback: если API недоступен — сохранить вкладку со статусом `new`, повторить при следующем запуске

---

## Браузерное расширение — правила

- Manifest V3 (работает в Edge, Chrome, Firefox с минимальными правками)
- Разрешения: `tabs`, `storage`, `bookmarks` (только необходимые)
- Связь с desktop app: `native messaging` через Tauri native host
- Формат передачи вкладок: JSON массив объектов `Tab`
- Расширение не хранит данные постоянно — только передаёт и читает статус

---

## Система "Чистилище" (еженедельный опрос)

- Запускается по расписанию (cron через Tauri) или вручную
- За один сеанс: 10–20 вкладок (настраивается в Settings)
- Интерфейс: одна вкладка на экране → кнопки Оставить / Удалить / Позже
- Решение записывается в таблицу `reviews` с датой
- "Удалить" = статус `deleted` в БД + запрос расширению закрыть вкладку
- История всех решений сохраняется (никогда не удаляется)

---

## Экспорт в браузер (папки закладок)

- Структура: `MindCapture > {collection_name} > {tab_title}`
- Экспорт через Bookmarks API расширения
- Синхронизация: только изменения с последнего экспорта (delta export)
- Формат также: HTML файл закладок (универсальный импорт в любой браузер)

---

## Правила разработки

1. Всё хранится локально. Никаких внешних серверов кроме Claude API.
2. API ключ — только через Tauri secure store. Никогда не хардкодить.
3. Валидировать входящие данные на границе: при импорте из расширения.
4. Небольшие сфокусированные PR — один спринт, одна задача.
5. После каждого спринта — верификация (см. QA чек-лист в plan.html).
6. По завершении спринта обновить в claude.md:
   - Раздел Dependencies (npm list --depth=0)
   - Раздел Completed Sprints
   - Ссылки на ключевые файлы если изменились

---

## Заложено в MVP, разрабатывается позже (V2+)

| Фича | Статус | Описание |
|---|---|---|
| YouTube API | V2 | История просмотров + AI резюме видео |
| RuTube | V2 | Парсинг (нет открытого API) |
| VK API | V2 | Сохранённые посты |
| Instagram API | V2 | Сохранённые посты |
| Граф знаний | V2 | Визуализация связей между заметками |
| Мобильное приложение | V3 | iOS/Android синхронизация |
| Edge Mobile Sync | V2 | Microsoft Sync API для мобильных вкладок |
| Еженедельный дайджест | V2 | AI-отчёт "что прошло мимо тебя" |
| Поведенческий профиль | V3 | AI анализирует паттерны: что важно для пользователя |

---

## Структура страниц (React)

| Страница | Путь | Описание |
|---|---|---|
| Inbox | `/` | Все новые импортированные вкладки |
| Library | `/library` | Организованные коллекции и заметки |
| Neglected | `/neglected` | Важное, но непрочитанное (AI-пометка) |
| Purgatory | `/purgatory` | Интерфейс еженедельного опроса |
| Settings | `/settings` | API ключ, браузеры, расписание |
