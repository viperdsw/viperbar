use config::clock_module_config::read_clock_module_config;

use chrono::Local;
use glib::{timeout_add_local, ControlFlow};
use gtk::prelude::*;
use gtk::{Box as GtkBox, CssProvider, Label};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config;

pub fn create_time_module(hbox: &GtkBox) -> Rc<RefCell<Label>> {
    let clock_config = read_clock_module_config();

    let time_label = Label::new(None);
    apply_time_module_css(&time_label);
    hbox.pack_end(&time_label, false, false, 5);

    let time_label = Rc::new(RefCell::new(time_label));

    let update_time = {
        let time_label = Rc::clone(&time_label);
        move || {
            let time = Local::now()
                .format(&clock_config.format.to_string())
                .to_string();
            time_label.borrow().set_text(&time);
            ControlFlow::Continue
        }
    };

    update_time();
    timeout_add_local(std::time::Duration::from_secs(1), update_time);

    time_label
}

fn apply_time_module_css(label: &Label) {
    let css = CssProvider::new();
    css.load_from_data(
        b"
        label {
            background-color: #4C566A;
            color: #ECEFF4;
            border-radius: 15px;
            padding: 5px;
            font-weight: bold;
        }
    ",
    )
    .expect("Failed to load CSS");

    let style_context = label.style_context();
    style_context.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
