use gpio::{GpioIn, GpioOut};
use rumqttc::Publish;
use std::collections::HashMap;

use crate::conf::{PinMode, Topic};

pub struct InputPin {
    pub gpio: gpio::sysfs::SysFsGpioInput,
    pub id: u16,
    pub topic: String,
}

pub struct OutputPin {
    gpio: gpio::sysfs::SysFsGpioOutput,
    off_timeout: u64,
    pub id: u16,
    pub topic: String,
}

/// Split topics into in and out pins. Returns them in this order
///
/// * `topics`: The topics to use
pub fn split_topics_and_configure_pins(topics: Vec<Topic>) -> (Vec<InputPin>, Vec<OutputPin>) {
    let mut in_pins: Vec<InputPin> = Vec::new();
    let mut out_pins: Vec<OutputPin> = Vec::new();
    let mut in_used: Vec<u16> = Vec::new();
    let mut out_used: Vec<u16> = Vec::new();
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
            in_pins.push(InputPin {
                topic: topic.topic,
                gpio: gpio::sysfs::SysFsGpioInput::open(topic.pin)
                    .expect(&format!("Unable to find GPIO pin {}", topic.pin)),
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
            out_pins.push(OutputPin {
                topic: topic.topic,
                gpio: gpio::sysfs::SysFsGpioOutput::open(topic.pin)
                    .expect(&format!("Unable to find GPIO pin {}", topic.pin)),
                id: topic.pin,
                off_timeout: topic.off_timeout,
            });
            out_used.push(topic.pin)
        }
    }

    return (in_pins, out_pins);
}

pub fn read_pin(pin: &mut gpio::sysfs::SysFsGpioInput) -> u8 {
    if let gpio::GpioValue::Low = pin.read_value().expect("Failed reading value") {
        return 1;
    } else {
        return 0;
    }
}

pub struct GPIOController {
    out_pins: HashMap<String, OutputPin>,
    topics: Vec<String>,
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
        };

        // Build hash maps
        for pin in pins {
            let topic = pin.topic.clone();
            controller.out_pins.insert(pin.topic.clone(), pin);
            controller.topics.push(topic);
        }

        return controller;
    }

    pub fn handle_event(&mut self, instruction: &Publish) {
        if self.topics.contains(&instruction.topic) {
            let pin = self
                .out_pins
                .get(&instruction.topic)
                .expect("Failed to load pins data");
            println!("Hello World, pin {}", pin.id);
            println!("Payload {}", instruction.payload.first().unwrap());
            // TODO: Off timeout
        }
    }
}
