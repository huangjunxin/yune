use std::{env, process::ExitCode};

mod args;
mod fixture;
mod render;
mod rime_frontend;
mod sample_core;
mod transcript;

use args::Command;
use fixture::check_fixture;
use sample_core::{run_sequence, DEFAULT_SEQUENCE};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    match Command::parse(&args)? {
        Command::Run { sequence } => {
            let output = run_sequence(&sequence)?;
            println!("{}", output.to_json());
            Ok(())
        }
        Command::Check { fixture } => check_fixture(&fixture),
        Command::Help => {
            render::print_help();
            Ok(())
        }
    }
}

pub(crate) fn default_sequence() -> &'static str {
    DEFAULT_SEQUENCE
}
