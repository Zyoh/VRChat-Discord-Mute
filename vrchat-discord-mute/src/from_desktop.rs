use std::error::Error;
use std::net::UdpSocket;
use rdev::{Event, Key, listen};
use rdev::EventType::KeyPress;
use rosc::{encoder, OscMessage, OscPacket, OscType};

use crate::timestamp::iso8601;

const VRCHAT_VOICE_ADDR: &str = "/input/Voice";

pub fn mainloop() -> Result<(), std::io::Error> {
    if let Err(e) = listen(callback) {
        println!("Error: {:?}", e);
    }

    Ok(())
}

fn callback(event: Event) {
    match event.event_type {
        KeyPress(Key::AltGr) => {
            match vrchat_toggle_mute() {
                Ok(_) => {
                    // TODO: Use a better logging system
                    let mut log = iso8601();
                    log.push_str(" | Toggled VRChat mute");
                    println!("{}", log);
                },
                Err(e) => println!("Error toggling mute: {}", e),
            }
        },
        _ => (),
    }
}

fn vrchat_toggle_mute() -> Result<(), Box<dyn Error>> {
    for value in [0, 1] {
        send_voice_value(value)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

fn send_voice_value(value: i32) -> Result<(), Box<dyn std::error::Error>> {
    let sock = UdpSocket::bind("127.0.0.1:49000")?;

    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: VRCHAT_VOICE_ADDR.to_string(),
        args: vec![OscType::Int(value)],
    }))?;

    sock.send_to(&msg_buf, "127.0.0.1:9000")?;

    Ok(())
}