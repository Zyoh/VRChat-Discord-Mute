use std::error::Error;
use std::path::{Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub vrchat_sends_to_addr: String,
    pub vrchat_listens_to_addr: String,

    pub modules_to_run: ModuleRunOption, // 0 -> All, 1 -> thread_desktop, 2 -> thread_vrchat
    pub application_binds_to_addr: String, // Any free port will work. Ex: "127.0.0.1:43249

    pub discord_mute_hotkey: u32, // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    pub vrchat_gesture_left_addr: String, // https://docs.vrchat.com/docs/animator-parameters#parameters
    pub vrchat_trigger_gesture: i32, // https://docs.vrchat.com/docs/animator-parameters#gestureleft-and-gestureright-values

    pub vrchat_mute_hotkey: String, // https://docs.rs/rdev/latest/rdev/enum.Key.html
    pub vrchat_voice_addr: String, // https://docs.vrchat.com/docs/animator-parameters#parameters
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

            modules_to_run: ModuleRunOption::All,
            application_binds_to_addr: "127.0.0.1:49590".to_string(),

            discord_mute_hotkey: 0x13,
            vrchat_gesture_left_addr: "/avatar/parameters/GestureLeft".to_string(),
            vrchat_trigger_gesture: 5,

            vrchat_mute_hotkey: "altgr".to_string(),
            vrchat_voice_addr: "/input/Voice".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ModuleRunOption {
    All = 0,
    Desktop = 1,
    VRChat = 2,
}


