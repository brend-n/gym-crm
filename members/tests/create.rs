use std::collections::HashMap;

mod helpers;

#[tokio::test]
async fn post_returns_200_for_valid_() {
    // Arrange
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("firstname", "Bob");
    body.insert("surname", "Mortimer");

    // Act
    let response = client
        .post(app.address)
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    let saved = sqlx::query!("SELECT firstname, surname FROM members;",)
    .fetch_one(&app.db_pool)
    .await
    .expect("failed to fetch saved members");

    assert_eq!(saved.firstname, Some("Bob".to_string()));
    assert_eq!(saved.surname, Some("Mortimer".to_string()));

}

#[tokio::test]
async fn post_returns_400_for_invalid_() {
    // Arrange
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("hello", "not valid json"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}",&app.address))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        // Assert
        assert_eq!(400, response.status().as_u16(), "The API did not failed with 400 when the payload was {}", error_message);
    }
}