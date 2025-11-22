use reqwest::StatusCode;
use serde_json::json;

const BASE_URL: &str = "http://localhost:8000";

#[tokio::test]
async fn test_register_201_created() {
    let client = reqwest::Client::new();
    let payload = json!({
        "email": format!("user_{}@example.com", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()),
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });

    let response = client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert!(body["user"].get("id").is_some());
    assert!(body["user"].get("email").is_some());
    assert!(body.get("token").is_some());
}

#[tokio::test]
async fn test_register_400_invalid_email() {
    let client = reqwest::Client::new();
    let payload = json!({
        "email": "invalid-email",
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });

    let response = client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.text().await.unwrap();
    assert!(body.contains("Invalid email") || body.contains("email"));
}

#[tokio::test]
async fn test_register_400_short_password() {
    let client = reqwest::Client::new();
    let payload = json!({
        "email": format!("user_{}@example.com", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()),
        "password": "123",
        "first_name": "John",
        "last_name": "Doe"
    });

    let response = client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.text().await.unwrap();
    assert!(body.contains("Password") || body.contains("password") || body.contains("6"));
}

#[tokio::test]
async fn test_register_400_email_already_exists() {
    let client = reqwest::Client::new();
    let email = format!(
        "duplicate_{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let payload1 = json!({
        "email": email.clone(),
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });
    client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&payload1)
        .send()
        .await
        .expect("Failed to send first request");
    let payload2 = json!({
        "email": email,
        "password": "password456",
        "first_name": "Jane",
        "last_name": "Smith"
    });

    let response = client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&payload2)
        .send()
        .await
        .expect("Failed to send second request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.text().await.unwrap();
    assert!(body.contains("already exists") || body.contains("Email"));
}

#[tokio::test]
async fn test_login_200_ok() {
    let client = reqwest::Client::new();
    let email = format!(
        "login_{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let register_payload = json!({
        "email": email.clone(),
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });
    client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to register");
    let login_payload = json!({
        "email": email,
        "password": "password123"
    });

    let response = client
        .post(&format!("{}/auth/login", BASE_URL))
        .json(&login_payload)
        .send()
        .await
        .expect("Failed to login");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert!(body["user"].get("id").is_some());
    assert!(body.get("token").is_some());
}

#[tokio::test]
async fn test_login_400_invalid_email() {
    let client = reqwest::Client::new();
    let payload = json!({
        "email": "nonexistent@example.com",
        "password": "password123"
    });

    let response = client
        .post(&format!("{}/auth/login", BASE_URL))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.text().await.unwrap();
    assert!(body.contains("Invalid") || body.contains("password"));
}

#[tokio::test]
async fn test_login_400_invalid_password() {
    let client = reqwest::Client::new();
    let email = format!(
        "login2_{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let register_payload = json!({
        "email": email.clone(),
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });
    client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to register");
    let login_payload = json!({
        "email": email,
        "password": "wrongpassword"
    });

    let response = client
        .post(&format!("{}/auth/login", BASE_URL))
        .json(&login_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.text().await.unwrap();
    assert!(body.contains("Invalid") || body.contains("password"));
}

#[tokio::test]
async fn test_profile_200_ok() {
    let client = reqwest::Client::new();
    let email = format!(
        "profile_{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let register_payload = json!({
        "email": email.clone(),
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });
    let register_resp = client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to register");
    let register_body = register_resp.json::<serde_json::Value>().await.unwrap();
    let token = register_body["token"].as_str().unwrap();

    let response = client
        .get(&format!("{}/auth/profile", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["email"], email);
    assert!(body.get("id").is_some());
}

#[tokio::test]
async fn test_profile_401_unauthorized_missing_token() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/auth/profile", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response.text().await.unwrap();
    assert!(body.contains("Missing") || body.contains("token") || body.contains("Unauthorized"));
}

#[tokio::test]
async fn test_profile_401_unauthorized_invalid_token() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/auth/profile", BASE_URL))
        .header("Authorization", "Bearer invalid_token_here")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response.text().await.unwrap();
    assert!(body.contains("Invalid") || body.contains("token") || body.contains("Unauthorized"));
}

#[tokio::test]
async fn test_profile_401_unauthorized_malformed_header() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/auth/profile", BASE_URL))
        .header("Authorization", "invalid_header")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_profile_200_ok() {
    let client = reqwest::Client::new();
    let email = format!(
        "update_{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let register_payload = json!({
        "email": email.clone(),
        "password": "password123",
        "first_name": "John",
        "last_name": "Doe"
    });
    let register_resp = client
        .post(&format!("{}/auth/register", BASE_URL))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to register");

    let register_body = register_resp.json::<serde_json::Value>().await.unwrap();
    let token = register_body["token"].as_str().unwrap();
    let update_payload = json!({
        "first_name": "Jane",
        "last_name": "Smith"
    });

    let response = client
        .put(&format!("{}/auth/profile", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["first_name"], "Jane");
    assert_eq!(body["last_name"], "Smith");
}

#[tokio::test]
async fn test_update_profile_401_unauthorized() {
    let client = reqwest::Client::new();
    let update_payload = json!({
        "first_name": "Jane",
        "last_name": "Smith"
    });

    let response = client
        .put(&format!("{}/auth/profile", BASE_URL))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_profile_401_invalid_token() {
    let client = reqwest::Client::new();
    let update_payload = json!({
        "first_name": "Jane",
        "last_name": "Smith"
    });

    let response = client
        .put(&format!("{}/auth/profile", BASE_URL))
        .header("Authorization", "Bearer invalid_token")
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
