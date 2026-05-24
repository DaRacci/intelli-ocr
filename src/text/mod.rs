pub mod atspi;
pub mod merge;
pub mod tesseract;

use anyhow::Result;

pub use atspi::AtSpiTextSource;
pub use merge::NullTextMerger;
pub use tesseract::TesseractTextSource;

use crate::{capture::CaptureFrame, types::Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSourceKind {
    AtSpi,
    Tesseract,
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

pub trait TextSource: Send + Sync {
    fn kind(&self) -> TextSourceKind;
    fn detect(&self, frame: &CaptureFrame) -> Result<Vec<TextFragment>>;
}

pub trait TextMerger: Send + Sync {
    fn merge(
        &self,
        primary: Vec<TextFragment>,
        fallback: Vec<TextFragment>,
    ) -> Result<Vec<TextFragment>>;
}
