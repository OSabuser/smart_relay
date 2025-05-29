use crate::relay::{RelayArray, RelayState, RELAYS_RANGE};
use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand, ValueEnum};
use std::{thread, time::Duration};

pub mod communication;
pub mod relay;

// TODO: crate thiserror
// Создать единый тип ошибок, вклюащий в себя ошибки от коммуникации и от реле

#[derive(Parser, Debug, Clone)]
#[command(
    name = "SmartRelay CLI tool",
    version = "0.0.1",
    about = "Check and control relay states"
)]
#[command(author = "Dmitry Akimov MU LLC")]
struct AppArgs {
    #[command(subcommand)]
    cmd: RelayCommand,
}

#[derive(Subcommand, Debug, Clone)]
enum RelayCommand {
    /// Get (relay_range) relays state
    GetState {
        /// Range of relays.
        /// Must be in range 1-18.
        /// Example: 1-3; 1,2,3, 7-11, 18; 4-5,7.
        relay_range: String,
    },

    /// Set (relay_range) relays state to (state)
    SetState {
        /// Range of relays.
        /// Must be in range 1-18.
        /// Example: 1-3; 1,2,3, 7-11, 18; 4-5,7.
        relay_range: String,
        /// Relay state
        #[arg(value_enum)]
        state: RelayState,
    },
}

// TODO: режим verbose
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Для отладки
    // let ports = serialport::available_ports().expect("No ports found!");
    // for p in ports {
    //     println!("{}", p.port_name);
    // }

    let args = AppArgs::parse();

    // TODO: работа с Native tty
    let mut relay_array = RelayArray::new("/dev/ttyS4", 9600, Duration::from_millis(1000));

    'handshake_loop:
    for attempt in 0..=1 {
        if let Err(_) = relay_array.say_handshake() {
            if attempt == 1 {
                return Err("Unable to establish connection with IMv1!".into());
            } 
        } else {
            println!("; smart_relay v0.1.0");
            println!("; Interface board version: IMv1");
            println!("; status: Connected");
            break 'handshake_loop;
        }
    }

    relay_array.fetch_state_from_remote()?;

    match args.cmd {
        RelayCommand::GetState { relay_range } => {
            match create_list_of_relay_numbers(&relay_range) {
                Ok(relays_list) => {
                    relay_array.fetch_state_from_remote()?;
                    //TODO: объединить с print_local_state
                    println!("{}", relay_array.export_local_state(&relays_list));
                }

                // TODO: запаковать в функцию
                Err(e) => {
                    let mut cmd = AppArgs::command();
                    if e == RangeError::InvalidRange {
                        cmd.error(
                            ErrorKind::InvalidValue,
                            "The range must be like a,b,c or x,y-z,a,f where y < z",
                        )
                        .exit();
                    } else {
                        cmd.error(
                            ErrorKind::InvalidValue,
                            "Relay number must be less than 18 and greater than 0!",
                        )
                        .exit();
                    }
                }
            }
        }
        RelayCommand::SetState { relay_range, state } => {
            match create_list_of_relay_numbers(&relay_range) {
                Ok(relays_list) => {
                    relay_array.push_state_to_remote(&relays_list, state)?;
                    //TODO: объединить с export_local_state
                    relay_array.print_local_state(&relays_list);
                }
                // TODO: запаковать в функцию
                Err(e) => {
                    let mut cmd = AppArgs::command();
                    if e == RangeError::InvalidRange {
                        cmd.error(
                            ErrorKind::InvalidValue,
                            "The range must be like a,b,c or x,y-z,a,f where y < z",
                        )
                        .exit();
                    } else {
                        cmd.error(
                            ErrorKind::InvalidValue,
                            "Relay number must be less than 18 and greater than 0!",
                        )
                        .exit();
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_list_of_relay_numbers(s: &str) -> Result<Vec<u8>, RangeError> {
    if let Ok(mut range) = range_parser::parse(s) {
        range.sort();
        range.dedup();

        let relay_range = RELAYS_RANGE.0..=RELAYS_RANGE.1;
        for relay_number in range.iter() {
            if !relay_range.contains(relay_number) {
                return Err(RangeError::InvalidValue);
            }
        }

        return Ok(range);
    }

    Err(RangeError::InvalidRange)
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
enum RangeError {
    InvalidRange,
    InvalidValue,
}
