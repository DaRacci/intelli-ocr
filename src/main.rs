mod capture;
mod cli;
mod clipboard;
mod compositor_backend;
mod daemon;
mod datasets;
mod error;
mod ipc;
mod overlay;
mod selection;
#[cfg(test)]
mod test_lib;
mod text;
mod types;

use anyhow::Result;
use clap::Parser;

use crate::{cli::Cli, types::Runtime};

#[tokio::main]
async fn main() -> Result<()> {
    let runtime = Runtime::new()?;
    let command = Cli::parse().command;
    cli::dispatch(command, runtime).await?;

    Ok(())
}
