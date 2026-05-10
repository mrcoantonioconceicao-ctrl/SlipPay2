use rusqlite::{params, Connection, Result};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, PartialEq)]
pub struct Charge {
    pub id: i32,
    pub amount: Decimal,
    pub status: String,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS charges (
            id      INTEGER PRIMARY KEY,
            amount  TEXT NOT NULL,
            status  TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_charge(conn: &Connection, charge: &Charge) -> Result<()> {
    conn.execute(
        "INSERT INTO charges (id, amount, status) VALUES (?1, ?2, ?3)",
        params![charge.id, charge.amount.to_string(), charge.status],
    )?;
    Ok(())
}

pub fn get_charge(conn: &Connection, id: i32) -> Result<Charge> {
    conn.query_row(
        "SELECT id, amount, status FROM charges WHERE id = ?1",
        params![id],
        |row| {
            let amount_str: String = row.get(1)?;
            Ok(Charge {
                id: row.get(0)?,
                amount: amount_str.parse::<Decimal>().unwrap(),
                status: row.get(2)?,
            })
        },
    )
}
