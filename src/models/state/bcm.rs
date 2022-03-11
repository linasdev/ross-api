use serde::Serialize;
use std::convert::TryFrom;

use ross_protocol::event::bcm::BcmValue;

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum BcmPeripheralState {
    #[serde(rename = "BCM_SINGLE")]
    Single {
        #[serde(rename = "ON")]
        on: bool,
        #[serde(rename = "BRIGHTNESS")]
        brightness: u8,
    },
    #[serde(rename = "BCM_RGB_B")]
    RgbB {
        #[serde(rename = "ON")]
        on: bool,
        #[serde(rename = "RED")]
        red: u8,
        #[serde(rename = "GREEN")]
        green: u8,
        #[serde(rename = "BLUE")]
        blue: u8,
        #[serde(rename = "BRIGHTNESS")]
        brightness: u8,
    },
    #[serde(rename = "BCM_RGBW_B")]
    RgbwB {
        #[serde(rename = "ON")]
        on: bool,
        #[serde(rename = "RED")]
        red: u8,
        #[serde(rename = "GREEN")]
        green: u8,
        #[serde(rename = "BLUE")]
        blue: u8,
        #[serde(rename = "WHITE")]
        white: u8,
        #[serde(rename = "BRIGHTNESS")]
        brightness: u8,
    },
}

impl TryFrom<BcmValue> for BcmPeripheralState {
    type Error = ();

    fn try_from(value: BcmValue) -> Result<Self, ()> {
        match value {
            BcmValue::Binary(_) => Err(()),
            BcmValue::Single(brightness) => Ok(BcmPeripheralState::Single {
                on: brightness != 0,
                brightness,
            }),
            BcmValue::RgbB(red, green, blue, brightness) => Ok(BcmPeripheralState::RgbB {
                on: (red != 0 || green != 0 || blue != 0) && brightness != 0,
                red,
                green,
                blue,
                brightness,
            }),
            BcmValue::RgbwB(red, green, blue, white, brightness) => Ok(BcmPeripheralState::RgbwB {
                on: (red != 0 || green != 0 || blue != 0 || white != 0) && brightness != 0,
                red,
                green,
                blue,
                white,
                brightness,
            }),
            _ => Err(())
        }
    }
}
