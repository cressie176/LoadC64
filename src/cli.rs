use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value = "./games")]
    pub games_dir: PathBuf,

    #[arg(long, default_value = "vice/bin/x64sc")]
    pub vice_path: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
