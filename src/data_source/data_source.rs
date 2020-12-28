use crate::api_error::ApiError;
use crate::weather_data::WeatherData;

type Result<T> = std::result::Result<T, ApiError>;

/// This trait describes a weather data source, such as [OpenWeatherMap](https://openweathermap.org)
/// and [WeatherBit](https://www.weatherbit.io/api).
pub trait DataSource {
    /// Forecast for the next n days.
    fn forecast_n_days(&self, location: &str, days: u32, skip_days: u32) -> Result<Vec<WeatherData>>;

    /// Forecast for the current day.
    fn forecast_today(&self, location: &str) -> Result<Vec<WeatherData>> {
        Ok(self.forecast_n_days(location, 1, 0)?)
    }

    /// Forecast for the next day.
    fn forecast_tomorrow(&self, location: &str) -> Result<Vec<WeatherData>> {
        // to get the forecast for tomorrow, we request it for two days (today, tomorrow)
        // and skip the first day.
        Ok(self.forecast_n_days(location, 2, 1)?)
    }

    /// Forecast for the next five days.
    fn forecast_5_days(&self, location: &str) -> Result<Vec<WeatherData>> {
        Ok(self.forecast_n_days(location, 5, 0)?)
    }
}
