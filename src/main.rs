mod utils;
mod solutions;
mod cli;

use std::env;

use cli::RunnerArgs;
use utils::environ::DEBUG_LEVEL;

use clap::Parser;



fn main() {
    
    let cli = RunnerArgs::parse();

    match cli.debug {
        0 => {},
        _ => {
            println!("Debug mode on");
            env::set_var(DEBUG_LEVEL, "1");
        },
    }

    // TODO: add arg parser and interactive shell which lets you select the solution to run add env var for verbosity and mod solutions to suit
    solutions::two_sum::run();

}