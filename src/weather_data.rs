use serde::Serialize;

/// Weather data that we return to the user.
/// Contains the data that it is forecasted for as well as the temperature.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeatherData {
    temperature: f64,
}

impl WeatherData {
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }
}

impl std::ops::Add for WeatherData {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            temperature: self.temperature + other.temperature,
        }
    }
}

impl std::ops::Div<f64> for WeatherData {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            temperature: self.temperature / other,
        }
    }
}
