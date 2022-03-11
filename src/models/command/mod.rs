use serde::Deserialize;

use bcm::BcmCommandPayload;

pub mod bcm;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayCommand {
    pub device_commands: Vec<DeviceCommand>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCommand {
    pub peripheral_address: u16,
    pub peripheral_index: u8,
    #[serde(flatten)]
    pub payload: CommandPayload,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CommandPayload {
    Bcm(BcmCommandPayload),
}
