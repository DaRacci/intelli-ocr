use anyhow::Result;

use crate::{capture::CaptureFrame, selection::SelectionResult, text::TextFragment, types::Point};

#[derive(Debug, Clone)]
pub struct OverlayState {
    pub frame: CaptureFrame,
    pub fragments: Vec<TextFragment>,
    pub selection: Option<SelectionResult>,
}

#[derive(Debug, Clone)]
pub enum OverlayCommand {
    ReplaceState(OverlayState),
    UpdateSelection(SelectionResult),
    ClearSelection,
}

#[derive(Debug, Clone)]
pub enum OverlayEvent {
    EscapePressed,
    CopyPressed,
    DragStarted(Point),
    DragUpdated(Point),
    DragFinished(Point),
}

pub trait OverlayBackend: Send {
    fn show(&mut self, state: &OverlayState) -> Result<()>;
    fn update(&mut self, command: OverlayCommand) -> Result<()>;
    fn next_event(&mut self) -> Result<OverlayEvent>;
    fn close(&mut self) -> Result<()>;
}

pub struct NullOverlayBackend;

impl OverlayBackend for NullOverlayBackend {
    fn show(&mut self, _state: &OverlayState) -> Result<()> {
        todo!("overlay show not implemented")
    }

    fn update(&mut self, _command: OverlayCommand) -> Result<()> {
        todo!("overlay update not implemented")
    }

    fn next_event(&mut self) -> Result<OverlayEvent> {
        todo!("overlay event loop not implemented")
    }

    fn close(&mut self) -> Result<()> {
        todo!("overlay close not implemented")
    }
}
