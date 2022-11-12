use std::time::Duration;

#[tauri::command]
pub async fn check_web_status(port: u16) -> bool {
    println!("Checking web status on port {}", port);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let resp = client
        .get(format!("http://127.0.0.1:{}", port))
        .send()
        .await;

    match resp {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}
