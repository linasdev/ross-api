#![feature(decl_macro)]

use rocket::fairing::AdHoc;
use rocket::Rocket;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Duration;

use ross_protocol::interface::serial::Serial;
use ross_protocol::protocol::{Protocol, BROADCAST_ADDRESS};

pub mod controllers;
pub mod errors;
pub mod models;
pub mod routes;

fn main() {
    let rocket = routes::build();

    let protocol = Arc::new(Mutex::new(open_protocol(&rocket)));

    let protocol_clone = Arc::clone(&protocol);
    let rocket_handle = spawn(|| {
        rocket
            .attach(AdHoc::on_attach("Protocol setup", move |rocket| {
                Ok(rocket.manage(protocol_clone))
            }))
            .launch();
    });

    rocket_handle.join().unwrap();
}

fn open_protocol<'a>(rocket: &Rocket) -> Protocol<'a, Serial> {
    let serial_device = rocket.config().get_string("serial_device").unwrap();
    let serial_baudrate = rocket.config().get_int("serial_baudrate").unwrap() as u32;
    let transaction_retry_count =
        rocket.config().get_int("transaction_retry_count").unwrap() as u32;
    let packet_timeout_ms = rocket.config().get_int("packet_timeout_ms").unwrap() as u32;

    let port = match serialport::new(serial_device.clone(), serial_baudrate)
        .timeout(Duration::from_millis(
            (transaction_retry_count * packet_timeout_ms) as u64,
        ))
        .open()
    {
        Ok(port) => port,
        Err(_) => {
            panic!("Failed to open serial port");
        }
    };

    let serial = Serial::new(port);
    Protocol::new(BROADCAST_ADDRESS, serial)
}
