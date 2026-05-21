use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, channel};

use crate::capture::{CaptureBackend, NullCaptureBackend};
use crate::ipc::IpcServer;
use crate::overlay::{NullOverlayBackend, OverlayBackend};
use crate::selection::{DragSelectionStrategy, SelectionStrategy};
use crate::text::{AtSpiTextSource, NullTextMerger, TesseractTextSource, TextMerger, TextSource};
use crate::types::{AppState, CaptureRequest, SessionId};

pub async fn run(shutdown_receiver: Receiver<()>, replace: bool) -> Result<()> {
    let daemon = Daemon::new(
        Box::new(NullCaptureBackend),
        Box::new(NullOverlayBackend),
        Box::new(AtSpiTextSource),
        Box::new(TesseractTextSource),
        Box::new(NullTextMerger),
        Box::new(DragSelectionStrategy),
        IpcServer::new(shutdown_receiver, replace)?,
    );

    daemon.run().await
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        capture_backend: Box<dyn CaptureBackend>,
        overlay_backend: Box<dyn OverlayBackend>,
        primary_text_source: Box<dyn TextSource>,
        fallback_text_source: Box<dyn TextSource>,
        text_merger: Box<dyn TextMerger>,
        selection_strategy: Box<dyn SelectionStrategy>,
        ipc_server: IpcServer,
    ) -> Self {
        Self {
            capture_backend,
            overlay_backend,
            primary_text_source,
            fallback_text_source,
            text_merger,
            selection_strategy,
            ipc_server,
            state: AppState::Idle,
        }
    }

    pub async fn run(self) -> Result<()> {
        let (tx, rx) = channel(1);
        let self_arc = Arc::new(Mutex::new(self));

        let receiver = {
            let self_arc = Arc::clone(&self_arc);
            tokio::spawn(async move {
                let guard = self_arc.lock().await;
                if let Err(e) = guard.ipc_server.recv(tx).await {
                    eprintln!("Error in IPC server: {e}");
                };
            })
        };

        let listener = {
            let self_arc = Arc::clone(&self_arc);
            tokio::spawn(async move {
                let mut guard = self_arc.lock().await;
                if let Err(e) = guard.run_request_handler(rx).await {
                    eprintln!("Error in request handler: {e}");
                };
            })
        };

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
            println!("Received IPC message: {message:?}");
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

        todo!("capture session orchestration not implemented")
    }
}
