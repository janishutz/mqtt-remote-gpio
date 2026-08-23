use rumqttc::{Client, MqttOptions, QoS};
use std::{fs, thread, time::Duration};

struct Topic {
    name: String,
    pin: u16,
}

fn main() {
    println!("mqtt-remote-gpio");

    // Read config file
    let conf = fs::read_to_string("config.yml").unwrap_or_else(|_| String::from("test"));
    println!("{}", conf);

    pin_setup();

    let mut mqttoptions = MqttOptions::new("mqtt", "10.0.9.60", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_credentials("mqtt", "PNS#Ka!kk8cb5uYYgXdGZkBvGPP24x");
    listener(
        vec![Topic {
            name: String::from("garage/test"),
            pin: 0,
        }],
        vec![Topic {
            name: String::from("garage/test"),
            pin: 0,
        }],
        mqttoptions,
    );
}

fn pin_setup() {
    // TODO: Check that a pin is not both in and out
    println!("Pin setup complete");
}

/// MQTT connection handler
///
/// * `topics`: The MQTT topics to subscribe to
/// * `mqttoptions`: MQTT options for conenction
fn listener(subscribe_topics: Vec<Topic>, publish_topics: Vec<Topic>, mqttoptions: MqttOptions) {
    let (client, mut connection) = Client::new(mqttoptions, 10);
    for topic in subscribe_topics {
        client.subscribe(topic.name, QoS::AtMostOnce).unwrap();
    }

    // Spawn thread to update pins
    thread::spawn(move || {
        loop {
            for topic in &publish_topics {
                client
                    .publish(&topic.name, QoS::AtLeastOnce, false, vec![0; 0 as usize])
                    .unwrap_or_default();
            }
        }
    });

    for (_, notification) in connection.iter().enumerate() {
        println!("{:?}", notification);
        // TODO: Handle pin value change instructions
    }
}
