use gtk::{prelude::*, CssProvider};
use gtk::{Box as GtkBox, Button, FlowBox, Image, ScrolledWindow};
use std::process::Command;

pub fn create_application_viewer(hbox: &GtkBox) -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);

    let flow_box = FlowBox::new();
    flow_box.set_valign(gtk::Align::Start);
    flow_box.set_max_children_per_line(10);
    flow_box.set_selection_mode(gtk::SelectionMode::None);

    let applications = get_active_applications();
    for (app_name, window_title) in applications {
        let button = create_application_button(&app_name, &window_title);
        flow_box.add(&button);
    }

    scrolled_window.add(&flow_box);
    hbox.pack_start(&scrolled_window, true, true, 0);

    // Apply CSS for styling (same as before)
    let css = CssProvider::new();
    css.load_from_data(
        b"
        flowbox {
            background-color: transparent;
        }
        button {
            background-color: #3B4252;
            color: #D8DEE9;
            border: none;
            border-radius: 5px;
        }
        button:hover {
            background-color: #4C566A;
        }
        ",
    )
    .expect("Failed to load CSS");

    let style_context = flow_box.style_context();
    style_context.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    scrolled_window
}

fn get_active_applications() -> Vec<(String, String)> {
    let output = Command::new("wmctrl")
        .arg("-l")
        .output()
        .expect("Failed to execute wmctrl");

    let output = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.splitn(4, ' ').collect();
        if parts.len() == 4 {
            let window_title = parts[3].to_string();
            let app_name = extract_app_name(&window_title);
            apps.push((app_name, window_title));
        }
    }

    apps
}

fn extract_app_name(window_title: &str) -> String {
    let lower_title = window_title.to_lowercase();

    let known_apps = [
        ("firefox", "Firefox"),
        ("chrome", "Chrome"),
        ("chromium", "Chromium"),
        ("visual studio code", "VS Code"),
        ("code", "VS Code"),
        ("discord", "Discord"),
        ("spotify", "Spotify"),
        ("terminal", "Terminal"),
        ("konsole", "Konsole"),
        ("dolphin", "Dolphin"),
        ("kate", "Kate"),
    ];

    for (keyword, app_name) in &known_apps {
        if lower_title.contains(keyword) {
            return app_name.to_string();
        }
    }

    // If we can't identify the app, use the first word of the title
    window_title
        .split_whitespace()
        .next()
        .unwrap_or(window_title)
        .to_string()
}

fn create_application_button(app_name: &str, window_title: &str) -> Button {
    let button = Button::new();
    let hbox = GtkBox::new(gtk::Orientation::Horizontal, 5);

    let image = create_default_image();
    hbox.pack_start(&image, false, false, 0);

    let label = gtk::Label::new(Some(app_name));
    hbox.pack_start(&label, false, false, 0);

    button.add(&hbox);
    button.set_tooltip_text(Some(window_title));
    button
}

fn create_default_image() -> Image {
    Image::from_icon_name(
        Some("application-x-executable"),
        gtk::IconSize::SmallToolbar,
    )
}
