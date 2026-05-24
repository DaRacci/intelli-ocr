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

#[tokio::main]
async fn main() -> Result<()> {
    let command = cli::parse();
    cli::dispatch(command).await?;

    Ok(())
}
