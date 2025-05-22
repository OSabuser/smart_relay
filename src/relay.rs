use std::{collections::HashMap, fmt::Display, time::Duration};

use ascii_converter::decimals_to_string;
use clap::ValueEnum;

use crate::communication::SerialInterface;

pub const RELAYS_RANGE: (u8, u8) = (1, 18);

pub struct RelayArray {
    state: HashMap<u8, RelayState>,
    serial_interface: SerialInterface,
}

impl RelayArray {
    pub fn new(port_name: &str, baudrate: u32, timeout: Duration) -> RelayArray {
        let mut state = HashMap::new();

        for i in RELAYS_RANGE.0..=RELAYS_RANGE.1 {
            state.insert(i, RelayState::Off);
        }

        let interface = SerialInterface::new(port_name, baudrate, timeout);
        RelayArray {
            state: state,
            serial_interface: interface,
        }
    }

    pub fn print_local_state(&self) {
        println!("Local state: {:0>8}", self.serialize_local_state());
    }

    pub fn update_local_state(
        &mut self,
        relay_range: &Vec<u8>,
        state: RelayState,
    ) -> Option<String> {
        // Обновление локального состояния реле
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

    pub fn export_local_state(&self, relay_range: &Vec<u8>) -> String {
        let mut result = String::new();
        result.push_str("; Test ini comment\n");
        result.push_str("[RELAYS]\n");
        for relay in relay_range {
            if let Some(state) = self.state.get(relay) {
                result.push_str(&format!("{}={}\n", relay, state));
            } else {
                result.push_str(&format!("{}=ERR\n", relay));
            }
        }
        result
    }

    /// Сериализация локального состояния реле
    pub fn serialize_local_state(&self) -> String {
        let mut relay_state: u32 = 0b000000000000000000;
        for (relay, state) in self.state.iter() {
            if *state == RelayState::On {
                relay_state |= 1 << (relay - 1);
            }
        }

        format!("{:x}", relay_state)
    }

    /// Отправка обновленного состояния реле на интерфейсную плату
    pub fn push_state_to_remote(
        &mut self,
        relay_range: &Vec<u8>,
        state: RelayState,
    ) -> Result<(), String> {
        // Получение состояния реле с интерфейсной платы, обновление локального состояния
        self.fetch_state_from_remote()?;

        // Внесение пользовательских изменений в локальное состояние
        if let None = self.update_local_state(relay_range, state) {
            return Err("Invalid relay range".to_string());
        }

        // Отправка обновленного состояния реле на интерфейсную плату
        let local_state = self.serialize_local_state();
        let data = format!("set {}\r\n", local_state);
        self.serial_interface.write_data(data.as_bytes())?;

        return Ok(());
    }

    /// Получение состояния реле с интерфейсной платы
    pub fn fetch_state_from_remote(&mut self) -> Result<(), String> {
        let mut serial_buf: Vec<u8> = vec![0; 26];

        // Отправка запроса на интерфейсную плату
        self.serial_interface.write_data(b"get\r\n")?;

        // Чтение ответа
        self.serial_interface.read_data(serial_buf.as_mut_slice())?;

        // Очистка буфера - приемника
        self.serial_interface.clear_input_buffer()?;

        // Оставляем только ASCII символы
        serial_buf.retain(|value| *value >= 48 && *value <= 128);

        // Сырые данные в ASCII строку
        if let Ok(x) = decimals_to_string(&serial_buf) {
            let ascii_string_parts = x.split(":").collect::<Vec<&str>>();

            // Справа от делиметера - число - состояние
            if let Some(x) = ascii_string_parts.get(1) {
                // Преобразование в hex
                //let hex_number = i32::from_str_radix(x, 16).unwrap();

                if let Ok(hex_number) = i32::from_str_radix(x, 16) {
                    // Обновление локального состояния реле
                    for i in RELAYS_RANGE.0..=RELAYS_RANGE.1 {
                        self.state.insert(
                            i,
                            if hex_number & (1 << (i - 1)) != 0 {
                                RelayState::On
                            } else {
                                RelayState::Off
                            },
                        );
                    }
                    return Ok(());
                }
                return Err("Failed to convert data to hex value".to_string());
            } else {
                return Err("Failed to split recieved string".to_string());
            }
        } else {
            return Err("Failed to convert data to ASCII string".to_string());
        }
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
