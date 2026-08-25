use rppal::gpio;
use rumqttc::Publish;
use std::{collections::HashMap, time::SystemTime};

use crate::conf::{PinMode, Topic};

pub struct InputPin {
    pub gpio: gpio::InputPin,
    pub id: u8,
    pub topic: String,
}

pub struct OutputPin {
    gpio: gpio::OutputPin,
    off_timeout: u64,
    pub id: u8,
    pub topic: String,
}

/// Split topics into in and out pins. Returns them in this order
///
/// * `topics`: The topics to use
pub fn split_topics_and_configure_pins(topics: Vec<Topic>) -> (Vec<InputPin>, Vec<OutputPin>) {
    let mut in_pins: Vec<InputPin> = Vec::new();
    let mut out_pins: Vec<OutputPin> = Vec::new();
    let mut in_used: Vec<u8> = Vec::new();
    let mut out_used: Vec<u8> = Vec::new();
    for topic in topics {
        if topic.mode == PinMode::IN {
            if in_used.contains(&topic.pin) {
                println!(
                    "Warning: Pin {} used more than once, all further uses dropped",
                    topic.pin
                );
                continue;
            } else if out_used.contains(&topic.pin) {
                panic!("Pin {} used for both input and output!", topic.pin)
            }
            let pin = gpio::Gpio::new().expect("GPIO pin access failed");
            in_pins.push(InputPin {
                topic: topic.topic,
                gpio: pin
                    .get(topic.pin)
                    .expect(&format!("Unable to find GPIO pin {}", topic.pin))
                    .into_input(),
                id: topic.pin,
            });
            in_used.push(topic.pin)
        } else {
            if out_used.contains(&topic.pin) {
                println!(
                    "Warning: Pin {} used more than once, all further uses dropped",
                    topic.pin
                );
                continue;
            } else if in_used.contains(&topic.pin) {
                panic!("Pin {} used for both input and output!", topic.pin)
            }
            let pin = gpio::Gpio::new().expect("GPIO pin access failed");
            out_pins.push(OutputPin {
                topic: topic.topic,
                gpio: pin
                    .get(topic.pin)
                    .expect(&format!("Unable to find GPIO pin {}", topic.pin))
                    .into_output_low(),
                id: topic.pin,
                off_timeout: topic.off_timeout,
            });
            out_used.push(topic.pin)
        }
    }

    return (in_pins, out_pins);
}

pub fn read_pin(pin: &mut gpio::InputPin) -> u8 {
    // TODO: Consider moving to interrupt based solution
    if let gpio::Level::High = pin.read() {
        return 1;
    } else {
        return 0;
    }
}

pub struct GPIOController {
    out_pins: HashMap<String, OutputPin>,
    topics: Vec<String>,
    timeout_topics: Vec<(String, SystemTime, u64)>,
}

impl GPIOController {
    /// Create a new GPIO controller.
    /// Note that the move of ownership is intentional behaviour. Only create one instance of this controller!
    ///
    /// * `pins`: The pins that are going to be managed by this controller
    pub fn new(pins: Vec<OutputPin>) -> Self {
        let mut controller = GPIOController {
            out_pins: HashMap::new(),
            topics: Vec::new(),
            timeout_topics: Vec::new(),
        };

        // Build hash maps
        for pin in pins {
            let topic = pin.topic.clone();
            controller.out_pins.insert(pin.topic.clone(), pin);
            controller.topics.push(topic);
        }

        return controller;
    }

    pub fn iter_handler(&mut self) {
        for i in 0..self.timeout_topics.len() {
            let topic = &self.timeout_topics[i];
            if topic.1.elapsed().unwrap().as_millis() > (topic.2 as u128) {
                let name = topic.0.clone();
                self.timeout_topics.remove(i);
                self.out_pins
                    .get_mut(&name)
                    .expect("Failed to retrieve pin")
                    .gpio
                    .set_low()
            }
        }
    }

    pub fn handle_event(&mut self, instruction: &Publish) {
        if self.topics.contains(&instruction.topic) {
            let pin = self
                .out_pins
                .get(&instruction.topic)
                .expect("Failed to load pins data");
            println!("Hello World, pin {}", pin.id);
            let mut instr = "".to_string();
            for char in &instruction.payload {
                instr.push_str(&char.to_string())
            }
            println!("{}", instr);
            if pin.off_timeout > 0 {
                self.timeout_topics
                    .push((pin.topic.clone(), SystemTime::now(), pin.off_timeout));
            }
        }
    }
}
