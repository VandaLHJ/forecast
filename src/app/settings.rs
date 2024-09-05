use cosmic::app::Settings;

use crate::app::config::WeatherConfig;
use crate::app::Flags;

use super::config::WeatherConfigState;
use super::localize::localize;

pub fn settings() -> Settings {
    localize();
    Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false)
}

pub fn flags() -> Flags {
    let (config_handler, config) = (WeatherConfig::config_handler(), WeatherConfig::config());
    let (config_state_handler, config_state) = (
        WeatherConfigState::config_handler(),
        WeatherConfigState::config(),
    );
    Flags {
        config_handler,
        config,
        config_state,
        config_state_handler,
    }
}
