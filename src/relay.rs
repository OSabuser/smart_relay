use std::{collections::HashMap, fmt::Display};

use clap::ValueEnum;

pub const RELAYS_RANGE: (u8, u8) = (1, 18);

pub struct RelayArray {
    state: HashMap<u8, RelayState>,
}

impl RelayArray {
    pub fn new() -> RelayArray {
        let mut state = HashMap::new();

        for i in RELAYS_RANGE.0..=RELAYS_RANGE.1 {
            state.insert(i, RelayState::Off);
        }
        RelayArray { state: state }
    }

    pub fn show_state(&self, relay_range: &Vec<u8>) -> Option<String> {
        let mut result = String::new();
        for relay in relay_range {
            if let Some(state) = self.state.get(relay) {
                println!("{} is {}", relay, state);
                result.push_str(&format!("{} is {}\n", relay, state));
            } else {
                println!("Relay {} is not found", relay);
                return None;
            }
        }
        Some(result)
    }

    pub fn set_state(&mut self, relay_range: &Vec<u8>, state: RelayState) -> Option<String> {
        let mut result = String::new();
        for relay in relay_range {
            if let Some(relay_state) = self.state.get_mut(relay) {
                *relay_state = state.clone();
                result.push_str(&format!("{} is {}\n", relay, state));
            } else {
                println!("Relay {} is not found", relay);
                return None;
            }
        }
        Some(result)
    }

    pub fn serialize_state(&self) -> String {
        let mut relay_state: u32 = 0b000000000000000000;
        for (relay, state) in self.state.iter() {
            if *state == RelayState::On {
                relay_state |= 1 << (relay - 1);
            }
        }
        println!("Binary relay state is: {:b}", relay_state);
        println!("Hex relay state is: {:x}", relay_state);

        format!(" {:x}", relay_state)
    }
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
pub enum RelayState {
    On,
    Off,
}

impl Display for RelayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelayState::On => write!(f, "ON"),
            RelayState::Off => write!(f, "OFF"),
        }
    }
}
