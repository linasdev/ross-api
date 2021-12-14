use std::time::Duration;

use ross_protocol::protocol::{BROADCAST_ADDRESS, Protocol};
use ross_protocol::interface::serial::Serial;

use crate::ProtocolConfig;
use crate::errors::ApiError;

pub fn create_protocol<'a>(config: &ProtocolConfig) -> Result<Protocol<'a, Serial>, ApiError> {
    let port = match serialport::new(config.serial_device.clone(), config.serial_baudrate)
        .timeout(Duration::from_millis(
            (config.transaction_retry_count * config.packet_timeout_ms) as u64,
        ))
        .open() {
            Ok(port) => port,
            Err(_) => {
                return Err(ApiError::CommunicationError(()));
            },
        };

    let serial = Serial::new(port);
    Ok(Protocol::new(BROADCAST_ADDRESS, serial))
}
