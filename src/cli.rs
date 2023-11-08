use clap::Parser;

// TODO: use cargo.toml info
#[derive(Parser)]
#[command(name = "choochoo")]
#[command(author = "Alix Macdonald <alixmacdonald10@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Runs Leetcode problems.", long_about = None)]
pub struct RunnerArgs {

    //TODO: add interactive way to select and run solutions

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    
}