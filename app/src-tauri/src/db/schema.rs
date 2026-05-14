pub const CREATE_TABLES: &str = "
CREATE TABLE IF NOT EXISTS tabs (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    url         TEXT NOT NULL,
    title       TEXT NOT NULL,
    favicon     TEXT,
    browser     TEXT NOT NULL,
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    status      TEXT NOT NULL DEFAULT 'new'
        CHECK (status IN ('new', 'analyzed', 'reviewed', 'exported', 'deleted'))
);

CREATE TABLE IF NOT EXISTS notes (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    tab_id     INTEGER NOT NULL REFERENCES tabs(id) ON DELETE CASCADE,
    content    TEXT NOT NULL,
    tags       TEXT NOT NULL DEFAULT '[]',
    priority   INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS collections (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL UNIQUE,
    color      TEXT,
    icon       TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS tab_collections (
    tab_id        INTEGER NOT NULL REFERENCES tabs(id) ON DELETE CASCADE,
    collection_id INTEGER NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    PRIMARY KEY (tab_id, collection_id)
);

CREATE TABLE IF NOT EXISTS reviews (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    tab_id      INTEGER NOT NULL REFERENCES tabs(id) ON DELETE CASCADE,
    decision    TEXT NOT NULL
        CHECK (decision IN ('keep', 'delete', 'later')),
    reviewed_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS sync_log (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    action    TEXT NOT NULL,
    entity_id INTEGER NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);
";

pub const CREATE_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_tabs_status ON tabs(status);",
    "CREATE INDEX IF NOT EXISTS idx_tabs_imported_at ON tabs(imported_at);",
    "CREATE INDEX IF NOT EXISTS idx_notes_tab_id ON notes(tab_id);",
    "CREATE INDEX IF NOT EXISTS idx_reviews_tab_id ON reviews(tab_id);",
    "CREATE INDEX IF NOT EXISTS idx_sync_log_timestamp ON sync_log(timestamp);",
];
