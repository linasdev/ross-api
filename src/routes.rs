use rocket::{routes, Rocket};

use crate::controllers;

pub fn build() -> Rocket {
    rocket::ignite().mount(
        "/devices",
        routes![
            controllers::devices::get_devices,
            controllers::devices::act_bcm,
        ],
    )
}
