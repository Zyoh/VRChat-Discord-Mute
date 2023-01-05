use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use rosc::OscType::Int;
use rdev::{simulate, EventType, Key};
use std::{thread, time};
use std::str::FromStr;

// TODO: Make these configurable
const DISCORD_MUTE_HOTKEY: Key = Key::Pause; // Set this in Discord as your mute toggle.
const VRCHAT_SENDS_TO_ADDR: &str = "127.0.0.1:9001";
const VRCHAT_GESTURE_LEFT_ADDR: &str = "/avatar/parameters/GestureLeft";
const VRCHAT_TRIGGER_GESTURE: i32 = 5; // This value corresponds to the gesture that triggers mute.

pub fn mainloop() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddrV4::from_str(VRCHAT_SENDS_TO_ADDR)?;
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
    if msg.addr == VRCHAT_GESTURE_LEFT_ADDR {
        if let Int(value) = msg.args[0] {
            if value == VRCHAT_TRIGGER_GESTURE {
                match discord_toggle_mute() {
                    Ok(_) => log::info!("Toggled Discord mute."),
                    Err(e) => log::warn!("Error toggling mute: {}", e),
                }
            }
        }
    }
}

fn discord_toggle_mute() -> Result<(), rdev::SimulateError> {
    simulate(&EventType::KeyPress(DISCORD_MUTE_HOTKEY))?;
    thread::sleep(time::Duration::from_millis(100));
    simulate(&EventType::KeyRelease(DISCORD_MUTE_HOTKEY))?;
    Ok(())
}
