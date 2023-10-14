use std::sync::{Arc, RwLock};
use std::time::Duration;
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
pub async fn fetch_token(state: tauri::State<'_, Arc<RwLock<SidecarState>>>) -> Result<String, String> {
    
    let port = {
        let state = state.try_read();
        if let Ok(state) = state {
            state.get_port()
        } else {
            return Err("failed to fetch token".into());
        }
    };

    
    let url = format!("http://localhost:{}/local/config/app", port);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let resp = client
        .get(url)
        .send()
        .await;

    return  match resp {
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
