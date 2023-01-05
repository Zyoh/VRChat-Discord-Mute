mod from_vrchat;
mod from_desktop;
mod logging;

extern crate rosc;

use std::thread;

fn main() {
    // TODO: Let user choose which to run
    logging::init();

    let thread_desktop = thread::spawn(|| {
        if let Err(e) = from_desktop::mainloop() {
            log::error!("Error: {:?}", e);
        }
    });

    let thread_vrchat = thread::spawn(|| {
        if let Err(e) = from_vrchat::mainloop() {
            log::error!("Error: {}", e);
        }
    });

    thread_desktop.join().unwrap();
    thread_vrchat.join().unwrap();
}
