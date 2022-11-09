use crate::model::cpu::CpuUsage;
use crate::model::disk::DiskDetail;
use crate::model::network::NetworkDetail;
use crate::model::process::Process;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct RealtimeStatus {
    pub cpu: Vec<CpuUsage>,
    pub load_avg: Vec<f64>,
    pub process: Vec<Process>,
    pub network: Vec<NetworkDetail>,
    pub disk: Vec<DiskDetail>,
    pub uptime: Vec<u64>,
}