use std::fmt::Display;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum, error::ErrorKind};

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

    // Check for verbose mode
    if args.verbose {
        println!("Verbose mode is enabled");
    }

    match args.cmd {
        RelayCommand::GetState { relay_range } => match parse_the_range(&relay_range) {
            Ok(relays_list) => println!("Get the range{:?}", relays_list),
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
        RelayCommand::SetState { relay_range, state } => match parse_the_range(&relay_range) {
            Ok(relays_list) => println!("Get the range{:?}", relays_list),
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
    }

    Ok(())
}

const TOTAL_RELAYS: u8 = 18;

fn parse_the_range(s: &str) -> Result<Vec<u8>, RangeError> {
    if let Ok(mut range) = range_parser::parse(s) {
        range.sort();
        range.dedup();

        match range.get(range.len() - 1) {
            Some(last) => {
                if last > &TOTAL_RELAYS || last < &1 {
                    return Err(RangeError::InvalidValue);
                }
                return Ok(range);
            }
            None => return Err(RangeError::InvalidRange),
        }
    }

    Err(RangeError::InvalidRange)
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
enum RangeError {
    InvalidRange,
    InvalidValue,
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
enum RelayState {
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
