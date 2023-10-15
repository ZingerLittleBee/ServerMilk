use log::warn;
use serde_json::json;
use tauri::api::process::CommandChild;
use tauri::Wry;
use tauri_plugin_store::Store;
use crate::constant;

#[derive(Default)]
pub struct SidecarState {
    child: Option<CommandChild>,
    store: Option<Store<Wry>>,
    port: Option<u16>,
}

impl SidecarState {
    pub fn set_store(&mut self, store: Store<Wry>) {
        self.store = Some(store);
    }

    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or_else(|| {
            let store = self.store.as_ref().unwrap();
            store
                .get("port")
                .map(|v| v.as_u64().unwrap() as u16)
                .unwrap_or(constant::DEFAULT_PORT)
        })
    }

    pub fn set_port(&mut self, port: u16) -> anyhow::Result<bool> {
        self.port = Some(port);

        if let Some(store) = self.store.as_mut() {
            match store.insert("port".into(), json!(port)) {
                Ok(_) => {
                    match store.save() {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("failed to save store: {}", e);
                        }
                    }
                    Ok(true)
                },
                Err(e) => {
                    warn!("failed to set port: {}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }

    }

    pub fn set_child(&mut self, child: CommandChild) {
        self.child = Some(child);
    }

    pub fn kill_sidecar(&mut self) -> bool {
        if let Some(child) = self.child.take() {
            child.kill().is_ok()
        } else {
            false
        }
    }

    pub fn get_pid(&self) -> u32 {
        if let Some(child) = &self.child {
            child.pid()
        } else {
            0
        }
    }
}
