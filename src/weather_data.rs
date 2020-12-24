use serde::Serialize;

/// Weather data that we return to the user.
/// Contains the data that it is forecasted for as well as the temperature.
#[derive(Debug, Serialize)]
pub struct WeatherData {
    temperature: f64
}

impl WeatherData {
    pub fn new(temperature: f64) -> Self {
        Self {
            temperature: temperature
        }
    }
}
