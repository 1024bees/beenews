use std::net::TcpListener;
use zero2bees::run;
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind to ephermal port");
    let port = listener.local_addr().expect("Malformed address").port();
    let server = run(listener);
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_sanity() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Request failed");
    assert!(resp.status().is_success());
    assert_eq!(resp.content_length(), Some(0));
}

#[tokio::test]
/// Tests that subscribe should return a 200 response for valid form data
async fn subscribe_valid_case() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
/// Tests that subscribe returns a 400 response when data is missing
async fn subscribe_invalid_data_missing() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 422 Bad Request when the payload was {}.",
            error_message
        );
    }
}
