use serde::{Deserialize, Serialize};
use sysinfo::{NetworkExt, Networks, NetworksExt};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetworkIO {
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetworkDetail {
    pub name: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64,
    pub packets_transmitted: u64,
    pub total_packets_transmitted: u64,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
}

impl NetworkDetail {
    pub fn new_list(net: &Networks) -> Vec<NetworkDetail> {
        net.iter()
            .map(|net| NetworkDetail {
                name: net.0.to_string(),
                received: net.1.received(),
                total_received: net.1.total_received(),
                transmitted: net.1.transmitted(),
                total_transmitted: net.1.total_transmitted(),
                packets_received: net.1.packets_received(),
                total_packets_received: net.1.total_packets_received(),
                packets_transmitted: net.1.packets_transmitted(),
                total_packets_transmitted: net.1.total_packets_transmitted(),
                errors_on_received: net.1.errors_on_received(),
                total_errors_on_received: net.1.total_errors_on_received(),
                errors_on_transmitted: net.1.errors_on_transmitted(),
                total_errors_on_transmitted: net.1.total_errors_on_transmitted(),
            })
            .collect()
    }
}
