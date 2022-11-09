use crate::model::usage::Usage;
use crate::vo::formator::{Convert, FormatData, Formator};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct UsageVo {
    pub total: FormatData,
    pub used: FormatData,
    pub free: FormatData,
}

impl Convert<UsageVo> for Usage {
    fn convert(&self) -> UsageVo {
        let formator = Formator::new();
        UsageVo {
            total: formator.format_from_byte(self.total),
            used: formator.format_from_byte(self.used),
            free: formator.format_from_byte(self.free),
        }
    }
}
