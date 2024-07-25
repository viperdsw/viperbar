mod bar;
mod close_button_module;
mod time_module;

use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // entry point to create the main bar. after this we add the modules
    let (window, hbox) = bar::create_bar();

    let _close_button: gtk::Button = close_button_module::create_close_button(&hbox);
    let _time_label: Rc<RefCell<gtk::Label>> = time_module::create_time_module(&hbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Stop
    });

    window.show_all();
    gtk::main();
}
