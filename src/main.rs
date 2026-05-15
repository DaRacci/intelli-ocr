mod capture;
mod cli;
mod clipboard;
mod daemon;
mod datasets;
mod error;
mod ipc;

mod overlay;
mod selection;
mod text;
mod types;

use anyhow::Result;

fn main() -> Result<()> {
    let command = cli::parse();
    cli::dispatch(command)
}
