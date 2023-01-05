use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use rosc::OscType::Int;
use rdev::{simulate, EventType, Key};
use std::{error, thread, time};
use std::str::FromStr;

use super::CONFIG;

pub fn mainloop() -> Result<(), Box<dyn error::Error>> {
    let addr = SocketAddrV4::from_str(&CONFIG.vrchat_sends_to_addr)?;
    let sock = UdpSocket::bind(addr).unwrap();
    log::info!("Listening to {}", addr);

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
    if msg.addr == CONFIG.vrchat_gesture_left_addr {
        if let Int(value) = msg.args[0] {
            if value == CONFIG.vrchat_trigger_gesture {
                match discord_toggle_mute() {
                    Ok(_) => log::info!("Toggled Discord mute."),
                    Err(e) => log::warn!("Error toggling mute: {}", e),
                }
            }
        }
    }
}

fn discord_toggle_mute() -> Result<(), rdev::SimulateError> {
    simulate(&EventType::KeyPress(Key::Unknown(CONFIG.discord_mute_hotkey)))?;
    thread::sleep(time::Duration::from_millis(100));
    simulate(&EventType::KeyRelease(Key::Unknown(CONFIG.discord_mute_hotkey)))?;
    Ok(())
}
