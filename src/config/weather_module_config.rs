use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherModuleConfig {
    pub api_key: String,
    pub city: String,
    pub country_code: String,
}

pub fn read_weather_config() -> WeatherModuleConfig {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("viperbar")
        .join("viperbar.json");

    if config_path.exists() {
        let mut file = File::open(config_path).expect("Failed to open config file...");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file...");

        let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON...");

        if let Some(clock_module) = json.get("weather") {
            return serde_json::from_value(clock_module.clone()).expect("Failed to parse JSON...");
        }
    }

    // default this
    WeatherModuleConfig {
        api_key: "YOUR_API_KEY".to_string(),
        city: "YOUR_CITY".to_string(),
        country_code: "YOUR_COUNTRY_CODE".to_string(),
    }
}
