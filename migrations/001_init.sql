CREATE TABLE IF NOT EXISTS charges (

    id TEXT PRIMARY KEY,

    memo TEXT NOT NULL UNIQUE,

    amount REAL NOT NULL,

    asset TEXT NOT NULL DEFAULT 'XLM',

    status TEXT NOT NULL DEFAULT 'pending',

    tx_hash TEXT,

    created_at TEXT NOT NULL

);
