use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {

    let pool = SqlitePool::connect("sqlite:slippay.db")
        .await
        .expect("erro conectando sqlite");

    // Tabela de pagamentos recebidos (listener)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS payments (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            tx_hash    TEXT UNIQUE,
            amount     TEXT,
            asset_type TEXT,
            sender     TEXT,
            receiver   TEXT,
            created_at TEXT
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("erro criando tabela payments");

    // Tabela de cobranças criadas via API
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS charges (
            id         TEXT PRIMARY KEY,
            memo       TEXT UNIQUE,
            amount     TEXT,
            asset      TEXT,
            status     TEXT DEFAULT 'pending',
            created_at TEXT,
            tx_hash    TEXT
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("erro criando tabela charges");

    println!("✅ Banco inicializado");

    pool
}

