use rusqlite::{params, Connection, Result};
use crate::repository::models::{Payment, PaymentEvent};

pub struct PaymentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PaymentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        PaymentRepository { conn }
    }

    pub fn insert_payment(&self, payment: &Payment) -> Result<()> {
        self.conn.execute(
            "INSERT INTO payments (id, memo, amount_brl, amount_xlm, destination, tx_hash, status, created_at, confirmed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                payment.id,
                payment.memo,
                payment.amount_brl,
                payment.amount_xlm,
                payment.destination,
                payment.tx_hash,
                payment.status,
                payment.created_at,
                payment.confirmed_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_payment(&self, id: &str) -> Result<Option<Payment>> {
        let mut stmt = self.conn.prepare("SELECT id, memo, amount_brl, amount_xlm, destination, tx_hash, status, created_at, confirmed_at FROM payments WHERE id=?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Payment {
                id: row.get(0)?,
                memo: row.get(1)?,
                amount_brl: row.get(2)?,
                amount_xlm: row.get(3)?,
                destination: row.get(4)?,
                tx_hash: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                confirmed_at: row.get(8)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn insert_event(&self, event: &PaymentEvent) -> Result<()> {
        self.conn.execute(
            "INSERT INTO payment_events (payment_id, event_type, payload, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                event.payment_id,
                event.event_type,
                event.payload,
                event.created_at,
            ],
        )?;
        Ok(())
    }
}
