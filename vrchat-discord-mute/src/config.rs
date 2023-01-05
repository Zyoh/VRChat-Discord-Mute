use std::error::Error;
use std::path::{Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    vrchat_sends_to_addr: String,
    vrchat_listens_to_addr: String,

    modules_to_run: u8, // 0 -> All, 1 -> thread_desktop, 2 -> thread_vrchat
    application_binds_to_addr: String, // Any free port will work. Ex: "127.0.0.1:43249

    discord_mute_hotkey: u32, // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    vrchat_gesture_left_addr: String, // https://docs.vrchat.com/docs/animator-parameters#parameters
    vrchat_trigger_gesture: i32, // https://docs.vrchat.com/docs/animator-parameters#gestureleft-and-gestureright-values

    vrchat_mute_hotkey: u32, // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    vrchat_voice_addr: String, // https://docs.vrchat.com/docs/animator-parameters#parameters
}

impl Config {
    pub fn load<P>(path: P) -> Result<Self, Box<dyn Error>> where P: AsRef<Path> {

        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;

        Ok(config)
    }

    pub fn save<P>(&self, path: P) -> Result<(), Box<dyn Error>> where P: AsRef<Path> {

        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            vrchat_sends_to_addr: "127.0.0.1:9001".to_string(),
            vrchat_listens_to_addr: "127.0.0.1:9000".to_string(),

            modules_to_run: 0,
            application_binds_to_addr: "127.0.0.1:49590".to_string(),

            discord_mute_hotkey: 0xA3,
            vrchat_gesture_left_addr: "/avatar/parameters/GestureLeft".to_string(),
            vrchat_trigger_gesture: 5,

            vrchat_mute_hotkey: 0xA5,
            vrchat_voice_addr: "/input/Voice".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
