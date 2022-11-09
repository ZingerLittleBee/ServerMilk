use crate::model::memory::MemoryUsage;
use crate::vo::formator::{Convert, FormatData, Formator};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct MemUsageVo {
    pub total: FormatData,
    pub used: FormatData,
    pub free: FormatData,
    pub swap_total: FormatData,
    pub swap_used: FormatData,
    pub swap_free: FormatData,
}

impl Convert<MemUsageVo> for MemoryUsage {
    fn convert(&self) -> MemUsageVo {
        let formator = Formator::new();
        MemUsageVo {
            total: formator.format_from_byte(self.total),
            used: formator.format_from_byte(self.used),
            free: formator.format_from_byte(self.free),
            swap_total: formator.format_from_byte(self.swap_total),
            swap_used: formator.format_from_byte(self.swap_used),
            swap_free: formator.format_from_byte(self.swap_free),
        }
    }
}
