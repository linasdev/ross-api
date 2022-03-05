#![feature(decl_macro)]

use std::time::Duration;
use rocket::fairing::AdHoc;

use ross_protocol::protocol::{BROADCAST_ADDRESS, Protocol};
use ross_protocol::interface::serial::Serial;

use std::sync::Mutex;

pub mod controllers;
pub mod routes;
pub mod models;
pub mod errors;

fn main() {
    routes::build()
        .attach(AdHoc::on_attach("Protocol setup", |rocket| {
            let serial_device = rocket.config().get_string("serial_device").unwrap();
            let serial_baudrate = rocket.config().get_int("serial_baudrate").unwrap() as u32;
            let transaction_retry_count = rocket.config().get_int("transaction_retry_count").unwrap() as u32;
            let packet_timeout_ms = rocket.config().get_int("packet_timeout_ms").unwrap() as u32;

            let port = match serialport::new(serial_device.clone(), serial_baudrate)
            .timeout(Duration::from_millis(
                (transaction_retry_count * packet_timeout_ms) as u64,
            ))
            .open() {
                Ok(port) => port,
                Err(_) => {
                    panic!("Failed to open serial port");
                },
            };
    
            let serial = Serial::new(port);
            Ok(rocket.manage(Mutex::new(Protocol::new(BROADCAST_ADDRESS, serial))))
        }))
        .launch();
}
