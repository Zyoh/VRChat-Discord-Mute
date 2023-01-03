mod from_vrchat;
mod from_desktop;
mod timestamp;

extern crate rosc;

use std::thread;

fn main() {
    thread::spawn(|| {
        if let Err(e) = from_desktop::mainloop() {
            println!("Error: {}", e);
        }
    });

    let t2 = thread::spawn(|| {
        if let Err(e) = from_vrchat::mainloop() {
            println!("Error: {}", e);
        }
    });

    t2.join().unwrap();
}
