use anyhow::Result;
use tokio::sync::mpsc::{Receiver, channel};

use crate::capture::grim_rs::GrimRsCaptureBackend;
use crate::capture::{CaptureBackend, CaptureRequest};
use crate::ipc::IpcServer;
use crate::overlay::{NullOverlayBackend, OverlayBackend};
use crate::selection::{DragSelectionStrategy, SelectionStrategy};
use crate::text::{AtSpiTextSource, NullTextMerger, TesseractTextSource, TextMerger, TextSource};
use crate::types::SessionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Idle,
    Capturing,
    Detecting,
    ShowingOverlay,
}

pub struct Daemon {
    capture_backend: Box<dyn CaptureBackend>,
    overlay_backend: Box<dyn OverlayBackend>,
    primary_text_source: Box<dyn TextSource>,
    fallback_text_source: Box<dyn TextSource>,
    text_merger: Box<dyn TextMerger>,
    selection_strategy: Box<dyn SelectionStrategy>,
    ipc_server: IpcServer,
    state: AppState,
}

impl Daemon {
    pub fn new(shutdown_receiver: Receiver<()>, replace: bool) -> Result<Self> {
        Ok(Self {
            capture_backend: Box::new(GrimRsCaptureBackend::new()?),
            overlay_backend: Box::new(NullOverlayBackend),
            primary_text_source: Box::new(AtSpiTextSource),
            fallback_text_source: Box::new(TesseractTextSource),
            text_merger: Box::new(NullTextMerger),
            selection_strategy: Box::new(DragSelectionStrategy),
            ipc_server: IpcServer::new(shutdown_receiver, replace)?,
            state: AppState::Idle,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        let (tx, rx) = channel(1);

        let receiver = {
            let ipc_server = self.ipc_server.clone();
            tokio::spawn(async move {
                println!("Starting IPC server...");
                if let Err(e) = ipc_server.recv(tx).await {
                    eprintln!("Error in IPC server: {e}");
                };
            })
        };

        let listener = tokio::spawn(async move {
            println!("Starting request handler...");
            if let Err(e) = self.run_request_handler(rx).await {
                eprintln!("Error in request handler: {e}");
            };
        });

        println!("Daemon is running and listening for IPC messages...");
        tokio::try_join!(listener, receiver)?;
        println!("Daemon is shutting down...");

        Ok(())
    }

    pub async fn run_request_handler(
        &mut self,
        mut rx: Receiver<crate::ipc::IpcMessage>,
    ) -> Result<()> {
        while let Some(message) = rx.recv().await {
            match message {
                crate::ipc::IpcMessage::CaptureNow(request) => {
                    println!("Received capture request: {request:?}");
                    let session_id = self.handle_capture_request(request)?;
                    println!("Started capture session with ID: {session_id:?}");
                }
            }
        }

        Ok(())
    }

    pub fn handle_capture_request(&mut self, request: CaptureRequest) -> Result<SessionId> {
        self.state = AppState::Capturing;
        self.capture_backend.capture(&request)?;

        Ok(SessionId::new())
    }
}
