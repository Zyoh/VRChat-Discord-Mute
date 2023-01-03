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

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, _)) => {
                let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            if msg.addr == "/avatar/parameters/GestureLeft" {
                if let Int(value) = msg.args.iter().next().unwrap() {
                    println!("GestureLeft: {}", value);
                    if *value == 5 {
                        println!("Toggling mute...");
                        match discord_toggle_mute() {
                            Ok(_) => println!("Toggled mute"),
                            Err(e) => println!("Error toggling mute: {}", e),
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn discord_toggle_mute() -> Result<(), rdev::SimulateError> {
    simulate(&EventType::KeyPress(Key::Pause))?;
    thread::sleep(time::Duration::from_millis(100));
    simulate(&EventType::KeyRelease(Key::Pause))?;
    Ok(())
}
