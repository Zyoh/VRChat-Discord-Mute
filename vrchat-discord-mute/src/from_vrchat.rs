use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use rosc::OscType::Int;
use rdev::{simulate, EventType, Key};
use std::{thread, time};
use std::str::FromStr;

use crate::timestamp::iso8601;

const VRCHAT_GESTURE_LEFT_ADDR: &str = "/avatar/parameters/GestureLeft";

pub fn mainloop() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddrV4::from_str("127.0.0.1:9001")?;
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        let (size, _) = sock.recv_from(&mut buf)?;
        let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
        if let OscPacket::Message(msg) = packet {
            handle_message(msg);
        }
    }
}

fn handle_message(msg: rosc::OscMessage) {
    if msg.addr == VRCHAT_GESTURE_LEFT_ADDR {
        if let Int(value) = msg.args[0] {
            if value == 5 {
                match discord_toggle_mute() {
                    Ok(_) => {
                        // TODO: Use a better logging system
                        let mut log = iso8601();
                        log.push_str(" | Toggled Discord mute");
                        println!("{}", log);
                    },
                    Err(e) => println!("Error toggling mute: {}", e),
                }
            }
        }
    }
}

fn discord_toggle_mute() -> Result<(), rdev::SimulateError> {
    simulate(&EventType::KeyPress(Key::Pause))?;
    thread::sleep(time::Duration::from_millis(100));
    simulate(&EventType::KeyRelease(Key::Pause))?;
    Ok(())
}