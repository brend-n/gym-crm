use std::{collections::HashMap, net::TcpListener};

#[tokio::test]
async fn post_returns_200_for_valid_() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("firstname", "Bob");
    body.insert("surname", "Mortimer");
    // Act
    let response = client
        .post(address)
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn post_returns_400_for_invalid_() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("hello", "not valid json"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}",&address))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        // Assert
        assert_eq!(400, response.status().as_u16(), "The API did not failed with 400 when the payload was {}", error_message);
    }
}


fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = members::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
