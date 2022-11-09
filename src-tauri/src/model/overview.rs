use crate::model::cpu::CpuInfo;
use crate::model::disk::DiskIO;
use crate::model::network::NetworkIO;
use crate::model::usage::Usage;
use crate::model::user::User;
use serde::{Deserialize, Serialize};

use super::memory::MemoryUsage;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Overview {
    pub cpu_usage: f32,
    pub memory_usage: MemoryUsage,
    pub disk_usage: Usage,
    pub disk_io: DiskIO,
    pub network_io: NetworkIO,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct OsOverview {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_info: CpuInfo,
    pub users: Vec<User>,
    pub boot_time: u64,
}
