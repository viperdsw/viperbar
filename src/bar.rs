// create_bar
use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Window, WindowType};
use gtk_layer_shell::{Edge, Layer, LayerShell};

pub fn create_bar() -> (Window, GtkBox) {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Wayland Bar");

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Top, true);

    window.set_size_request(800, 30);
    window.set_margin_start(0);
    window.set_margin_end(0);
    window.set_margin_top(0);
    window.set_exclusive_zone(30);

    let hbox = GtkBox::new(Orientation::Horizontal, 5);
    window.add(&hbox);

    (window, hbox)
}

pub fn apply_css() {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(
            b"
        window {
            background-color: #2e3440;
        }
        label {
            color: white;
            font-size: 14px;
        }
    ",
        )
        .expect("Failed to load CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
