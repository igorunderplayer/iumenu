use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub config: PathBuf,
}

pub fn parse_arguments() -> Args {
    Args::parse()
}
