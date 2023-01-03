extern crate rosc;

use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use rosc::OscType::Int;
use rdev::{simulate, EventType, Key};
use std::{thread, time};


fn main() {
    let addr = match SocketAddrV4::from_str("127.0.0.1:9001") {
        Ok(addr) => addr,
        Err(_) => panic!("Invalid address"),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    if let Err(e) = mainloop(&sock, &mut buf) {
        println!("Error: {}", e);
    }
}

fn mainloop(sock : &UdpSocket, buf : &mut [u8]) -> Result<(), std::io::Error> {
    loop {
        let (size, _) = sock.recv_from(buf)?;
        let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
        if let OscPacket::Message(msg) = packet {
            handle_message(msg);
        }
    }
}

fn handle_message(msg: rosc::OscMessage) {
    if msg.addr == "/avatar/parameters/GestureLeft" {
        if let Int(value) = msg.args[0] {
            println!("GestureLeft: {}", value);
            if value == 5 {
                println!("Toggling mute...");
                match discord_toggle_mute() {
                    Ok(_) => println!("Toggled mute"),
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
