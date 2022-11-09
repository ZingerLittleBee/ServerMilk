use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Usage {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}
