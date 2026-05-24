pub mod hyprland;
// pub mod wayland;

use anyhow::Result;

use crate::error::IntelliOcrError;

pub trait CompositorBackend {
    fn get_active_output(&self) -> Result<String>;
}

pub fn get_compositor_backend() -> Result<Box<dyn CompositorBackend>, IntelliOcrError> {
    let desktop_type_str =
        std::env::var("XDG_CURRENT_DESKTOP").or(Err(IntelliOcrError::UnsupportedEnvironment(
            "XDG_CURRENT_DESKTOP environment variable is not set, unable to determine desktop environment",
        )))?;

    match desktop_type_str.to_lowercase().as_str() {
        "hyprland" => Ok(Box::new(hyprland::HyprlandCompositorBackend)),
        _ => Err(IntelliOcrError::UnsupportedEnvironment(
            "Unsupported desktop environment",
        )),
    }
}
