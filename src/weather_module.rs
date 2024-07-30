use config::weather_module_config::read_weather_config;

use glib::{timeout_add_local, ControlFlow};
use gtk::prelude::*;
use gtk::{Box as GtkBox, CssProvider, Label};
use std::cell::RefCell;
use std::rc::Rc;

extern crate openweathermap;

use openweathermap::{init, update};

use crate::config;

pub fn create_weather_module(hbox: &GtkBox) -> Rc<RefCell<Label>> {
    let weather_config = read_weather_config();

    let weather_label = Label::new(None);
    apply_weather_module_css(&weather_label);
    hbox.pack_end(&weather_label, false, false, 5);

    let weather_label = Rc::new(RefCell::new(weather_label));

    let update_weather = {
        let weather_label = Rc::clone(&weather_label);
        move || {
            let weather = get_weather(
                &weather_config.api_key,
                &weather_config.city,
                &weather_config.country_code,
            );
            weather_label.borrow().set_text(&weather);
            ControlFlow::Continue
        }
    };

    update_weather();
    timeout_add_local(std::time::Duration::from_secs(60), update_weather);

    weather_label
}

fn apply_weather_module_css(label: &Label) {
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

fn get_weather(api_key: &str, city: &str, country: &str) -> String {
    let city_country = format!("{},{}", city, country);

    let receiver = &init(&city_country, "metric", "en", &api_key, 10);

    let mut weather: String = String::new();

    loop {
        match update(receiver) {
            Some(response) => match response {
                Ok(current) => {
                    weather = format!(
                        "{}°C, {}",
                        current.main.temp.round(),
                        current.weather[0].description
                    );
                    println!("{}", weather);
                    break; // Exit the loop once we get a valid response
                }
                Err(e) => {
                    println!("Could not fetch weather because: {}", e);
                    // Optionally handle retry logic or break the loop after a certain number of retries
                }
            },
            None => {
                // Optionally handle the case where update returns None
            }
        }
    }
    weather
}
