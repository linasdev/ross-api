use serde::Deserialize;

use ross_protocol::event::relay::RelayValue;

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RelayCommandPayload {
    #[serde(rename = "RELAY_SET_SINGLE")]
    SetSingle {
        #[serde(rename = "VALUE")]
        value: bool,
    },
}

impl From<RelayCommandPayload> for RelayValue {
    fn from(payload: RelayCommandPayload) -> Self {
        match payload {
            RelayCommandPayload::SetSingle { value } => RelayValue::Single(value),
        }
    }
}
