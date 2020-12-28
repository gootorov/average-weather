use serde::Serialize;

/// Weather data that we return to the user.
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

impl std::ops::AddAssign for WeatherData {
    fn add_assign(&mut self, other: Self) {
        self.temperature += other.temperature;
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

impl std::ops::DivAssign<f64> for WeatherData {
    fn div_assign(&mut self, other: f64) {
        self.temperature /= other;
    }
}
