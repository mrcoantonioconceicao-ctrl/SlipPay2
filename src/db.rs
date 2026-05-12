use rusqlite::{
    params,
    Connection,
    Result,
};

use rust_decimal::Decimal;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Charge {
    pub id: i32,
    pub charge_id: String,
    pub memo: String,
    pub brl_amount: String,
    pub status: String,
    pub tx_hash: Option<String>,
}

pub fn init_db(conn: &Connection) -> Result<()> {

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS charges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            charge_id TEXT NOT NULL UNIQUE,
            memo TEXT NOT NULL UNIQUE,
            brl_amount TEXT NOT NULL,
            status TEXT NOT NULL,
            tx_hash TEXT
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS payment_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            charge_id INTEGER NOT NULL,
            event_type TEXT NOT NULL,
            tx_hash TEXT,
            payload TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        ",
        [],
    )?;

    Ok(())
}

pub fn create_charge(
    conn: &Connection,
    brl_amount: Decimal,
    memo: &str,
    charge_id: &str,
) -> Result<()> {

    conn.execute(
        "
        INSERT INTO charges (
            charge_id,
            memo,
            brl_amount,
            status
        )
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![
            charge_id,
            memo,
            brl_amount.to_string(),
            "pending"
        ],
    )?;

    Ok(())
}

pub fn get_charge_by_memo(
    conn: &Connection,
    memo: &str,
) -> Result<Option<Charge>> {

    let mut stmt =
        conn.prepare(
            "
            SELECT
                id,
                charge_id,
                memo,
                brl_amount,
                status,
                tx_hash
            FROM charges
            WHERE memo = ?1
            "
        )?;

    let mut rows =
        stmt.query(params![memo])?;

    if let Some(row) = rows.next()? {

        Ok(Some(Charge {
            id: row.get(0)?,
            charge_id: row.get(1)?,
            memo: row.get(2)?,
            brl_amount: row.get(3)?,
            status: row.get(4)?,
            tx_hash: row.get(5)?,
        }))

    } else {

        Ok(None)

    }
}

pub fn get_charge_by_tx_hash(
    conn: &Connection,
    tx_hash: &str,
) -> Result<Option<Charge>> {

    let mut stmt =
        conn.prepare(
            "
            SELECT
                id,
                charge_id,
                memo,
                brl_amount,
                status,
                tx_hash
            FROM charges
            WHERE tx_hash = ?1
            "
        )?;

    let mut rows =
        stmt.query(params![tx_hash])?;

    if let Some(row) = rows.next()? {

        Ok(Some(Charge {
            id: row.get(0)?,
            charge_id: row.get(1)?,
            memo: row.get(2)?,
            brl_amount: row.get(3)?,
            status: row.get(4)?,
            tx_hash: row.get(5)?,
        }))

    } else {

        Ok(None)

    }
}

pub fn update_charge_status(
    conn: &Connection,
    charge_id: i32,
    status: &str,
    tx_hash: Option<&str>,
) -> Result<()> {

    conn.execute(
        "
        UPDATE charges
        SET
            status = ?1,
            tx_hash = ?2
        WHERE id = ?3
        ",
        params![
            status,
            tx_hash,
            charge_id
        ],
    )?;

    Ok(())
}

pub fn insert_payment_event(
    conn: &Connection,
    charge_id: i32,
    event_type: &str,
    tx_hash: Option<&str>,
    payload: Option<&str>,
) -> Result<()> {

    conn.execute(
        "
        INSERT INTO payment_events (
            charge_id,
            event_type,
            tx_hash,
            payload
        )
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![
            charge_id,
            event_type,
            tx_hash,
            payload
        ],
    )?;

    Ok(())
}
