use anyhow::Result;

use crate::capture::{CaptureBackend, NullCaptureBackend};
use crate::ipc::IpcServer;
use crate::overlay::{NullOverlayBackend, OverlayBackend};
use crate::selection::{DragSelectionStrategy, SelectionStrategy};
use crate::text::{AtSpiTextSource, NullTextMerger, TesseractTextSource, TextMerger, TextSource};
use crate::types::{AppState, CaptureRequest, SessionId};

pub fn run() -> Result<()> {
    let mut daemon = Daemon::new(
        Box::new(NullCaptureBackend),
        Box::new(NullOverlayBackend),
        Box::new(AtSpiTextSource),
        Box::new(TesseractTextSource),
        Box::new(NullTextMerger),
        Box::new(DragSelectionStrategy),
        IpcServer::new()?,
    );

    daemon.run()
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

    pub fn run(&mut self) -> Result<()> {
        todo!("daemon event loop not implemented")
    }

    pub fn handle_capture_request(&mut self, _request: CaptureRequest) -> Result<SessionId> {
        todo!("capture session orchestration not implemented")
    }
}
