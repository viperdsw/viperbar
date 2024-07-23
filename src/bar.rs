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

    apply_bar_css(&window);

    (window, hbox)
}

fn apply_bar_css(window: &Window) {
    let css = gtk::CssProvider::new();
    css.load_from_data(
        b"
        window {
            background-color: transparent;
        }
    ",
    )
    .expect("Failed to load CSS");

    let style_context = window.style_context();
    style_context.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
