use anyhow::Result;

use crate::{
    capture::CaptureFrame,
    text::{TextFragment, TextSource, TextSourceKind},
};

pub struct TesseractTextSource;

impl TextSource for TesseractTextSource {
    fn kind(&self) -> TextSourceKind {
        TextSourceKind::Tesseract
    }

    fn detect(&self, _frame: &CaptureFrame) -> Result<Vec<TextFragment>> {
        todo!("tesseract text detection not implemented")
    }
}
