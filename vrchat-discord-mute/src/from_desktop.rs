use std::error::Error;
use std::net::UdpSocket;
use rdev::{Event, listen, ListenError};
use rdev::EventType::KeyPress;
use rosc::{encoder, OscMessage, OscPacket, OscType};

use super::CONFIG;

pub fn mainloop() -> Result<(), ListenError> {
    listen(callback)?;
    Ok(())
}

fn callback(event: Event) {
    if let KeyPress(key) = event.event_type {
        log::debug!("Detected keypress: {:?}", key);
        if format!("{:?}", key).to_ascii_lowercase() == CONFIG.vrchat_mute_hotkey.to_ascii_lowercase() {
            match vrchat_toggle_mute() {
                Ok(_) => log::info!("Toggled VRChat mute."),
                Err(e) => log::warn!("Error toggling mute: {}", e),
            }
        }
    }
}

fn vrchat_toggle_mute() -> Result<(), Box<dyn Error>> {
    for value in [0, 1] {
        send_voice_value(value)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

fn send_voice_value(value: i32) -> Result<(), Box<dyn Error>> {
    let sock = UdpSocket::bind(&CONFIG.application_binds_to_addr)?;

    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: CONFIG.vrchat_voice_addr.to_string(),
        args: vec![OscType::Int(value)],
    }))?;

    sock.send_to(&msg_buf, &CONFIG.vrchat_listens_to_addr)?;

    Ok(())
}
