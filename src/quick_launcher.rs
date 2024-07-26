use std::process::Command;

use gtk::{prelude::*, CssProvider};
use gtk::{Box as GtkBox, Button};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::container::container;

#[derive(Debug, Serialize, Deserialize)]
struct QuickLaunchStruct {
    icon: String,
    tooltip: String,
    exec: String,
}

fn read_config() -> Vec<QuickLaunchStruct> {
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

pub fn create_quick_launchers(hbox: &GtkBox) -> Vec<Button> {
    let config_items = read_config();
    println!("{:#?}", config_items);

    let mut buttons = Vec::new();

    let css = CssProvider::new();
    css.load_from_data(
        b"
        button {
            color: #d8dee9;
            padding: 0px;
            background-color: #4C566A;
            margin-left: 10px;
        }
        ",
    )
    .expect("Failed to load CSS");

    for launch in config_items {
        let button = Button::with_label(&launch.icon);
        button.set_tooltip_text(Some(&launch.tooltip));

        let button_style_context = button.style_context();
        button_style_context.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        // add on click
        let exec_cmd = launch.exec.to_string();
        button.connect_clicked(move |_| {
            if let Err(e) = Command::new(&exec_cmd).spawn() {
                eprintln!("failed to execute: {}: {}", exec_cmd, e);
            }
        });

        buttons.push(button);
    }

    let container = container(hbox, true);
    for button in &buttons {
        container.add(button);
    }
    hbox.pack_start(&container, false, false, 5);

    buttons
}
