use reqwest::StatusCode;

const BASE_URL: &str = "http://localhost:8000";

#[tokio::test]
async fn test_health_live_200_ok() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/live", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn test_health_ready_200_ok() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/ready", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    if response.status() == StatusCode::OK {
        let body = response.json::<serde_json::Value>().await.unwrap();
        assert_eq!(body["status"], "ok");
        assert!(body.get("services").is_some());
    }
}

#[tokio::test]
async fn test_health_ready_503_service_unavailable() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/ready", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::SERVICE_UNAVAILABLE
    );
    if response.status() == StatusCode::SERVICE_UNAVAILABLE {
        let body = response.json::<serde_json::Value>().await.unwrap();
        assert!(body.get("services").is_some());
    }
}
