type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// This trait describes a weather data source, such as [OpenWeatherMap](https://openweathermap.org)
/// and [Weatherstack](https://weatherstack.com).
///
/// Each method here should make a request to the data source,
/// handle it, and return in the correct format.
///
/// In case a new data source should be added, it must implement this trait.
trait DataSource {
    /// Forecast for the current day.
    fn forecast_today(&self) -> Result<WeatherData>;

    /// Forecast for tomorrow.
    fn forecast_tomorrow(&self) -> Result<WeatherData>;

    /// Forecast for the next 5 days.
    fn forecast_5_days(&self) -> Result<[WeatherData, 5]>;
}
