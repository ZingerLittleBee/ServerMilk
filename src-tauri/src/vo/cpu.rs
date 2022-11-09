use crate::model::cpu::{CpuInfo, CpuUsage};
use crate::vo::formator::Convert;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct CpuInfoVo {
    pub core_num: usize,
    pub brand: String,
    pub frequency: u64,
    pub vendor_id: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct CpuUsageVo {
    pub name: String,
    pub usage: f32,
}

impl Convert<CpuUsageVo> for CpuUsage {
    fn convert(&self) -> CpuUsageVo {
        CpuUsageVo {
            name: self.name.clone(),
            usage: self.cpu_usage.clone(),
        }
    }
}

impl Convert<CpuInfoVo> for CpuInfo {
    fn convert(&self) -> CpuInfoVo {
        CpuInfoVo {
            core_num: self.core_num,
            brand: self.brand.clone(),
            frequency: self.frequency,
            vendor_id: self.vendor_id.clone(),
        }
    }
}
