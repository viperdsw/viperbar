use std::process::Command;

use gtk::{prelude::*, CssProvider};
use gtk::{Box as GtkBox, Button};

use crate::container::container;

struct QuickLaunchStruct {
    icon: &'static str,
    tooltip: &'static str,
    exec: &'static str,
}

pub fn create_quick_launchers(hbox: &GtkBox) -> Vec<Button> {
    let launches = vec![
        QuickLaunchStruct {
            icon: "",
            tooltip: "Discord",
            exec: "discord",
        },
        QuickLaunchStruct {
            icon: "",
            tooltip: "Terminal",
            exec: "kitty", // allow user to change this one day
        },
        QuickLaunchStruct {
            icon: "",
            tooltip: "Spotify",
            exec: "spotify",
        },
        QuickLaunchStruct {
            icon: "󰈹",
            tooltip: "Firefox",
            exec: "firefox",
        },
        QuickLaunchStruct {
            icon: "󰨞",
            tooltip: "VSCode",
            exec: "code",
        },
    ];

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

    for launch in launches {
        let button = Button::with_label(launch.icon);
        button.set_tooltip_text(Some(launch.tooltip));

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
