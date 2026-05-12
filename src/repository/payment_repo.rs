use rusqlite::{params, Connection, Result};

use crate::models::Payment;

pub fn create_payment(conn: &Connection, payment: &Payment) -> Result<()> {
    conn.execute(
        "
        INSERT INTO payments (
            id,
            memo,
            amount_brl,
            amount_xlm,
            destination,
            tx_hash,
            status,
            created_at,
            confirmed_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ",
        params![
            payment.id,
            payment.memo,
            payment.amount_brl,
            payment.amount_xlm,
            payment.destination,
            payment.tx_hash,
            payment.status,
            payment.created_at,
            payment.confirmed_at
        ],
    )?;

    Ok(())
}
