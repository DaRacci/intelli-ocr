pub mod relm4;

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
    Event(OverlayEvent),
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
    fn new() -> Result<Self>
    where
        Self: Sized;

    fn update(&mut self, command: OverlayCommand) -> Result<()>;

    fn close(&mut self) -> Result<()>;
}
