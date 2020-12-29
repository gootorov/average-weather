use super::rocket;
use crate::api_error::{ApiError, ErrorKind};
use crate::{compute_average_data, partition_data};
use crate::data_source::{DataSource, MetaWeather, WeatherBit};
use crate::constants::INDEX;
use crate::weather_data::WeatherData;
use rocket::http;
use rocket::local::Client;

#[test]
fn test_weatherbit_response_len() {
    let wb = WeatherBit::from_envvar();

    let location = "Moscow";
    // really not sure if making network requests in unit-testing is ok.
    let today = wb.forecast_today(&location).unwrap();
    let tomorrow = wb.forecast_tomorrow(&location).unwrap();
    let five_days = wb.forecast_5_days(&location).unwrap();

    assert_eq!(today.len(), 1);
    assert_eq!(tomorrow.len(), 1);
    assert_eq!(five_days.len(), 5);
}

#[test]
fn test_metaweather_response_len() {
    let mw = MetaWeather::new();

    let location = "Moscow";
    // again, making network requests in unit tests is probably bad,
    // but making a mock suite will be overkill.
    let today = mw.forecast_today(&location).unwrap();
    let tomorrow = mw.forecast_tomorrow(&location).unwrap();
    let five_days = mw.forecast_5_days(&location).unwrap();

    assert_eq!(today.len(), 1);
    assert_eq!(tomorrow.len(), 1);
    assert_eq!(five_days.len(), 5);
}

#[test]
fn test_partition_data() {
    let error = ApiError::new("test", ErrorKind::InvalidLocation);
    let error_cloned = error.clone();

    let response_data_source1 = Ok(vec![WeatherData::new(1.), WeatherData::new(2.)]);
    let response_data_source2 = Err(error);

    let responses = vec![response_data_source1, response_data_source2];

    let expected_data = vec![vec![WeatherData::new(1.), WeatherData::new(2.)]];
    let expected_errors = vec![error_cloned];

    assert_eq!(partition_data(responses.into_iter()), (expected_data, expected_errors));
}

#[test]
fn test_average_data() {
    let data1 = vec![1., 2., 3.].into_iter().map(WeatherData::new).collect();
    let data2 = vec![5., 4., 9.].into_iter().map(WeatherData::new).collect();
    let data = vec![data1, data2];

    let expected_average = vec![(1. + 5.) / 2., (2. + 4.) / 2., (3. + 9.) / 2.]
        .into_iter().map(WeatherData::new).collect::<Vec<_>>();

    assert_eq!(compute_average_data(data), expected_average);
}

#[test]
fn test_average_data_empty() {
    let data = vec![];

    assert_eq!(compute_average_data(data), vec![]);
}

#[test]
fn test_index() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), http::Status::Ok);
    assert_eq!(response.body_string(), Some(INDEX.into()));
}
