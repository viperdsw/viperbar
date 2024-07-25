use gtk::{prelude::*, CssProvider};
use gtk::{Box as GtkBox, Button};

pub fn create_close_button(hbox: &GtkBox) -> Button {
    let close_button = Button::with_label("");
    hbox.pack_end(&close_button, false, false, 5);

    // Apply CSS to remove padding
    let css = CssProvider::new();
    css.load_from_data(
        b"button {
                color: #ed4337;
                padding: 0px;
                background-color: #4C566A;
            }
        ",
    )
    .expect("Failed to load CSS");

    let style_context = close_button.style_context();
    style_context.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    close_button.connect_clicked(|_| {
        gtk::main_quit();
    });
    close_button
}
