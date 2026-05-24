use std::process::Command;

use crate::compositor_backend::CompositorBackend;

pub struct HyprlandCompositorBackend;

fn select_active_from_json(json_string: &str) -> anyhow::Result<String> {
    let monitors = match serde_json::from_str::<serde_json::Value>(json_string) {
        Ok(value) => value
            .as_array()
            .expect("Unexpceted JSON format, expected an array")
            .clone(),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to parse JSON from hyprctl output: {}",
                e
            ));
        }
    };

    Ok(monitors
        .iter()
        .filter(|mon| {
            mon.get("focused")
                .and_then(|f| f.as_bool())
                .unwrap_or(false)
        })
        .filter_map(|mon| mon.get("name").and_then(|n| n.as_str()))
        .next()
        .unwrap_or("")
        .to_string())
}

impl CompositorBackend for HyprlandCompositorBackend {
    fn get_active_output(&self) -> anyhow::Result<String> {
        let output = Command::new("hyprctl").arg("monitors").arg("-j").output();
        let json_string = match output {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(e) => return Err(anyhow::anyhow!("Failed to execute hyprctl: {}", e)),
        };

        select_active_from_json(&json_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_lib::MONITORS_JSON_STR;

    #[test]
    fn test_get_active_output() {
        let monitors_capture = &MONITORS_JSON_STR;
        let active_output = super::select_active_from_json(monitors_capture).unwrap();

        assert_eq!(active_output, "DP-6");
    }
}
