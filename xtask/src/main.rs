use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[macro_use]
mod utils;
mod bench;
mod prototyper;
mod test;

use crate::bench::BenchArg;
use crate::prototyper::PrototyperArg;
use crate::test::TestArg;

#[derive(Parser)]
#[clap(
    name = "xtask",
    about = "A task runner for building, running and testing Prototyper",
    long_about = None,
)]
struct Cli {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Prototyper(PrototyperArg),
    Test(TestArg),
    Bench(BenchArg),
}

fn main() -> ExitCode {
    if let Some(code) = match Cli::parse().cmd {
        Cmd::Prototyper(ref arg) => prototyper::run(arg),
        Cmd::Test(ref arg) => test::run(arg),
        Cmd::Bench(ref arg) => bench::run(arg),
    } {
        if code.success() {
            return ExitCode::SUCCESS;
        }
    }
    ExitCode::FAILURE
}
