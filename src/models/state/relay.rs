use serde::Serialize;
use std::convert::TryFrom;

use ross_protocol::event::relay::RelayValue;

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum RelayPeripheralState {
    #[serde(rename = "RELAY_SINGLE")]
    Single {
        #[serde(rename = "ON")]
        on: bool,
    },
}

impl TryFrom<RelayValue> for RelayPeripheralState {
    type Error = ();

    fn try_from(value: RelayValue) -> Result<Self, ()> {
        match value {
            RelayValue::Single(on) => Ok(RelayPeripheralState::Single {
                on,
            }),
            _ => Err(())
        }
    }
}
