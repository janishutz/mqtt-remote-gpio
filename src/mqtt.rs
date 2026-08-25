use crate::conf::Config;
use crate::gpio_utils::{GPIOController, read_pin, split_topics_and_configure_pins};
use rumqttc::Packet::Publish;
use rumqttc::{Client, Event, MqttOptions, QoS};
use std::{thread, time::Duration};

/// MQTT connection handler
///
/// * `topics`: The MQTT topics to subscribe to
/// * `mqttoptions`: MQTT options for conenction
pub fn handler(config: Config) {
    // Configure pins
    let (mut input_pins, output_pins) = split_topics_and_configure_pins(config.topics);

    // Configure MQTT
    let mut mqttoptions = MqttOptions::new("mqtt", config.mqtt.host, config.mqtt.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    if config.mqtt.authentication {
        mqttoptions.set_credentials(config.mqtt.user, config.mqtt.password);
    }
    let (client, mut connection) = Client::new(mqttoptions, 10);

    // Subscribe to events
    for topic in output_pins.iter() {
        println!("Setting up topic {}", topic.topic);
        client
            .subscribe(topic.topic.as_str(), QoS::AtMostOnce)
            .unwrap_or_else(|x| println!("Setup failed, error: {:?}", x));
    }

    // Spawn thread to monitor pins
    thread::spawn(move || {
        let mut pin_state: Vec<u8> = vec![0; input_pins.len()];
        let off: Vec<u8> = b"OFF".to_vec();
        let on: Vec<u8> = b"ON".to_vec();
        loop {
            let mut i = 0;
            for topic in input_pins.iter_mut() {
                // TODO: handle fails
                let val = read_pin(&mut topic.gpio);
                if val != pin_state[i] {
                    pin_state[i] = val;
                    client
                        .publish(
                            String::from(&topic.topic),
                            QoS::AtLeastOnce,
                            false,
                            if val == 0 { off.clone() } else { on.clone() },
                        )
                        .unwrap_or_default();
                }
                i += 1;
            }
            thread::sleep(Duration::from_millis(config.poll_interval));
        }
    });

    // Main thread listens to topic updates
    let mut controller = GPIOController::new(output_pins);
    for (_, notification) in connection.iter().enumerate() {
        controller.iter_handler();
        let msg = notification.unwrap();
        if let Event::Incoming(val) = msg {
            if let Publish(content) = val {
                controller.handle_event(&content);
            }
        }
        // println!("Notification {:?}", msg);
        thread::sleep(Duration::from_millis(config.poll_interval / 2));
        // TODO: Handle pin value change instructions
    }
}
