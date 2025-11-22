use reqwest::StatusCode;
use serde_json::json;

const BASE_URL: &str = "http://localhost:8000";

#[tokio::test]
async fn test_list_hotels_200_ok() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/hotels", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_hotel_200_ok() {
    let client = reqwest::Client::new();
    let email = format!(
        "hotel_creator_{}@example.com",
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
    let create_payload = json!({
        "name": "Test Hotel",
        "description": "A test hotel",
        "address": "123 Main St",
        "city": "New York",
        "country": "USA"
    });
    let create_resp = client
        .post(&format!("{}/hotels", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create hotel");
    let create_body = create_resp.json::<serde_json::Value>().await.unwrap();
    let hotel_id = create_body["id"].as_i64().unwrap();

    let response = client
        .get(&format!("{}/hotels/{}", BASE_URL, hotel_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["id"], hotel_id);
    assert_eq!(body["name"], "Test Hotel");
}

#[tokio::test]
async fn test_get_hotel_404_not_found() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/hotels/999999", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.text().await.unwrap();
    assert!(body.contains("not found") || body.contains("Hotel"));
}

#[tokio::test]
async fn test_create_hotel_201_created() {
    let client = reqwest::Client::new();
    let email = format!(
        "creator_{}@example.com",
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
    let create_payload = json!({
        "name": "Luxury Hotel",
        "description": "A luxury 5-star hotel",
        "address": "456 Oak Ave",
        "city": "Los Angeles",
        "country": "USA"
    });

    let response = client
        .post(&format!("{}/hotels", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert!(body.get("id").is_some());
    assert_eq!(body["name"], "Luxury Hotel");
}

#[tokio::test]
async fn test_create_hotel_422_invalid_input() {
    let client = reqwest::Client::new();
    let email = format!(
        "creator2_{}@example.com",
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
    let create_payload = json!({"name": "Hotel"});

    let response = client
        .post(&format!("{}/hotels", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_create_hotel_401_unauthorized() {
    let client = reqwest::Client::new();
    let create_payload = json!({
        "name": "Luxury Hotel",
        "description": "A luxury 5-star hotel",
        "address": "456 Oak Ave",
        "city": "Los Angeles",
        "country": "USA"
    });

    let response = client
        .post(&format!("{}/hotels", BASE_URL))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_hotel_401_invalid_token() {
    let client = reqwest::Client::new();
    let create_payload = json!({
        "name": "Luxury Hotel",
        "description": "A luxury 5-star hotel",
        "address": "456 Oak Ave",
        "city": "Los Angeles",
        "country": "USA"
    });

    let response = client
        .post(&format!("{}/hotels", BASE_URL))
        .header("Authorization", "Bearer invalid_token")
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_hotel_200_ok() {
    let client = reqwest::Client::new();
    let email = format!(
        "updater_{}@example.com",
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
    let create_payload = json!({
        "name": "Original Hotel",
        "description": "Original description",
        "address": "123 Main St",
        "city": "Boston",
        "country": "USA"
    });
    let create_resp = client
        .post(&format!("{}/hotels", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create hotel");
    let create_body = create_resp.json::<serde_json::Value>().await.unwrap();
    let hotel_id = create_body["id"].as_i64().unwrap();
    let update_payload = json!({
        "name": "Updated Hotel",
        "description": "Updated description",
        "address": "456 Elm St",
        "city": "New York",
        "country": "USA"
    });

    let response = client
        .put(&format!("{}/hotels/{}", BASE_URL, hotel_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["name"], "Updated Hotel");
}

#[tokio::test]
async fn test_update_hotel_401_unauthorized() {
    let client = reqwest::Client::new();
    let update_payload = json!({
        "name": "Updated Hotel",
        "description": "Updated description"
    });

    let response = client
        .put(&format!("{}/hotels/1", BASE_URL))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_hotel_401_invalid_token() {
    let client = reqwest::Client::new();
    let update_payload = json!({
        "name": "Updated Hotel",
        "description": "Updated description"
    });

    let response = client
        .put(&format!("{}/hotels/1", BASE_URL))
        .header("Authorization", "Bearer invalid_token")
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_hotel_404_not_found() {
    let client = reqwest::Client::new();
    let email = format!(
        "updater2_{}@example.com",
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
        "name": "Updated Hotel"
    });

    let response = client
        .put(&format!("{}/hotels/999999", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.text().await.unwrap();
    assert!(body.contains("not found") || body.contains("Hotel"));
}

#[tokio::test]
async fn test_delete_hotel_204_no_content() {
    let client = reqwest::Client::new();
    let email = format!(
        "deleter_{}@example.com",
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
    let create_payload = json!({
        "name": "Hotel to Delete",
        "address": "789 Pine St",
        "city": "Chicago",
        "country": "USA"
    });
    let create_resp = client
        .post(&format!("{}/hotels", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create hotel");
    let create_body = create_resp.json::<serde_json::Value>().await.unwrap();
    let hotel_id = create_body["id"].as_i64().unwrap();

    let response = client
        .delete(&format!("{}/hotels/{}", BASE_URL, hotel_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_delete_hotel_401_unauthorized() {
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("{}/hotels/1", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_hotel_401_invalid_token() {
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("{}/hotels/1", BASE_URL))
        .header("Authorization", "Bearer invalid_token")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_hotel_404_not_found() {
    let client = reqwest::Client::new();
    let email = format!(
        "deleter2_{}@example.com",
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
        .delete(&format!("{}/hotels/999999", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.text().await.unwrap();
    assert!(body.contains("not found") || body.contains("Hotel"));
}
