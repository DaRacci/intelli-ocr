#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Idle,
    Capturing,
    Detecting,
    ShowingOverlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSourceKind {
    AtSpi,
    Tesseract,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    pub output_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CaptureFrame {
    pub width: u32,
    pub height: u32,
    pub pixels_rgba8: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TextFragment {
    pub id: u64,
    pub text: String,
    pub bounds: Rect,
    pub source: TextSourceKind,
    pub confidence: Option<f32>,
    pub block_index: Option<u32>,
    pub line_index: Option<u32>,
    pub fragment_index: Option<u32>,
}

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

#[derive(Debug, Clone)]
pub struct SelectionInput {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
pub struct SelectionResult {
    pub fragment_ids: Vec<u64>,
    pub text: String,
    pub bounds: Vec<Rect>,
}
