use crate::model::process::Process;
use crate::vo::disk::DiskIOVo;
use crate::vo::formator::Convert;
use serde::{Deserialize, Serialize};

use super::formator::{FormatData, Formator};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ProcessVo {
    pub name: String,

    /// command
    pub cmd: Vec<String>,

    /// executable path
    pub exe: String,

    pub pid: u32,

    pub environ: Vec<String>,

    /// current working directory
    pub cwd: String,

    pub root: String,

    pub memory: FormatData,

    pub vir_mem: FormatData,

    pub parent: Option<u32>,

    pub status: String,

    pub start_time: u64,

    pub run_time: u64,

    pub cpu: f32,

    pub disk: DiskIOVo,

    pub user_id: Option<String>,

    pub group_id: Option<String>,
}

impl Convert<ProcessVo> for Process {
    fn convert(&self) -> ProcessVo {
        let formator = Formator::new();
        ProcessVo {
            name: self.name.clone(),
            cmd: self.cmd.clone(),
            exe: self.exe.clone(),
            pid: self.pid,
            environ: self.environ.clone(),
            cwd: self.cwd.clone(),
            root: self.root.clone(),
            memory: formator.format_from_kilo_byte(self.memory as f64),
            vir_mem: formator.format_from_kilo_byte(self.virtual_memory as f64),
            parent: self.parent,
            status: self.status.clone(),
            start_time: self.start_time,
            run_time: self.run_time,
            cpu: self.cpu_usage,
            disk: self.disk_usage.convert(),
            user_id: self.user_id.clone(),
            group_id: self.group_id.clone(),
        }
    }
}
