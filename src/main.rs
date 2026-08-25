use std::fs;

mod conf;
mod gpio_utils;
mod mqtt;

fn main() {
    println!(r"
        __ _  _    _                                _               __ _  _ __  _      
 _ __  / _` || |_ | |_        _ _  ___  _ __   ___ | |_  ___       / _` || '_ \(_) ___ 
| '  \ \__. ||  _||  _|      | '_|/ -_)| '  \ / _ \|  _|/ -_)      \__. || .__/| |/ _ \
|_|_|_|   |_| \__| \__|      |_|  \___||_|_|_|\___/ \__|\___|      |___/ |_|   |_|\___/
        ");

    let mut conf_path = std::env::args().nth(1).unwrap_or("config.yml".to_string());
    if fs::exists(conf_path).unwrap_or(false) {
        conf_path = "/etc/mqtt-remote-gpio.config.yml".to_string();
    }

    let config = conf::load_config(&conf_path).unwrap();

    mqtt::handler(config);
}
