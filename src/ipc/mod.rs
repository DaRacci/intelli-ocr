use std::{
    io,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    sync::mpsc::{Receiver, Sender},
};
use xdg::BaseDirectories;

use crate::capture::CaptureRequest;

const IPC_DIR_NAME: &str = "intelli-ocr";
const IPC_SOCKET_FILENAME: &str = "ipc.sock";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    CaptureNow(CaptureRequest),
}

#[derive(Debug)]
pub struct IpcServer {
    socket_path: PathBuf,
    replace: bool,
}

impl IpcServer {
    pub fn new(mut shutdown_receiver: Receiver<()>, replace: bool) -> Result<Self> {
        let socket_path = default_socket_path()?;
        let shutdown_socket_path = socket_path.clone();
        tokio::spawn(async move {
            match shutdown_receiver.recv().await {
                Some(()) => tokio::fs::remove_file(shutdown_socket_path)
                    .await
                    .expect("No bad"),
                None => {
                    eprintln!("received nothing from the shutdown receiver")
                }
            };

            exit(0);
        });

        Ok(Self {
            socket_path,
            replace,
        })
    }

    pub async fn recv(&self, channel: Sender<IpcMessage>) -> Result<()> {
        self.cleanup_socket()?;
        let socket_path = self.socket_path.clone();
        let listener = UnixListener::bind(&socket_path)?;

        while let Ok((mut stream, _)) = listener.accept().await {
            println!("Listening on {socket_path:?}");
            let mut buffer: [u8; 1024] = [0u8; 1024];
            let channel = channel.clone();
            tokio::spawn(async move {
                loop {
                    match stream.read(&mut buffer).await {
                        Ok(n) => {
                            if n == 0 {
                                break;
                            }

                            let message_json = String::from_utf8_lossy(&buffer[..n]);
                            println!("Received message: {message_json}");
                            match serde_json::from_str::<IpcMessage>(&message_json) {
                                Ok(message) => {
                                    println!("Parsed message: {message:?}");
                                    if let Err(e) = channel.send(message).await {
                                        eprintln!("Failed to send message to channel; error: {e}");
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse message; error: {e}");
                                }
                            };
                        }

                        Err(e) => {
                            eprintln!("Error writing to client; error: {e}");
                            break;
                        }
                    }
                }
            });
        }

        Ok(())
    }

    fn cleanup_socket(&self) -> Result<()> {
        let socket_path = &self.socket_path;
        if socket_path.exists() {
            if !socket_is_listening(&socket_path)? {
                println!("Removing stale IPC socket at {socket_path:?}");
                std::fs::remove_file(&socket_path)?;
            } else if self.replace {
                println!("Replacing IPC Socket at {socket_path:?}.");
                std::fs::remove_file(&socket_path)?;
            } else {
                return Err(anyhow::anyhow!(
                    "IPC socket is in use at {socket_path:?}. Use --replace to remove it."
                ));
            }
        }

        Ok(())
    }
}

pub async fn send_trigger(mut shutdown_receiver: Receiver<()>) -> Result<()> {
    let socket_path = default_socket_path()?;
    if !socket_path.exists() || !socket_is_listening(&socket_path)? {
        return Err(anyhow::anyhow!(
            "No IPC socket found at {socket_path:?} or socket is not listening"
        ));
    }

    let mut unixstream = UnixStream::connect(&socket_path).await?;
    println!("Connected to IPC socket at {socket_path:?}");

    tokio::spawn(async move {
        match shutdown_receiver.recv().await {
            Some(()) => {
                println!("Shutting down trigger sender");
                exit(1);
            }
            None => {
                eprintln!("received nothing from the shutdown receiver")
            }
        }
    });

    let message = IpcMessage::CaptureNow(CaptureRequest { output_name: None });
    let message_json = serde_json::to_string(&message)?;
    unixstream.write_all(message_json.as_bytes()).await?;
    println!("Sent trigger message: {message_json}");

    Ok(())
}

pub fn default_socket_path() -> Result<PathBuf> {
    let socket_dir = BaseDirectories::with_prefix(IPC_DIR_NAME);
    let socket_file = socket_dir.place_runtime_file(IPC_SOCKET_FILENAME)?;

    Ok(socket_file)
}

fn socket_is_listening(path: &PathBuf) -> io::Result<bool> {
    match std::os::unix::net::UnixStream::connect(path) {
        Ok(_) => Ok(true), // connected -> listener present
        Err(e) => {
            // ConnectionRefused usually means file exists but no listener
            if e.kind() == io::ErrorKind::ConnectionRefused {
                Ok(false)
            } else {
                Err(e) // e.g., NotFound, PermissionDenied, etc.
            }
        }
    }
}
