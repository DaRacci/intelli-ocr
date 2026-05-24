use anyhow::Result;

use crate::text::{TextFragment, TextMerger};

pub struct NullTextMerger;

impl TextMerger for NullTextMerger {
    fn merge(
        &self,
        _primary: Vec<TextFragment>,
        _fallback: Vec<TextFragment>,
    ) -> Result<Vec<TextFragment>> {
        todo!("text merge strategy not implemented")
    }
}
