use std::{fmt::Display, ops::RangeInclusive};

use clap::{Parser, Subcommand, ValueEnum};

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
        #[arg(value_parser = port_in_range)]
        relay_range: String,
        /// Relay state
        #[arg(value_enum)]
        state: RelayState,
    },
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn main() {
    let args = AppArgs::parse();
    dbg!(args.clone());

    // Check for verbose mode
    if args.verbose {
        println!("Verbose mode is enabled");
    }

    match args.cmd {
        RelayCommand::GetState { relay_range } => {
            println!("Getting value from relay range:{}", relay_range)
        }
        RelayCommand::SetState { relay_range, state } => {
            println!("Setting value {} for relay range: {}", state, relay_range)
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
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
