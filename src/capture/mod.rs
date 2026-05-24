use anyhow::Result;
use serde::{Deserialize, Serialize};
pub mod grim_rs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    pub output_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CaptureFrame {
    pub width: u32,
    pub height: u32,
    pub pixels_rgba8: Vec<u8>,
}

pub trait OutputInfo: Send + Sync {
    fn name(&self) -> &str;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub trait CaptureBackend: Send + Sync {
    fn outputs(&mut self) -> Result<Vec<Box<dyn OutputInfo>>>;

    fn capture(&mut self, request: &CaptureRequest) -> Result<CaptureFrame>;
}
