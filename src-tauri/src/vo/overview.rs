use crate::model::overview::{OsOverview, Overview};
use crate::vo::cpu::CpuInfoVo;
use crate::vo::disk::DiskIOVo;
use crate::vo::formator::Convert;
use crate::vo::memory::MemUsageVo;
use crate::vo::network::NetworkIOVo;
use crate::vo::usage::UsageVo;
use crate::vo::user::UserVo;
use bytestring::ByteString;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct OverviewVo {
    pub cpu_usage: String,
    pub memory_usage: MemUsageVo,
    pub disk_usage: UsageVo,
    pub disk_io: DiskIOVo,
    pub network_io: NetworkIOVo,
}

impl From<OverviewVo> for ByteString {
    fn from(overview: OverviewVo) -> Self {
        serde_json::to_string(&overview).unwrap().into()
    }
}

impl Convert<OverviewVo> for Overview {
    fn convert(&self) -> OverviewVo {
        OverviewVo {
            cpu_usage: format!("{:.1}", self.cpu_usage),
            memory_usage: self.memory_usage.convert(),
            disk_usage: self.disk_usage.convert(),
            disk_io: self.disk_io.convert(),
            network_io: self.network_io.convert(),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct OsOverviewVo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_info: CpuInfoVo,
    pub users: Vec<UserVo>,
    pub boot_time: u64,
}

impl Convert<OsOverviewVo> for OsOverview {
    fn convert(&self) -> OsOverviewVo {
        OsOverviewVo {
            name: self.name.clone(),
            kernel_version: self.kernel_version.clone(),
            os_version: self.os_version.clone(),
            hostname: self.hostname.clone(),
            cpu_info: self.cpu_info.convert(),
            users: self.users.iter().map(|user| user.convert()).collect(),
            boot_time: self.boot_time,
        }
    }
}
