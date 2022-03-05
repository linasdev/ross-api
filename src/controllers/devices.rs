use rocket::{State, get, post};
use rocket_contrib::json::Json;

use ross_configurator::get_programmer::get_programmer;
use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm::BcmChangeBrightnessEvent;

use crate::models::device::Device;
use crate::models::action::bcm::BcmAction;
use crate::ProtocolConfig;
use crate::helpers::create_protocol;
use crate::errors::ApiError;

#[get("/")]
pub fn get_devices(protocol_config: State<ProtocolConfig>) -> Result<Json<Vec<Device>>, ApiError> {
    let mut protocol = create_protocol(protocol_config.inner())?;

    let programmer = get_programmer(&mut protocol)?;
    let devices = ross_configurator::get_devices::get_devices(&mut protocol, &programmer)?
        .iter()
        .map(|device| Device {
            address: device.bootloader_address,
        })
        .collect();

    Ok(Json(devices))
}

#[post("/<bcm_address>/bcm/<peripheral_id>", data = "<action>")]
pub fn act_bcm(bcm_address: u16, peripheral_id: u8, action: Json<BcmAction>, protocol_config: State<ProtocolConfig>) -> Result<(), ApiError> {
    let mut protocol = create_protocol(protocol_config.inner())?;

    let programmer = get_programmer(&mut protocol)?;

    let packet = match action.into_inner() {
        BcmAction::ChangeBrightness { action_value } => {
            BcmChangeBrightnessEvent {
                bcm_address,
                transmitter_address: programmer.programmer_address,
                index: peripheral_id,
                value: action_value.into(),
            }.to_packet()
        }
    };

    protocol.send_packet(&packet)?;

    Ok(())
}
