#![feature(proc_macro_hygiene, decl_macro)]

mod api_error;
mod api_response;
mod constants;
mod data_source;
#[cfg(test)]
mod tests;
mod weather_data;

use api_error::ApiError;
use api_response::ApiResponse;
use data_source::{DataSource, MetaWeather, WeatherBit};
use itertools::{Either, Itertools};
use rocket::{get, routes, State};
use rocket::response::content::Html;
use weather_data::WeatherData;

type DataSources = [Box<dyn DataSource + Send + Sync>; 2];
/// Initializes available data sources.
/// In case you want to add another data source,
/// implement the DataSource trait for it and add it to this array.
fn get_data_sources() -> DataSources {
    [
        Box::new(WeatherBit::from_envvar()),
        Box::new(MetaWeather::new()),
    ]
}

/// Partition a sequence of responses into two parts:
/// a vector of successful responses and a vector of failures.
fn partition_data<T>(responses: T) -> (Vec<Vec<WeatherData>>, Vec<ApiError>)
where
    T: Iterator<Item = Result<Vec<WeatherData>, ApiError>>,
{
    let (data, errors) = responses
        .partition_map(|r| match r {
            Ok(data) => Either::Left(data),
            Err(e) => Either::Right(e)
        });

    (data, errors)
}

/// Computes the average of the data taken from multiple sources.
fn compute_average_data(data: Vec<Vec<WeatherData>>) -> Vec<WeatherData> {
    let n_sources = data.len();
    let mut average_data = data
        .into_iter()
        .fold1(|mut summed, next_source_data| {
            summed.iter_mut()
                .zip(next_source_data.into_iter())
                .for_each(|(current, next)| *current += next);

            summed
        })
        .unwrap_or_default();

    average_data.iter_mut().for_each(|day| *day /= n_sources as f64);
    average_data
}

fn response_handler<T>(responses: T) -> ApiResponse
where
    T: Iterator<Item = Result<Vec<WeatherData>, ApiError>>,
{
    let (data, errors) = partition_data(responses);

    let average_data = compute_average_data(data);

    ApiResponse::new(average_data, errors)
}

#[get("/forecast/today/<location>")]
fn forecast_today(location: String, sources: State<DataSources>) -> ApiResponse {
    let responses = sources.iter().map(|source| source.forecast_today(&location));
    response_handler(responses)
}

#[get("/forecast/tomorrow/<location>")]
fn forecast_tomorrow(location: String, sources: State<DataSources>) -> ApiResponse {
    let responses = sources.iter().map(|source| source.forecast_tomorrow(&location));
    response_handler(responses)
}

#[get("/forecast/five-days/<location>")]
fn forecast_5_days(location: String, sources: State<DataSources>) -> ApiResponse {
    let responses = sources.iter().map(|source| source.forecast_5_days(&location));
    response_handler(responses)
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(constants::INDEX)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(get_data_sources())
        .mount("/", routes![index, forecast_today, forecast_tomorrow, forecast_5_days])
}

fn main() {
    rocket().launch();
}
