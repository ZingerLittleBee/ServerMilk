use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct User {
    pub uid: String,
    pub gid: String,
    pub name: String,
    pub groups: Vec<String>,
}
