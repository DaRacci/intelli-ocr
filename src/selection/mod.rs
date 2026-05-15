use anyhow::Result;

use crate::types::{SelectionInput, SelectionResult, TextFragment};

pub trait SelectionStrategy: Send + Sync {
    fn select(
        &self,
        fragments: &[TextFragment],
        input: &SelectionInput,
    ) -> Result<SelectionResult>;
}

pub struct DragSelectionStrategy;

impl SelectionStrategy for DragSelectionStrategy {
    fn select(
        &self,
        _fragments: &[TextFragment],
        _input: &SelectionInput,
    ) -> Result<SelectionResult> {
        todo!("drag selection strategy not implemented")
    }
}
