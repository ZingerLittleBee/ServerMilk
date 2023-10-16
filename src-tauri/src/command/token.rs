use crate::utils::get_port_from_state;
use crate::SidecarState;
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct ResponseData {
    token: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    success: bool,
    data: Option<ResponseData>,
}

#[tauri::command]
pub async fn fetch_token(
    state: tauri::State<'_, Arc<RwLock<SidecarState>>>,
) -> Result<String, String> {
    let port = match get_port_from_state(state.clone()) {
        Ok(port) => port,
        Err(_) => {
            return Err("failed to fetch token".into());
        }
    };

    let url = format!("http://localhost:{}/local/config/app", port);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let resp = client.get(url).send().await;

    match resp {
        Ok(resp) => {
            let api_resp: ApiResponse = resp.json().await.unwrap();
            if !api_resp.success {
                return Err("failed to fetch token".into());
            }
            if api_resp.data.is_some() {
                Ok(api_resp.data.unwrap().token)
            } else {
                Ok("".into())
            }
        }
        Err(_) => Err("failed to fetch token".into()),
    }
}

#[tauri::command]
pub async fn set_token(
    state: tauri::State<'_, Arc<RwLock<SidecarState>>>,
    token: String,
) -> Result<bool, String> {
    let port = match get_port_from_state(state.clone()) {
        Ok(port) => port,
        Err(_) => {
            return Err("failed to fetch token".into());
        }
    };

    let url = format!("http://localhost:{}/local/config/app", port);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let resp = client
        .post(url)
        .json(&json!({
            "token": token
        }))
        .send()
        .await;

    match resp {
        Ok(resp) => {
            let api_resp: ApiResponse = resp.json().await.unwrap();
            Ok(api_resp.success)
        }
        Err(_) => Err("failed to fetch token".into()),
    }
}
