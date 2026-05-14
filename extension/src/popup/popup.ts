interface TabInfo {
  url: string;
  title: string;
  favicon: string | null;
  browser: string;
}

interface ImportResponse {
  imported: number;
  duplicates: number;
}

function getBrowser(): string {
  const ua = navigator.userAgent;
  if (ua.includes("Edg")) return "Edge";
  if (ua.includes("Chrome")) return "Chrome";
  if (ua.includes("Firefox")) return "Firefox";
  return "Unknown";
}

interface Bookmark {
  title: string;
  url: string;
}

interface BookmarkFolder {
  name: string;
  bookmarks: Bookmark[];
}

interface ExportPayload {
  folders: BookmarkFolder[];
  total: number;
  delta: boolean;
}

document.getElementById("import-btn")?.addEventListener("click", async () => {
  const status = document.getElementById("status")!;
  status.textContent = "Получение закладок...";

  try {
    const res = await fetch("http://127.0.0.1:1422/export-payload");
    if (!res.ok) {
      throw new Error(`HTTP ${res.status}`);
    }

    const payload: ExportPayload = await res.json();

    if (payload.total === 0) {
      status.textContent = "Нет закладок для импорта.";
      return;
    }

    status.textContent = `Импорт ${payload.total} закладок...`;

    const mindCapture = await chrome.bookmarks.create({
      title: `MindCapture ${payload.delta ? "(новое)" : ""}`,
    });

    let imported = 0;
    for (const folder of payload.folders) {
      const folderNode = await chrome.bookmarks.create({
        parentId: mindCapture.id,
        title: folder.name,
      });

      for (const bm of folder.bookmarks) {
        await chrome.bookmarks.create({
          parentId: folderNode.id,
          title: bm.title,
          url: bm.url,
        });
        imported++;
      }
    }

    status.textContent = `Готово! Импортировано ${imported} закладок.`;
  } catch (e) {
    if (e instanceof TypeError && e.message.includes("fetch")) {
      status.textContent = "Ошибка: приложение MindCapture не запущено.";
    } else {
      status.textContent = "Error: " + (e instanceof Error ? e.message : String(e));
    }
  }
});

document.getElementById("capture-btn")?.addEventListener("click", async () => {
  const status = document.getElementById("status")!;
  status.textContent = "Сбор вкладок...";

  try {
    const browserTabs = await chrome.tabs.query({});
    const tabs: TabInfo[] = browserTabs
      .filter((t) => t.url && t.title)
      .map((t) => ({
        url: t.url!,
        title: t.title!,
        favicon: t.favIconUrl ?? null,
        browser: getBrowser(),
      }));

    status.textContent = `Отправка ${tabs.length} вкладок...`;

    const res = await fetch("http://127.0.0.1:1422/import", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(tabs),
    });

    if (!res.ok) {
      const err = await res.json();
      throw new Error(err.error ?? `HTTP ${res.status}`);
    }

    const data: ImportResponse = await res.json();
    status.textContent = `Готово! ${data.imported} импортировано, ${data.duplicates} уже известно.`;
  } catch (e) {
    if (e instanceof TypeError && e.message.includes("fetch")) {
      status.textContent = "Ошибка: приложение MindCapture не запущено. Сначала запустите десктопное приложение.";
    } else {
      status.textContent = "Error: " + (e instanceof Error ? e.message : String(e));
    }
  }
});
