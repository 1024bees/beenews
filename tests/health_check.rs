use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2bees::configuration::get_configuration;
use zero2bees::startup::run;
async fn spawn_app() -> String {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind to ephermal port");
    let port = listener.local_addr().expect("Malformed address").port();
    let server = run(listener, connection);
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_sanity() {
    let addr = spawn_app().await;
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
    let addr = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

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
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
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
