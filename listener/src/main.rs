use rumqttc::{Client, Event, MqttOptions, QoS};
use std::time::Duration;

fn main() {
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "test.mosquito.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    // mqttoptions.set_credentials("", "");

    let (client, mut connection) = Client::new(mqttoptions, 10);
    let topic = "testing/get/17";
    println!("listening to topic {}", topic);
    client.subscribe(topic, QoS::AtMostOnce).unwrap();

    // Iterate to poll the eventloop for connection progress
    for (_, notification) in connection.iter().enumerate() {
        let msg = notification.unwrap();
        if let Event::Incoming(val) = msg {
            println!("Notification = {:?}", val);
        }
    }
}
