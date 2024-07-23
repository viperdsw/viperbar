use smithay_client_toolkit::reexports::client::protocol::wl_keyboard::KeyState;
use smithay_client_toolkit::seat::keyboard::{KeyEvent, KeyboardHandler, Modifiers};
use smithay_client_toolkit::seat::Capability;
use std::process;
use xkbcommon::xkb;

pub struct KeybindHandler {
    pub should_restart: bool,
}

impl KeybindHandler {
    pub fn new() -> Self {
        KeybindHandler {
            should_restart: false,
        }
    }
}

impl KeyboardHandler for KeybindHandler {
    fn enter(&mut self, _: &smithay_client_toolkit::seat::keyboard::KeyboardFocus, _: &Modifiers) {}
    fn leave(&mut self) {}
    fn modifiers_changed(&mut self, _: &Modifiers) {}

    fn key(&mut self, _: &smithay_client_toolkit::seat::keyboard::KeyboardFocus, event: KeyEvent) {
        if event.state == KeyState::Pressed {
            match event.keysym {
                xkb::KEY_q if event.modifiers.ctrl && event.modifiers.alt => {
                    println!("Quitting application");
                    process::exit(0);
                }
                xkb::KEY_r if event.modifiers.ctrl && event.modifiers.alt => {
                    println!("Restarting application");
                    self.should_restart = true;
                }
                _ => {}
            }
        }
    }
}
