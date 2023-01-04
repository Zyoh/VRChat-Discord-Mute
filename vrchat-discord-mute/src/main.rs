mod from_vrchat;
mod from_desktop;
mod timestamp;

extern crate rosc;

use std::thread;

fn main() {
    // TODO: Let user choose which to run

    let thread_desktop = thread::spawn(|| {
        if let Err(e) = from_desktop::mainloop() {
            println!("Error: {}", e);
        }
    });

    let thread_vrchat = thread::spawn(|| {
        if let Err(e) = from_vrchat::mainloop() {
            println!("Error: {}", e);
        }
    });

    thread_desktop.join().unwrap();
    thread_vrchat.join().unwrap();
}
