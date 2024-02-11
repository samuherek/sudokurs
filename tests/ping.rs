use sudokurs::server::spawn_app;

#[tokio::test]
async fn ping() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client.get(&format!("{}/api", app))
        .send()
        .await
        .expect("Failed to execute reqwest.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
