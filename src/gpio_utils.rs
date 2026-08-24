use std::collections::HashMap;

use crate::conf::{PinMode, Topic};

/// Split topics into in and out pins. Returns them in this order
///
/// * `topics`: The topics to use
pub fn split_topics_and_configure_pins(topics: Vec<Topic>) -> (Vec<Topic>, Vec<Topic>) {
    let mut in_pins: Vec<Topic> = Vec::new();
    let mut out_pins: Vec<Topic> = Vec::new();
    for topic in topics {
        if topic.mode == PinMode::IN {
            in_pins.push(topic);
        } else {
            out_pins.push(topic);
        }
    }

    return (in_pins, out_pins);
}

pub fn read_pin_value(id: u16) {}

pub struct GPIOController {
    in_pins: HashMap<String, Topic>,
}

impl GPIOController {
    /// Create a new GPIO controller.
    /// Note that the move of ownership is intentional behaviour. Only create one instance of this controller!
    ///
    /// * `topics`: The topics that are going to be managed by this controller (All OUT mode pins
    /// are dropped without error)
    pub fn new(topics: Vec<Topic>) -> Self {
        let mut controller = GPIOController {
            in_pins: HashMap::new(),
        };

        // Build hash maps
        for topic in topics {
            if topic.mode == PinMode::IN {
                controller
                    .in_pins
                    .insert(String::from(topic.topic.as_str()), topic);
            }
        }

        return controller;
    }

    pub fn handle_event(&self) {}
}
