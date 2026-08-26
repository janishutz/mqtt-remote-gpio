use std::fs;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Topic {
    pub topic: String,
    pub pin: u8,
    pub mode: PinMode,
    pub off_timeout: i64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PinMode {
    IN,
    OUT,
}

#[derive(Debug)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub authentication: bool,
    pub user: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub topics: Vec<Topic>,
    pub poll_interval: u64,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Read config file
    let conf = fs::read_to_string(path).unwrap_or("config".to_string());
    let yaml = &YamlLoader::load_from_str(&conf).unwrap()[0];
    let conf = yaml
        .as_hash()
        .expect("Invalid config found at line 1. Not an object");
    let mqtt_conf = conf
        .get(&Yaml::String(String::from("mqtt")))
        .expect("MQTT config is missing.")
        .as_hash()
        .expect("MQTT config is invalid");

    // Load topics and pin config
    let mut topics: Vec<Topic> = Vec::new();
    let mut i = 0;
    for raw_topic in conf
        .get(&Yaml::String(String::from("topics")))
        .expect("Topics config missing")
        .as_vec()
        .expect("Topics config invalid. Expected an array")
    {
        i += 1;
        let topic = raw_topic.as_hash().expect(
            &format!("Invalid topic configuration found for topic at index {}. All topics should be objects with topic, pin and mode!", i));
        topics.push(
            Topic { 
                topic: String::from(
                    topic
                        .get(&Yaml::String(String::from("topic")))
                        .expect(&format!("Invalid topic configuration found at index {}. The topic name is missing", i))
                        .as_str()
                        .expect(&format!("Invalid topic configuration found index {}. The topic name should be a string", i)),
                ),
                pin: topic
                        .get(&Yaml::String(String::from("pin")))
                        .expect(&format!("Invalid topic configuration found at index {}. A pin is required", i))
                        .as_i64()
                        .expect(&format!("Invalid topic configuration found at index {}. The pin should be a 16 bit integer (0-65535)", i)) as u8,
                off_timeout: topic
                        .get(&Yaml::String(String::from("pin")))
                        .unwrap_or(&Yaml::Integer(-1))
                        .as_i64()
                        .expect(&format!("Invalid topic configuration found at index {}. The pin should be a 16 bit integer (0-65535)", i)),
                mode: if topic
                        .get(&Yaml::String(String::from("mode")))
                        .expect(&format!("Invalid topic configuration found at index {}. Mode is unset", i))
                        .as_str()
                        .expect(&format!("Invalid topic configuration found at index {}. Mode should be a string of either 'in' or 'out'", i)) == "in"
                    { PinMode::IN } else { PinMode::OUT },
            }
        );
    }

    // Create the config struct
    Ok(Config {
        mqtt: MqttConfig {
            host: String::from(
                mqtt_conf
                    .get(&Yaml::String(String::from("host")))
                    .expect("Host config missing")
                    .as_str()
                    .expect("Host configuration is invalid"),
            ),
            port: mqtt_conf
                .get(&Yaml::String(String::from("port")))
                .unwrap_or(&Yaml::Integer(1883))
                .as_i64()
                .expect("Invalid port configuration. Expected integer") as u16,
            authentication: mqtt_conf
                .get(&Yaml::String(String::from("authentication")))
                .unwrap_or(&Yaml::Boolean(false))
                .as_bool()
                .expect("Authentication configuration value incorrect"),
            user: String::from(
                mqtt_conf
                    .get(&Yaml::String(String::from("user")))
                    .unwrap_or(&Yaml::String(String::from("")))
                    .as_str()
                    .expect("User configuration invalid"),
            ),
            password: String::from(
                mqtt_conf
                    .get(&Yaml::String(String::from("password")))
                    .unwrap_or(&Yaml::String(String::from("")))
                    .as_str()
                    .expect("Password configuration invalid"),
            ),
        },
        topics: topics,
        poll_interval: conf
            .get(&Yaml::String(String::from("pollInterval")))
            .expect("Missing config for pollInterval")
            .as_i64()
            .expect("pollInterval is not an integer value") as u64,
    })
}
