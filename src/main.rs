mod conf;
mod gpio_utils;
mod mqtt;

fn main() {
    // TODO: Proper cli interface and start screen
    // CLI interface should allow setting the config file
    println!("mqtt-remote-gpio");

    // let config = conf::load_config("config.yml").unwrap();
    let config = conf::load_config("config.secret.yml").unwrap();

    // TODO: Remove this when done
    println!("{:#?}", config);

    mqtt::handler(config);
}
