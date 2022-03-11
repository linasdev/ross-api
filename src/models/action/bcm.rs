use serde::Deserialize;

use ross_protocol::event::bcm::BcmValue;

#[derive(Deserialize)]
#[serde(tag = "actionType", rename_all = "camelCase")]
pub enum BcmAction {
    #[serde(rename = "changeBrightness")]
    ChangeBrightness {
        action_value: BcmChangeBrightnessValue,
    },
}

#[derive(Deserialize)]
#[serde(tag = "valueType", rename_all = "camelCase")]
pub enum BcmChangeBrightnessValue {
    Binary { value: bool },
    Single { value: u8 },
    Rgb { red: u8, green: u8, blue: u8 },
    RgbB {
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
    },
    Rgbw {
        red: u8,
        green: u8,
        blue: u8,
        white: u8,
    },
    RgbwB {
        red: u8,
        green: u8,
        blue: u8,
        white: u8,
        brightness: u8,
    },
}

impl From<BcmChangeBrightnessValue> for BcmValue {
    fn from(value: BcmChangeBrightnessValue) -> BcmValue {
        match value {
            BcmChangeBrightnessValue::Binary { value } => BcmValue::Binary(value),
            BcmChangeBrightnessValue::Single { value } => BcmValue::Single(value),
            BcmChangeBrightnessValue::Rgb { red, green, blue } => BcmValue::Rgb(red, green, blue),
            BcmChangeBrightnessValue::RgbB {
                red,
                green,
                blue,
                brightness,
            } => BcmValue::RgbB(red, green, blue, brightness),
            BcmChangeBrightnessValue::Rgbw {
                red,
                green,
                blue,
                white,
            } => BcmValue::Rgbw(red, green, blue, white),
            BcmChangeBrightnessValue::RgbwB {
                red,
                green,
                blue,
                white,
                brightness,
            } => BcmValue::RgbwB(red, green, blue, white, brightness),
        }
    }
}
