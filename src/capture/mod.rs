use anyhow::Result;

use crate::types::{CaptureFrame, CaptureRequest};

pub trait CaptureBackend: Send + Sync {
    fn capture(&self, request: &CaptureRequest) -> Result<CaptureFrame>;
}

pub struct NullCaptureBackend;

impl CaptureBackend for NullCaptureBackend {
    fn capture(&self, _request: &CaptureRequest) -> Result<CaptureFrame> {
        todo!("screen capture backend not implemented")
    }
}
