use gtk::Box as GtkBox;
use gtk::{prelude::*, CssProvider, Orientation};

pub fn container(hbox: &GtkBox, is_start: bool) -> GtkBox {
    let container = GtkBox::new(Orientation::Horizontal, 0);

    let css = CssProvider::new();
    css.load_from_data(
        b"
    .container {
        margin-left: 50px;
     }
    ",
    )
    .expect("Failed to load css");

    let style_context = container.style_context();
    style_context.add_class("container");
    style_context.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    if is_start {
        hbox.pack_start(&container, false, false, 5);
    } else {
        hbox.pack_end(&container, false, false, 5);
    }

    container
}
