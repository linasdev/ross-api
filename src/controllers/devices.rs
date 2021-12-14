use rocket::{State, get};
use rocket_contrib::json::Json;

use ross_configurator::get_programmer::get_programmer;

use crate::models::device::Device;
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
