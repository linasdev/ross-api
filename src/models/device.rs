use serde::Serialize;

#[derive(Serialize)]
pub struct Device {
    pub address: u16,
}
