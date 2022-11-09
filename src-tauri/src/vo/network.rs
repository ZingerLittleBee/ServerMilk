use crate::model::network::{NetworkDetail, NetworkIO};
use crate::vo::formator::{Convert, FormatData, Formator};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetworkIOVo {
    pub rx: FormatData,
    pub ttl_rx: FormatData,
    pub tx: FormatData,
    pub ttl_tx: FormatData,
}

impl Convert<NetworkIOVo> for NetworkIO {
    fn convert(&self) -> NetworkIOVo {
        let formator = Formator::new();
        NetworkIOVo {
            rx: formator.format_from_byte(self.received),
            ttl_rx: formator.format_from_byte(self.total_received),
            tx: formator.format_from_byte(self.transmitted),
            ttl_tx: formator.format_from_byte(self.total_transmitted),
        }
    }
}

/// rx = received
/// tx = transmitted
/// pkts = packets
/// ttl = total
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetworkDetailVo {
    pub name: String,
    pub rx: FormatData,
    pub ttl_rx: FormatData,
    pub tx: FormatData,
    pub ttl_tx: FormatData,
    pub pkt_rx: FormatData,
    pub ttl_pkt_rx: FormatData,
    pub pkt_tx: FormatData,
    pub ttl_pkt_tx: FormatData,
    pub err_rx: FormatData,
    pub ttl_err_rx: FormatData,
    pub err_tx: FormatData,
    pub ttl_err_tx: FormatData,
}

impl Convert<NetworkDetailVo> for NetworkDetail {
    fn convert(&self) -> NetworkDetailVo {
        let formator = Formator::new();
        NetworkDetailVo {
            name: self.name.clone(),
            rx: formator.format_from_byte(self.received),
            ttl_rx: formator.format_from_byte(self.total_received),
            tx: formator.format_from_byte(self.transmitted),
            ttl_tx: formator.format_from_byte(self.total_transmitted),
            pkt_rx: formator.format_from_byte(self.packets_received),
            ttl_pkt_rx: formator.format_from_byte(self.total_packets_received),
            pkt_tx: formator.format_from_byte(self.packets_transmitted),
            ttl_pkt_tx: formator.format_from_byte(self.total_packets_transmitted),
            err_rx: formator.format_from_byte(self.errors_on_received),
            ttl_err_rx: formator.format_from_byte(self.total_errors_on_received),
            err_tx: formator.format_from_byte(self.errors_on_transmitted),
            ttl_err_tx: formator.format_from_byte(self.total_errors_on_transmitted),
        }
    }
}
