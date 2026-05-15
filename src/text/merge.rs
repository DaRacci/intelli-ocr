use anyhow::Result;

use crate::text::TextMerger;
use crate::types::TextFragment;

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
