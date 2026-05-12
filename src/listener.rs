use std::time::Duration;

use rusqlite::Connection;

use tokio::time::sleep;

use crate::stellar::fetch_latest_payments;

use crate::reconciler::reconcile_payment;

pub async fn start_listener() {

    println!("================================");
    println!(" SlipPay Stellar Listener");
    println!("================================");

    loop {

        println!("checking stellar payments...");

        let payments =
            match fetch_latest_payments().await {

                Ok(p) => p,

                Err(err) => {

                    println!(
                        "stellar error: {}",
                        err
                    );

                    sleep(
                        Duration::from_secs(10)
                    ).await;

                    continue;
                }
            };

        for payment in payments {

            println!(
                "tx detected: {}",
                payment.tx_hash
            );

            let conn =
                match Connection::open(
                    "slippay.db"
                ) {

                    Ok(c) => c,

                    Err(err) => {

                        println!(
                            "db error: {}",
                            err
                        );

                        continue;
                    }
                };

            let result =
                reconcile_payment(
                    &conn,
                    &payment.tx_hash,
                ).await;

            match result {

                Ok(r) => {

                    println!(
                        "reconciled: {}",
                        r.message
                    );
                }

                Err(err) => {

                    println!(
                        "reconcile failed: {}",
                        err
                    );
                }
            }
        }

        sleep(
            Duration::from_secs(15)
        ).await;
    }
}
