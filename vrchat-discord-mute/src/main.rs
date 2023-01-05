mod from_vrchat;
mod from_desktop;
mod logging;
mod config;

extern crate rosc;
#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::path::{PathBuf};
use std::thread;
use config::Config;

const CONFIG_NAME: &str = "config.json";

lazy_static! {
    static ref CONFIG: Config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            log::warn!("Failed to load config file: {}", e);
            Config::new()
        }
    };
}

fn main() {
    log::set_logger(&logging::Logger).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    match save_config(&CONFIG) {
        Ok(_) => {
            log::debug!("Saved config file.")
        }
        Err(e) => {
            log::error!("Failed to save config file: {}", e)
        }
    }

    if CONFIG.modules_to_run == config::ModuleRunOption::All
        || CONFIG.modules_to_run == config::ModuleRunOption::Desktop {

        thread::spawn(|| {
            log::info!("Starting desktop thread...");
            if let Err(e) = from_desktop::mainloop() {
                log::error!("Error: {:?}", e);
            }
        });
    }
    if CONFIG.modules_to_run == config::ModuleRunOption::All
        || CONFIG.modules_to_run == config::ModuleRunOption::VRChat {

        thread::spawn(|| {
            log::info!("Starting VRChat thread...");
            if let Err(e) = from_vrchat::mainloop() {
                log::error!("Error: {}", e);
            }
        });
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_path = std::env::current_dir()?.join(CONFIG_NAME);
    config.save(config_path)?;

    Ok(())
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let config = Config::load(config_path()?)?;

    Ok(config)
}

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    let config_path = std::env::current_dir()?.join(CONFIG_NAME);

    Ok(config_path)
}
