use crate::model::disk::{DiskDetail, DiskIO};
use crate::vo::formator::{Convert, FormatData, Formator};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DiskIOVo {
    pub read: FormatData,
    pub total_read: FormatData,
    pub write: FormatData,
    pub total_write: FormatData,
}

impl Convert<DiskIOVo> for DiskIO {
    fn convert(&self) -> DiskIOVo {
        let formator = Formator::new();
        DiskIOVo {
            read: formator.format_from_byte(self.read),
            total_read: formator.format_from_byte(self.total_read),
            write: formator.format_from_byte(self.write),
            total_write: formator.format_from_byte(self.total_write),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DiskDetailVo {
    pub disk_type: String,
    pub device_name: String,
    pub file_system: String,
    pub total_space: FormatData,
    pub available_space: FormatData,
    pub is_removable: bool,
}

impl Convert<DiskDetailVo> for DiskDetail {
    fn convert(&self) -> DiskDetailVo {
        let formator = Formator::new();
        DiskDetailVo {
            disk_type: self.disk_type.clone(),
            device_name: self.device_name.clone(),
            file_system: self.file_system.clone(),
            total_space: formator.format_from_byte(self.total_space),
            available_space: formator.format_from_byte(self.available_space),
            is_removable: self.is_removable,
        }
    }
}
