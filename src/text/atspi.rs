use anyhow::Result;

use crate::{
    capture::CaptureFrame,
    text::{TextFragment, TextSource, TextSourceKind},
};

pub struct AtSpiTextSource;

impl TextSource for AtSpiTextSource {
    fn kind(&self) -> TextSourceKind {
        TextSourceKind::AtSpi
    }

    fn detect(&self, _frame: &CaptureFrame) -> Result<Vec<TextFragment>> {
        todo!("at-spi text detection not implemented")
    }
}
