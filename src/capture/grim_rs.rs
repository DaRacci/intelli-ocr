use anyhow::Result;
use grim_rs::{CaptureResult, Grim};

use crate::capture::{CaptureBackend, CaptureFrame, CaptureRequest, OutputInfo};

pub struct GrimRsCaptureBackend {
    grim: Grim,
}

impl GrimRsCaptureBackend {
    pub fn new() -> Result<Self> {
        Ok(Self { grim: Grim::new()? })
    }
}

impl OutputInfo for grim_rs::Output {
    fn name(&self) -> &str {
        self.name()
    }

    fn width(&self) -> i32 {
        self.geometry().width()
    }

    fn height(&self) -> i32 {
        self.geometry().height()
    }
}

impl From<CaptureResult> for CaptureFrame {
    fn from(result: CaptureResult) -> Self {
        Self {
            width: result.width(),
            height: result.height(),
            pixels_rgba8: result.into_data(),
        }
    }
}

impl CaptureBackend for GrimRsCaptureBackend {
    fn outputs(&mut self) -> Result<Vec<Box<dyn super::OutputInfo>>> {
        let outputs = &self.grim.get_outputs()?;

        Ok(outputs
            .iter()
            .map(|output| Box::new(output.clone()) as Box<dyn OutputInfo>)
            .collect())
    }

    fn capture(&mut self, request: &CaptureRequest) -> Result<CaptureFrame> {
        let output = request
            .output_name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Output name is required for capture"))?;
        let capture_result = self.grim.capture_output(&output)?;

        Ok(capture_result.into())
    }
}
