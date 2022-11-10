

#[tokio::test]
async fn health_check_works() {
    // Arrange: spin up app instance
    spawn_app();

    // Need reqwest to peform HTTP request against application
    let client = reqwest::Client::new();

    // Act : make HTTP request against app
    let response  = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");


    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

// Launch application in the background
fn spawn_app() {
    let server = rnewsletter_api::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}