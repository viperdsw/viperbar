use config::quick_launch_config::read_quick_launch_config;

use gtk::{prelude::*, CssProvider};
use gtk::{Box as GtkBox, Button};
use std::process::Command;

use crate::config;
use crate::container::container;

pub fn create_quick_launchers(hbox: &GtkBox) -> Vec<Button> {
    let config_items = read_quick_launch_config();

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
