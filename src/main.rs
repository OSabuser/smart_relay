use crate::relay::{RELAYS_RANGE, RelayArray, RelayState};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum, error::ErrorKind};

pub mod relay;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "SmartRelay CLI tool",
    version = "0.0.1",
    about = "Check and control relay states"
)]
#[command(author = "Dmitry Akimov MU LLC")]
struct AppArgs {
    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = AppArgs::parse();

    //dbg!(args.clone());

    let mut relay_array = RelayArray::new();

    // Check for verbose mode
    if args.verbose {
        println!("Verbose mode is enabled");
    }

    match args.cmd {
        RelayCommand::GetState { relay_range } => match get_list_of_relay_numbers(&relay_range) {
            Ok(relays_list) => {
                println!("Get the range{:?}", relays_list);
                // TODO: получение состояния требуемых реле
                relay_array.show_state(&relays_list);
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
        },
        RelayCommand::SetState { relay_range, state } => {
            match get_list_of_relay_numbers(&relay_range) {
                Ok(relays_list) => {
                    println!("Set the range{:?}", relays_list);
                    // TODO: установка состояний требуемых реле
                    relay_array.show_state(&relays_list);
                    relay_array.set_state(&relays_list, state);
                    relay_array.show_state(&relays_list);
                    let s = relay_array.serialize_state();
                    println!("Serialized state: {}", s);
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

fn get_list_of_relay_numbers(s: &str) -> Result<Vec<u8>, RangeError> {
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
