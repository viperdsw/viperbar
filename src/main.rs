mod bar;
mod time_module;

use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let (window, hbox) = bar::create_bar();

    let _time_label: Rc<RefCell<gtk::Label>> = time_module::create_time_module(&hbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Stop
    });

    window.show_all();
    gtk::main();
}
