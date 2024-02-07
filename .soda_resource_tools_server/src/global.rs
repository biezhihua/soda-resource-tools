use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::config::Config;

pub(crate) static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let mut config = Config::new();
    config.update();
    Mutex::new(config)
});