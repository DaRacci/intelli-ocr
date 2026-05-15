use anyhow::Result;

use crate::text::TextSource;
use crate::types::{CaptureFrame, TextFragment, TextSourceKind};

pub struct TesseractTextSource;

impl TextSource for TesseractTextSource {
    fn kind(&self) -> TextSourceKind {
        TextSourceKind::Tesseract
    }

    fn detect(&self, _frame: &CaptureFrame) -> Result<Vec<TextFragment>> {
        todo!("tesseract text detection not implemented")
    }
}
