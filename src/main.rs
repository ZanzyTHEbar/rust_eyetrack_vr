// NOTE: this rule is not supported by rust-analyzer or JetBrains Rust plugin go to definition/refactoring tools so disable it until it's supported properly
#![allow(clippy::uninlined_format_args)]

extern crate log;
use crate::prelude::*;
use eyetrackvr::root_process::EyeTrackVR;

mod error;
mod eyetrackvr;
mod logger;
mod prelude;
mod utils;

/// Main entry point
fn main() -> Result<()> {
    let process = EyeTrackVR::new();
    handle(run(process));
    Ok(())
}

/// Run the command passed in
fn run(process: EyeTrackVR) -> Result<()> {
    dc_stderr!(f!(
        "{} v{}",
        Color::new("EyeTrackVR").bold(),
        Color::new(env!("CARGO_PKG_VERSION")).bold()
    )
    .as_str());

    dc_stdout!(f!("EyetrackVR {}", Color::new("Rust Development").bold()).as_str());

    process.run()
}

/// Handle the result of the command
fn handle(result: Result<()>) {
    if let Err(error) = result {
        match error {
            Error::Generic(msg) => {
                dc_stderr!(&msg);
                std::process::exit(1)
            }
            Error::OperationCancelled(msg) => {
                dc_stdout!(f!("Operation cancelled: {}", Color::new(&msg).bold()).as_str());
            }
            Error::IO(error) => {
                dc_stderr!(&error.to_string());
                std::process::exit(1)
            }
        }
    }
}
