use reqwest::Client;

#[tokio::test]
async fn test_health_endpoint() {
    let resp = Client::new()
        .get("http://localhost:8080/health")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert_eq!(body, "SlipPay OK");
}

#[tokio::test]
async fn test_eval_endpoint() {
    let resp = Client::new()
        .post("http://localhost:8080/eval")
        .body("{\"expression\":\"2+2\"}")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}
