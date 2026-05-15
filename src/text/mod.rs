pub mod atspi;
pub mod merge;
pub mod tesseract;

use anyhow::Result;

pub use atspi::AtSpiTextSource;
pub use merge::NullTextMerger;
pub use tesseract::TesseractTextSource;

use crate::types::{CaptureFrame, TextFragment, TextSourceKind};

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
