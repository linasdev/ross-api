use serde::Deserialize;

use ross_protocol::event::bcm::BcmValue;

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum BcmCommandPayload {
    #[serde(rename = "BCM_SET_BINARY")]
    SetBinary {
        #[serde(rename = "VALUE")]
        value: bool,
    },
    #[serde(rename = "BCM_SET_SINGLE")]
    SetSingle {
        #[serde(rename = "VALUE")]
        value: u8,
    },
    #[serde(rename = "BCM_SET_RGB")]
    SetRgb {
        #[serde(rename = "RED")]
        red: u8,
        #[serde(rename = "GREEN")]
        green: u8,
        #[serde(rename = "BLUE")]
        blue: u8,
    },
    #[serde(rename = "BCM_SET_RGBW")]
    SetRgbw {
        #[serde(rename = "RED")]
        red: u8,
        #[serde(rename = "GREEN")]
        green: u8,
        #[serde(rename = "BLUE")]
        blue: u8,
        #[serde(rename = "WHITE")]
        white: u8,
    },
}

impl From<BcmCommandPayload> for BcmValue {
    fn from(payload: BcmCommandPayload) -> Self {
        match payload {
            BcmCommandPayload::SetBinary { value } => BcmValue::Binary(value),
            BcmCommandPayload::SetSingle { value } => BcmValue::Single(value),
            BcmCommandPayload::SetRgb { red, green, blue } => BcmValue::Rgb(red, green, blue),
            BcmCommandPayload::SetRgbw { red, green, blue, white } => BcmValue::Rgbw(red, green, blue, white),
        }
    }
}
