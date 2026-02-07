use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value = "./games")]
    pub games_dir: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
