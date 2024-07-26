use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickLaunchStruct {
    pub icon: String,
    pub tooltip: String,
    pub exec: String,
}

pub fn read_quick_launch_config() -> Vec<QuickLaunchStruct> {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("viperbar")
        .join("viperbar.json");

    if config_path.exists() {
        let mut file = File::open(config_path).expect("Failed to open config file...");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file...");

        let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON...");

        if let Some(quick_launch) = json.get("quickLaunch") {
            if let Some(array) = quick_launch.as_array() {
                return array
                    .iter()
                    .filter_map(|item| serde_json::from_value(item.clone()).ok())
                    .collect();
            }
        }
    }
    vec![
        QuickLaunchStruct {
            icon: " ".to_string(),
            tooltip: "Discord".to_string(),
            exec: "discord".to_string(),
        },
        QuickLaunchStruct {
            icon: "".to_string(),
            tooltip: "Terminal".to_string(),
            exec: "kitty".to_string(), // allow user to change this one day
        },
        QuickLaunchStruct {
            icon: "".to_string(),
            tooltip: "Spotify".to_string(),
            exec: "spotify".to_string(),
        },
        QuickLaunchStruct {
            icon: "󰈹".to_string(),
            tooltip: "Firefox".to_string(),
            exec: "firefox".to_string(),
        },
        QuickLaunchStruct {
            icon: "󰨞".to_string(),
            tooltip: "VSCode".to_string(),
            exec: "code".to_string(),
        },
    ]
}
