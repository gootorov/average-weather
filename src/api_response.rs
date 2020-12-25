use serde::Serialize;
use crate::weather_data::WeatherData;
use crate::api_error::ApiError;

/// Api Response.
/// Contains a response status, data, and possible errors.
struct ApiResponse {
    status: Status,
    data: Vec<WeatherData>,
    errors: Vec<ApiError>

}

/// Api Response Status.
#[derive(Serialize)]
enum Status {
    /// Success is returned if at least one data source returns a weather forecast.
    #[serde(rename = "success")]
    Success,
    /// Fail is returned if none of the data sources returned a weather forecast.
    #[serde(rename = "fail")]
    Fail
}
