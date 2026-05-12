CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    memo TEXT NOT NULL UNIQUE,
    amount_brl REAL NOT NULL,
    amount_xlm REAL NOT NULL,
    destination TEXT NOT NULL,
    tx_hash TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    confirmed_at TEXT
);

CREATE TABLE IF NOT EXISTS payment_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    payment_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT,
    created_at TEXT NOT NULL
);
