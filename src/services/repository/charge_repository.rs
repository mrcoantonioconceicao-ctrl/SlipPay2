use sqlx::SqlitePool;
use sqlx::Row;

pub async fn match_payment_to_charge(
    pool: &SqlitePool,
    memo: &str,
    tx_hash: &str,
    amount: &str,
) -> Result<bool, sqlx::Error> {
    // 1. Procura a charge com esse memo
    let charge_row = sqlx::query(
        "SELECT id, amount, status FROM charges WHERE memo = ?"
    )
    .bind(memo)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = charge_row {
        let charge_id: String = row.get(0);
        let charge_amount: String = row.get(1);
        let charge_status: String = row.get(2);

        // 2. Verifica se o valor está correto e está pendente
        if charge_amount == amount && charge_status == "pending" {
            // 3. Atualiza a charge como paga
            sqlx::query(
                "UPDATE charges SET status = ?, tx_hash = ? WHERE id = ?"
            )
            .bind("paid")
            .bind(tx_hash)
            .bind(&charge_id)
            .execute(pool)
            .await?;

            println!("✅ Charge {} marcada como paga!", charge_id);
            return Ok(true);
        }
    }

    Ok(false)
}
