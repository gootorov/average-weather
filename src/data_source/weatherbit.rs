use std::env;
use serde::Deserialize;
use crate::api_error::{ApiError, ErrorKind};
use crate::data_source::DataSource;
use crate::weather_data::WeatherData;

pub struct WeatherBit {
    api_key: String
}

impl WeatherBit {
    pub fn from_envvar() -> Self {
        let api_key = env::var("WEATHERBIT_KEY").unwrap_or_default();

        Self {
            api_key
        }
    }
}

impl DataSource for WeatherBit {
    fn forecast_n_days(
        &self,
        location: &str,
        days: u32
    ) -> Result<Vec<WeatherData>, ApiError> {
        let url = format!(
            "https://api.weatherbit.io/v2.0/forecast/daily?city={}&days={}&key={}",
            location,
            days,
            self.api_key
        );

        let response = match reqwest::blocking::get(&url) {
            // weatherbit returns status code 204 if the location is invalid.
            Ok(response) if response.status().as_u16() == 204 =>
                return Err(ApiError::new("WeatherBit", ErrorKind::InvalidLocation)),

            Ok(response) => response,
            Err(_) => return Err(ApiError::new("WeatherBit", ErrorKind::FailedConnection))
        };

        let raw_data = match response.json::<WeatherBitResponse>() {
            Ok(json) => json.data,
            Err(_) => return Err(ApiError::new("WeatherBit", ErrorKind::InvalidJSON))
        };

        log::debug!("{:#?}", raw_data);

        Ok(raw_data.iter()
            .map(|day| WeatherData::new(day.temp))
            .collect::<Vec<_>>())
    }

    fn forecast_today(&self, location: &str) -> Result<Vec<WeatherData>, ApiError> {
        Ok(self.forecast_n_days(location, 1)?)
    }

    fn forecast_tomorrow(&self, location: &str) -> Result<Vec<WeatherData>, ApiError> {
        // to get the forecast for tomorrow, we request it for two days (today, tomorrow)
        // and skip the first day.
        Ok(self.forecast_n_days(location, 2)?.drain(1..).collect())
    }

    fn forecast_5_days(&self, location: &str) -> Result<Vec<WeatherData>, ApiError> {
        Ok(self.forecast_n_days(location, 5)?)
    }
}

// Intermediate types that we use to map WeatherBit's responses to.
/// Represents API response from
/// [WeatherBit](https://www.weatherbit.io/api/weather-forecast-16-day)
/// We need only the data field.
#[derive(Debug, Deserialize)]
struct WeatherBitResponse {
    data: Vec<WeatherBitData>
}

/// In the data field of WeatherBit's response,
/// we need only the valid date (for debug purposes) and temperature.
#[derive(Debug, Deserialize)]
struct WeatherBitData {
    // keep the date for debug purposes.
    valid_date: String,
    temp: f64
}
