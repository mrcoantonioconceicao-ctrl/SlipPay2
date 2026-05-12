use hmac::{Hmac, Mac};
use sha2::Sha256;

use serde::Serialize;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize)]
pub struct WebhookPayload {
    pub event: String,
    pub charge_id: String,
    pub tx_hash: String,
}

pub fn sign_payload(
    secret: &str,
    payload: &str,
) -> String {

    let mut mac =
        HmacSha256::new_from_slice(
            secret.as_bytes()
        )
        .expect("invalid hmac key");

    mac.update(payload.as_bytes());

    let result = mac.finalize();

    let bytes = result.into_bytes();

    hex::encode(bytes)
}

pub async fn send_webhook(
    url: &str,
    secret: &str,
    payload: &WebhookPayload,
) -> Result<(), String> {

    let body =
        serde_json::to_string(payload)
            .map_err(|e| e.to_string())?;

    let signature =
        sign_payload(
            secret,
            &body,
        );

    let client =
        reqwest::Client::new();

    let response =
        client
            .post(url)
            .header(
                "x-slippay-signature",
                signature,
            )
            .header(
                "content-type",
                "application/json",
            )
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

    if !response.status().is_success() {

        return Err(format!(
            "webhook failed: {}",
            response.status()
        ));
    }

    Ok(())
}
