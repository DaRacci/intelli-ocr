use anyhow::Result;

use crate::{
    text::TextFragment,
    types::{Point, Rect},
};

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

pub trait SelectionStrategy: Send + Sync {
    fn select(&self, fragments: &[TextFragment], input: &SelectionInput)
    -> Result<SelectionResult>;
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
