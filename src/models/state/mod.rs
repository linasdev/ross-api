use serde::Serialize;

use bcm::BcmPeripheralState;

pub mod bcm;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayState {
    pub device_states: Vec<DeviceState>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    pub peripheral_address: u16,
    pub peripheral_index: u8,
    pub peripheral_state: PeripheralState,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum PeripheralState {
    Bcm(BcmPeripheralState)
}
