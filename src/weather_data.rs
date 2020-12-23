use std::chrono::naive::NaiveDate;
use serde::Deserialize;

/// Weather data that we return to the user.
/// Contains the data that it is forecasted for as well as the temperature.
#[derive(Serialize)]
struct WeatherData {
    date: NaiveDate,
    temperature: f64
}
