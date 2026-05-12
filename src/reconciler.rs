use serde::Serialize;

use rusqlite::Connection;

use crate::stellar::fetch_transaction;

use crate::webhook::{
    send_webhook,
    WebhookPayload,
};

use crate::db::{
    get_charge_by_tx_hash,
    get_charge_by_memo,
    update_charge_status,
    insert_payment_event,
};

#[derive(Serialize)]
pub struct ReconcileResult {
    pub success: bool,
    pub message: String,
}

pub async fn reconcile_payment(
    conn: &Connection,
    tx_hash: &str,
) -> Result<ReconcileResult, String> {

    /*
        IDEMPOTÊNCIA
    */

    match get_charge_by_tx_hash(
        conn,
        tx_hash,
    ) {

        Ok(Some(_)) => {

            return Ok(
                ReconcileResult {
                    success: true,
                    message:
                        "payment already processed"
                            .to_string(),
                }
            );
        }

        Ok(None) => {}

        Err(err) => {

            return Err(
                format!(
                    "db error: {}",
                    err
                )
            );
        }
    }

    /*
        Busca transação Stellar
    */

    let tx =
        fetch_transaction(tx_hash)
            .await?;

    if !tx.successful {

        return Err(
            "transaction failed"
                .to_string()
        );
    }

    /*
        Memo on-chain
    */

    let memo = tx.memo;

    /*
        Busca cobrança
    */

    let charge =
        match get_charge_by_memo(
            conn,
            &memo,
        ) {

            Ok(Some(charge)) => charge,

            Ok(None) => {

                return Err(
                    "charge not found"
                        .to_string()
                );
            }

            Err(err) => {

                return Err(
                    format!(
                        "db error: {}",
                        err
                    )
                );
            }
        };

    /*
        Atualiza status
    */

    update_charge_status(
        conn,
        charge.id,
        "paid",
        Some(tx_hash),
    ).map_err(|e| e.to_string())?;

    /*
        Persistência de evento
    */

    insert_payment_event(
        conn,
        charge.id,
        "payment_confirmed",
        Some(tx_hash),
        None,
    ).map_err(|e| e.to_string())?;

    /*
        WEBHOOK
    */

    let payload =
        WebhookPayload {

            event:
                "payment.confirmed"
                    .to_string(),

            charge_id:
                charge.charge_id.clone(),

            tx_hash:
                tx_hash.to_string(),
        };

    let _ =
        send_webhook(
            "https://webhook.site/test",
            "slippay_secret",
            &payload,
        ).await;

    Ok(
        ReconcileResult {
            success: true,
            message:
                "payment confirmed"
                    .to_string(),
        }
    )
}
