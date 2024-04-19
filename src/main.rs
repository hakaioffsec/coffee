#![feature(c_variadic)]
#![feature(core_intrinsics)]

use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;

mod loader;
use loader::{beacon_pack::BeaconPack, Coffee};

#[derive(Parser, Debug)]
#[command(name = "Coffee")]
#[command(version = "1.0")]
#[command(about = "Coffee: A COFF loader made in Rust")]
struct Args {
    /// Path to the Beacon Object File (BOF)
    #[arg(short, long)]
    bof_path: PathBuf,

    /// The entrypoint name to execute in case of a custom entrypoint name
    #[arg(short, long)]
    #[clap(default_value = "go")]
    entrypoint: Option<String>,

    /// Verbosity level, 0 = ERROR, 1 = WARN, 2 = INFO, 3 = DEBUG, 4 = TRACE
    #[arg(short, long)]
    #[clap(default_value = "1")]
    verbosity: Option<u8>,

    /// Arguments to the BOF passed after the "--" delimiter, supported types are: str, wstr, int, short
    #[clap(last = true)]
    args: Vec<String>,
}

/// Unhexilify a string of hexadecimal characters to pass as arguments to the BOF
fn unhexilify_args(value: &str) -> Result<Vec<u8>> {
    if value.len() % 2 != 0 {
        panic!("Invalid argument hexadecimal string");
    }

    let bytes: Result<Vec<u8>, _> = (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
        .collect();

    Ok(bytes?)
}

/// Hexilifies a list of arguments passed after the delimiter with the type and value
fn hexlify_args(args: Vec<String>) -> Result<String> {
    let mut beacon_pack = BeaconPack::new();

    for arg in args {
        let tokens: Vec<&str> = arg.splitn(2, ':').collect();
        if tokens.len() != 2 {
            panic!("Invalid argument format! Expected: <type>:<value>, Example: str:HelloWorld or int:123");
        }

        let argument_type = tokens[0].trim();
        let argument_value = tokens[1].trim();

        match argument_type {
            "str" => beacon_pack.add_str(argument_value),
            "wstr" => beacon_pack.add_wstr(argument_value),
            "int" => {
                if let Ok(int_value) = argument_value.parse::<i32>() {
                    beacon_pack.add_int(int_value);
                } else {
                    panic!("Invalid integer value");
                }
            }
            "short" => {
                if let Ok(short_value) = argument_value.parse::<i16>() {
                    beacon_pack.add_short(short_value);
                } else {
                    panic!("Invalid short value");
                }
            }
            _ => panic!("Invalid argument type"),
        }
    }

    let hex_buffer = beacon_pack
        .get_buffer()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect();

    Ok(hex_buffer)
}

/// Main function
fn main() -> Result<()> {
    // Initialize color_eyre
    color_eyre::install()?;

    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(match args.verbosity.unwrap_or(0) {
            0 => LevelFilter::ERROR,
            1 => LevelFilter::WARN,
            2 => LevelFilter::INFO,
            3 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        })
        .without_time()
        .init();

    // Get arguments after the delimiter --
    let after_delimiter_args: Vec<String> = args
        .args
        .split(|arg| arg == "--")
        .flat_map(|args| args.iter())
        .map(|arg| arg.clone())
        .collect();

    debug!("Arguments: {:?}", after_delimiter_args);

    // Hexlify the arguments
    let arguments = hexlify_args(after_delimiter_args)?;
    debug!("Hexilified arguments: {}", arguments);

    // Load the buffer from the BOF path
    let coff_buffer = std::fs::read(&args.bof_path)?;

    // Unhexlify the arguments
    let unhexilified = unhexilify_args(arguments.as_str())?;
    debug!("Unhexilified arguments: {:?}", unhexilified);

    // Execute the BOF
    // TODO: Arguments as Option<&[u8]>
    info!("Loading BOF: {}", args.bof_path.display());
    let output = Coffee::new(coff_buffer.as_slice())?.execute(
        Some(unhexilified.as_ptr()),
        Some(unhexilified.len()),
        args.entrypoint,
    )?;

    println!("Execution output: {}", output);

    Ok(())
}
