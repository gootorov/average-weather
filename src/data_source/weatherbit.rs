use std::env;
use std::env::VarError;
use std::error::Error;
use serde::Deserialize;
use crate::data_source::DataSource;
use crate::weather_data::WeatherData;
use log;

pub struct WeatherBit {
    api_key: String
}

impl WeatherBit {
    pub fn from_envvar() -> Result<Self, VarError> {
        let api_key = env::var("WEATHERBIT_KEY")?;

        Ok(Self {
            api_key: api_key
        })
    }
}

impl DataSource for WeatherBit {
    fn forecast_n_days(
        &self,
        location: String,
        days: u32
    ) -> Result<Vec<WeatherData>, Box<dyn Error>> {
        let url = format!(
            "https://api.weatherbit.io/v2.0/forecast/daily?city={}&days={}&key={}",
            location,
            days,
            self.api_key
        );

        let raw_data = reqwest::blocking::get(&url)?
            .json::<WeatherBitResponse>()?
            .data;

        log::debug!("{:#?}", raw_data);

        Ok(raw_data.iter()
            .map(|day| WeatherData::new(day.temp))
            .collect::<Vec<_>>())
    }

    fn forecast_today(&self, location: String) -> Result<WeatherData, Box<dyn Error>> {
        // kinda ugly, but perhaphs the only way to move it out of the vector.
        Ok(self.forecast_n_days(location, 1)?.into_iter().nth(0).ok_or("No data")?)
    }

    fn forecast_tomorrow(&self, location: String) -> Result<WeatherData, Box<dyn Error>> {
        // to get the forecast for tomorrow, we request it for two days (today, tomorrow)
        // and skip the first day.
        Ok(self.forecast_n_days(location, 2)?.into_iter().nth(1).ok_or("No Data")?)
    }

    fn forecast_5_days(&self, location: String) -> Result<Vec<WeatherData>, Box<dyn Error>> {
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
