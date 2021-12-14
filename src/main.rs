#![feature(decl_macro)]

use rocket::fairing::AdHoc;

pub mod controllers;
pub mod routes;
pub mod models;
pub mod helpers;
pub mod errors;

fn main() {
    routes::build()
        .attach(AdHoc::on_attach("Protocol setup", |rocket| {
            let serial_device = rocket.config().get_string("serial_device").unwrap();
            let serial_baudrate = rocket.config().get_int("serial_baudrate").unwrap() as u32;
            let transaction_retry_count = rocket.config().get_int("transaction_retry_count").unwrap() as u32;
            let packet_timeout_ms = rocket.config().get_int("packet_timeout_ms").unwrap() as u32;

            Ok(rocket.manage(ProtocolConfig {
                serial_device,
                serial_baudrate,
                transaction_retry_count,
                packet_timeout_ms,
            }))
        }))
        .launch();
}

pub struct ProtocolConfig {
    pub serial_device: String,
    pub serial_baudrate: u32,
    pub transaction_retry_count: u32,
    pub packet_timeout_ms: u32,
}
