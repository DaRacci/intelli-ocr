use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::daemon;
use crate::ipc;

#[derive(Debug, Parser)]
#[command(name = "intelli-ocr")]
#[command(about = "Wayland OCR overlay daemon")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Daemon,
    Trigger,
}

pub fn parse() -> Command {
    Cli::parse().command
}

pub fn dispatch(command: Command) -> Result<()> {
    match command {
        Command::Daemon => daemon::run(),
        Command::Trigger => ipc::send_trigger(),
    }
}
