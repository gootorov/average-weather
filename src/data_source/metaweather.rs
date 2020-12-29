use crate::api_error::{ApiError, ErrorKind};
use crate::data_source::DataSource;
use crate::weather_data::WeatherData;
use serde::Deserialize;

/// [MetaWeather](https://www.metaweather.com/) API
pub struct MetaWeather;

impl MetaWeather {
    pub fn new() -> Self {
        Self {}
    }
}

impl DataSource for MetaWeather {
    fn forecast_n_days(
        &self,
        location: &str,
        days: u32,
        skip_days: u32
    ) -> Result<Vec<WeatherData>, ApiError> {
        let source = "MetaWeather";

        let location_url = format!(
            "https://www.metaweather.com/api/location/search/?query={}",
            location
        );

        let location_response = match reqwest::blocking::get(&location_url) {
            Ok(response) => response,
            Err(_) => return Err(ApiError::new(source, ErrorKind::FailedConnection)),
        };

        let location_woeid = match location_response.json::<Vec<MetaWeatherLocation>>() {
            // metaweather returns empty data with code 200 for invalid location.
            Ok(json) if json.len() == 0 => {
                return Err(ApiError::new(source, ErrorKind::InvalidLocation));
            },
            // metwather returns top-level array with one element.
            Ok(json) => json[0].woeid.to_string(),
            Err(_) => return Err(ApiError::new(source, ErrorKind::InvalidJSON)),
        };

        let url = format!("https://www.metaweather.com/api/location/{}/", location_woeid);

        let response = match reqwest::blocking::get(&url) {
            Ok(response) => response,
            Err(_) => return Err(ApiError::new(source, ErrorKind::FailedConnection)),
        };

        let raw_data = match response.json::<MetaWeatherResponse>() {
            Ok(json) => json.consolidated_weather,
            Err(_) => return Err(ApiError::new(source, ErrorKind::InvalidJSON)),
        };

        Ok(raw_data
            .iter()
            .take(days as usize)
            .skip(skip_days as usize)
            .map(|day| WeatherData::new(day.the_temp))
            .collect::<Vec<_>>())
    }
}

// Intermediate types that we use to map WeatherBit's responses to.
/// MetaWeather response for location lookup.
#[derive(Debug, Deserialize)]
struct MetaWeatherLocation {
    /// Where on Earth ID.
    woeid: i64,
}

/// Represents API response from
/// [MetaWeather](https://www.metaweather.com/)
#[derive(Debug, Deserialize)]
struct MetaWeatherResponse {
    consolidated_weather: Vec<MetaWeatherData>,
}

#[derive(Debug, Deserialize)]
struct MetaWeatherData {
    the_temp: f64,
}
