use chrono::Local;
use glib::{timeout_add_local, ControlFlow};
use gtk::prelude::*;
use gtk::{Box as GtkBox, Label};
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_time_module(hbox: &GtkBox) -> Rc<RefCell<Label>> {
    let time_label = Label::new(None);
    hbox.pack_end(&time_label, false, false, 5);

    let time_label = Rc::new(RefCell::new(time_label));

    let update_time = {
        let time_label = Rc::clone(&time_label);
        move || {
            let time = Local::now().format("%H:%M:%S").to_string();
            time_label.borrow().set_text(&time);
            ControlFlow::Continue
        }
    };

    update_time();
    timeout_add_local(std::time::Duration::from_secs(1), update_time);

    time_label
}
