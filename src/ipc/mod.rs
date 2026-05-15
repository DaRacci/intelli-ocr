use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::types::CaptureRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    CaptureNow(CaptureRequest),
}

pub struct IpcServer {
    socket_path: PathBuf,
}

impl IpcServer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket_path: default_socket_path()?,
        })
    }

    pub fn socket_path(&self) -> &PathBuf {
        &self.socket_path
    }

    pub fn recv(&self) -> Result<IpcMessage> {
        todo!("ipc receive loop not implemented")
    }
}

pub fn send_trigger() -> Result<()> {
    todo!("ipc trigger client not implemented")
}

pub fn default_socket_path() -> Result<PathBuf> {
    todo!("xdg runtime socket path resolution not implemented")
}
