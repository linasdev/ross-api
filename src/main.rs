#![feature(decl_macro)]

use rocket::fairing::AdHoc;
use rocket::Rocket;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Duration;
use std::convert::TryInto;

use ross_protocol::interface::serial::Serial;
use ross_protocol::protocol::{Protocol, BROADCAST_ADDRESS};
use ross_protocol::event::bcm::BcmChangeBrightnessEvent;
use ross_protocol::event::relay::RelaySetValueEvent;
use ross_protocol::event::gateway::GatewayDiscoverEvent;
use ross_protocol::convert_packet::ConvertPacket;
use ross_configurator::get_programmer::get_programmer;
use ross_configurator::get_devices::get_devices;

use crate::mqtt::Mqtt;
use crate::models::command::CommandPayload;
use crate::models::state::{GatewayState, DeviceState, PeripheralState};

pub mod controllers;
pub mod errors;
pub mod models;
pub mod routes;
pub mod mqtt;

fn main() {
    let rocket = routes::build();

    let protocol = Arc::new(Mutex::new(open_protocol(&rocket)));
    let mut mqtt = Mqtt::new(&rocket).unwrap();

    let protocol_clone = Arc::clone(&protocol);
    let _rocket_handle = spawn(|| {
        rocket
            .attach(AdHoc::on_attach("Protocol setup", move |rocket| {
                Ok(rocket.manage(protocol_clone))
            }))
            .launch();
    });

    mqtt.connect().unwrap();
    mqtt.subscribe_to_commands(1).unwrap();

    let gateway_state = Arc::new(Mutex::new(GatewayState {
        device_states: vec![],
    }));
    let commands = Arc::new(Mutex::new(vec![]));
    let discover = Arc::new(Mutex::new(false));

    let gateway_state_clone = Arc::clone(&gateway_state);
    let commands_clone = Arc::clone(&commands);
    let discover_clone = Arc::clone(&discover);

    let _mqtt_handle = spawn(move || {
        mqtt.start_loop(gateway_state_clone, commands_clone, discover_clone);
    });

    let gateway_state_clone = Arc::clone(&gateway_state);
    protocol.lock().unwrap().add_packet_handler(
            Box::new(move |packet, _protocol| {
                if let Ok(event) = BcmChangeBrightnessEvent::try_from_packet(packet) {
                    if let Ok(peripheral_state) = event.value.try_into() {
                        gateway_state_clone.lock().unwrap().device_states.push(DeviceState {
                                peripheral_address: event.transmitter_address,
                                peripheral_index: event.index,
                                peripheral_state: PeripheralState::Bcm(peripheral_state),
                        });
                    }
                } else if let Ok(event) = RelaySetValueEvent::try_from_packet(packet) {
                    if let Ok(peripheral_state) = event.value.try_into() {
                        gateway_state_clone.lock().unwrap().device_states.push(DeviceState {
                                peripheral_address: event.transmitter_address,
                                peripheral_index: event.index,
                                peripheral_state: PeripheralState::Relay(peripheral_state),
                        });
                    }
                }
            }),
            false,
        )
        .unwrap();

    let programmer = get_programmer(&mut protocol.lock().unwrap()).unwrap();

    loop {
        if let Err(err) = protocol.lock().unwrap().tick() {
            println!("Unexpected error occurred: {:?}", err);
        }

        if let Some(gateway_command) = commands.lock().unwrap().pop() {
            for device_command in gateway_command.device_commands {
                let packet = match device_command.payload {
                    CommandPayload::Bcm(payload) => BcmChangeBrightnessEvent {
                        bcm_address: device_command.peripheral_address,
                        transmitter_address: programmer.programmer_address,
                        index: device_command.peripheral_index,
                        value: payload.into(),
                    }.to_packet(),
                    CommandPayload::Relay(payload) => RelaySetValueEvent {
                        relay_address: device_command.peripheral_address,
                        transmitter_address: programmer.programmer_address,
                        index: device_command.peripheral_index,
                        value: payload.into(),
                    }.to_packet(),
                };

                if let Err(err) = protocol.lock().unwrap().send_packet(&packet) {
                    println!("Failed to send packet with error ({:?})", err);
                } else {
                    println!("Sent packet ({:?})", packet);
                }
            }
        }

        if *discover.lock().unwrap() {
            let devices = get_devices(&mut protocol.lock().unwrap(), &programmer).unwrap();

            for device in devices {
                let packet = GatewayDiscoverEvent {
                    device_address: device.bootloader_address,
                    gateway_address: programmer.programmer_address,
                }
                .to_packet();

                if let Err(err) = protocol.lock().unwrap().send_packet(&packet) {
                    println!("Failed to send packet with error ({:?})", err);
                } else {
                    println!("Sent packet ({:?})", packet);
                }
            }

            *discover.lock().unwrap() = false;
        }
    }

    // rocket_handle.join().unwrap();
    // mqtt_handle.join().unwrap();
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
