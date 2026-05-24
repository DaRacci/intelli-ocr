use anyhow::Result;
use clap::{Parser, Subcommand};
use tokio::signal;
use tokio::sync::mpsc::channel;

use crate::daemon;
use crate::ipc;
use crate::types::Runtime;

#[derive(Debug, Parser)]
#[command(name = "intelli-ocr")]
#[command(about = "Wayland OCR overlay daemon")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Daemon {
        #[arg(short, long, help = "Replace existing IPC socket if it exists", action = clap::ArgAction::SetTrue)]
        replace: bool,
    },
    Trigger,
}

pub async fn dispatch(command: Command, runtime: Runtime) -> Result<()> {
    let (shutdown_sender, shutdown_receiver) = channel(1);
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                shutdown_sender.send(()).await.unwrap();
            }
            Err(e) => {
                eprintln!("{e}")
            }
        }
    });

    match command {
        Command::Daemon { replace } => {
            let daemon = daemon::Daemon::new(shutdown_receiver, replace)?;
            daemon.run().await
        }
        Command::Trigger => ipc::send_trigger(runtime, shutdown_receiver).await,
    }
}
