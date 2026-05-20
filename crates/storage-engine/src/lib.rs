use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite, Row};
use std::str::FromStr;
use rust_decimal::Decimal;

pub struct Storage {
    pub pool: Pool<Sqlite>,
}

/// 📊 Estrutura pública que mapeia uma linha do banco para a auditoria
#[derive(serde::Serialize)]
pub struct TransactionRecord {
    pub id: i64,
    pub wallet_address: String,
    pub amount: Decimal,
    pub status: String,
    pub created_at: String,
}

impl Storage {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let db_options = SqliteConnectOptions::from_str(db_url)?
            .create_if_missing(true);
            
        let pool = SqlitePoolOptions::new()
            .connect_with(db_options)
            .await?;
            
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                wallet_address TEXT NOT NULL,
                amount TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );"
        )
        .execute(&self.pool)
        .await?;

        let count: i64 = sqlx::query("SELECT COUNT(*) FROM transactions")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        if count == 0 {
            sqlx::query("INSERT INTO transactions (wallet_address, amount, status) VALUES 
                ('G_CLEAN_STELLAR_WALLET', '1200.00', 'APPROVED'),
                ('G_CLEAN_STELLAR_WALLET', '1400.00', 'APPROVED'),
                ('G_CLEAN_STELLAR_WALLET', '1350.00', 'APPROVED')")
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    pub async fn get_history(&self, wallet: &str) -> anyhow::Result<Vec<Decimal>> {
        let rows = sqlx::query("SELECT amount FROM transactions WHERE wallet_address = ?")
            .bind(wallet)
            .fetch_all(&self.pool)
            .await?;
        
        let mut history = Vec::new();
        for row in rows {
            let val_str: String = row.get("amount");
            if let Ok(dec) = Decimal::from_str(&val_str) {
                history.push(dec);
            }
        }
        Ok(history)
    }

    /// 🔍 Busca TODAS as transações registradas para auditoria
    pub async fn get_all_transactions(&self) -> anyhow::Result<Vec<TransactionRecord>> {
        let rows = sqlx::query("SELECT id, wallet_address, amount, status, datetime(created_at, 'localtime') as created_at FROM transactions ORDER BY id DESC")
            .fetch_all(&self.pool)
            .await?;
            
        let mut records = Vec::new();
        for row in rows {
            let val_str: String = row.get("amount");
            let amount = Decimal::from_str(&val_str).unwrap_or_default();
            
            records.push(TransactionRecord {
                id: row.get("id"),
                wallet_address: row.get("wallet_address"),
                amount,
                status: row.get("status"),
                created_at: row.get("created_at"),
            });
        }
        Ok(records)
    }

    pub async fn save_transaction(&self, wallet: &str, amount: Decimal, status: &str) -> anyhow::Result<()> {
        let amount_str = amount.to_string();
        sqlx::query("INSERT INTO transactions (wallet_address, amount, status) VALUES (?, ?, ?)")
            .bind(wallet)
            .bind(amount_str)
            .bind(status)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

