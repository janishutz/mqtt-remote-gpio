use std::{thread, time::Duration};
mod conf;
mod gpio_utils;
mod mqtt;

fn main() {
    // TODO: Proper cli interface and start screen
    // CLI interface should allow setting the config file
    println!("mqtt-remote-gpio");

    let config = conf::load_config("config.yml").unwrap();

    // TODO: Remove this when done
    println!("{:#?}", config);
    thread::sleep(Duration::from_secs(2));

    mqtt::handler(config);
}
