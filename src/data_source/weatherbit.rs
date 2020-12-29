use crate::api_error::{ApiError, ErrorKind};
use crate::data_source::DataSource;
use crate::weather_data::WeatherData;
use serde::Deserialize;
use std::env;

/// [WeatherBit](https://www.weatherbit.io/api) API
pub struct WeatherBit {
    api_key: String,
}

impl WeatherBit {
    pub fn from_envvar() -> Self {
        let api_key = env::var("WEATHERBIT_KEY").unwrap_or_default();

        Self { api_key }
    }
}

impl DataSource for WeatherBit {
    fn forecast_n_days(
        &self,
        location: &str,
        days: u32,
        skip_days: u32
    ) -> Result<Vec<WeatherData>, ApiError> {
        let url = format!(
            "https://api.weatherbit.io/v2.0/forecast/daily?city={}&days={}&key={}",
            location,
            days,
            self.api_key
        );

        let response = match reqwest::blocking::get(&url) {
            // weatherbit returns status code 204 if the location is invalid.
            Ok(response) if response.status().as_u16() == 204 => {
                return Err(ApiError::new("WeatherBit", ErrorKind::InvalidLocation))
            },
            Ok(response) if response.status().as_u16() == 403 => {
                return Err(ApiError::new("WeatherBit", ErrorKind::InvalidApiKey))
            },
            Ok(response) => response,
            Err(_) => return Err(ApiError::new("WeatherBit", ErrorKind::FailedConnection)),
        };

        let raw_data = match response.json::<WeatherBitResponse>() {
            Ok(json) => json.data,
            Err(_) => return Err(ApiError::new("WeatherBit", ErrorKind::InvalidJSON)),
        };

        Ok(raw_data
            .iter()
            .skip(skip_days as usize)
            .map(|day| WeatherData::new(day.temp))
            .collect::<Vec<_>>())
    }
}

// Intermediate types that we use to map WeatherBit's responses to.
/// Represents API response from
/// [WeatherBit](https://www.weatherbit.io/api/weather-forecast-16-day)
#[derive(Debug, Deserialize)]
struct WeatherBitResponse {
    data: Vec<WeatherBitData>,
}

#[derive(Debug, Deserialize)]
struct WeatherBitData {
    temp: f64,
}
