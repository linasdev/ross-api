use serde::Deserialize;

use ross_protocol::event::bcm::BcmValue;

#[derive(Deserialize)]
#[serde(tag = "action_type")]
pub enum BcmAction {
    #[serde(rename = "change_brightness")]
    ChangeBrightness {
        action_value: BcmChangeBrightnessValue,
    }
}

#[derive(Deserialize)]
#[serde(tag = "value_type")]
pub enum BcmChangeBrightnessValue {
    #[serde(rename = "single")]
    Single {
        value: u8,
    },
    #[serde(rename = "rgb")]
    Rgb {
        red: u8,
        green: u8,
        blue: u8,
    },
    #[serde(rename = "rgb_b")]
    RgbB {
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
    },
    #[serde(rename = "rgbw")]
    Rgbw {
        red: u8,
        green: u8,
        blue: u8,
        white: u8,
    },
    #[serde(rename = "rgbw_b")]
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
            BcmChangeBrightnessValue::Single { value } => BcmValue::Single(value),
            BcmChangeBrightnessValue::Rgb { red, green, blue } => BcmValue::Rgb(red, green, blue),
            BcmChangeBrightnessValue::RgbB { red, green, blue, brightness } => BcmValue::RgbB(red, green, blue, brightness),
            BcmChangeBrightnessValue::Rgbw { red, green, blue, white } => BcmValue::Rgbw(red, green, blue, white),
            BcmChangeBrightnessValue::RgbwB { red, green, blue, white, brightness } => BcmValue::RgbwB(red, green, blue, white, brightness),
        }
    }
}
