use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSettings {
    pub(crate) theme: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) is_enable_auto_launch: Option<bool>,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            theme: None,
            port: Option::from(6000),
            is_enable_auto_launch: Option::from(true ),
        }
    }
}
