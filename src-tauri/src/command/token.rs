use std::time::Duration;
use anyhow::anyhow;
use crate::SidecarState;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct ResponseData {
    token: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    success: bool,
    data: ResponseData,
}

#[tauri::command]
pub async fn fetch_token(state: tauri::State<'_, SidecarState>) -> Result<String, String> {
    let port = state.port.lock().unwrap().unwrap();

    let url = format!("http://localhost:{}/local/config/app", port);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let resp = client
        .get(url)
        .send()
        .await;

    match resp {
        Ok(resp) => {
            let api_resp: ApiResponse = resp.json().await.unwrap();
            if !api_resp.success {
                return Err("failed to fetch token".into());
            }
            Ok(api_resp.data.token)
        }
        Err(_) => {
            Err("failed to fetch token".into())
        }
    }
}
