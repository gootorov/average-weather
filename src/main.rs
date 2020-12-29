#![feature(proc_macro_hygiene, decl_macro)]

mod api_error;
mod api_response;
mod data_source;
mod constants;
#[cfg(test)]
mod tests;
mod weather_data;

use api_error::ApiError;
use api_response::{ApiResponse, Status};
use data_source::{DataSource, MetaWeather, WeatherBit};
use itertools::{Either, Itertools};
use rocket::{get, routes, State};
use rocket::response::content::Html;
use rocket_contrib::json::Json;
use weather_data::WeatherData;

type DataSources = [Box<dyn DataSource + Send + Sync>; 2];
fn get_data_sources() -> DataSources {
    [Box::new(WeatherBit::from_envvar()),
     Box::new(MetaWeather::new())]
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
    // sum up data from multiple sources.
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

fn response_handler<T>(responses: T) -> Json<ApiResponse>
where
    T: Iterator<Item = Result<Vec<WeatherData>, ApiError>>,
{
    let (data, errors) = partition_data(responses);

    let average_data = compute_average_data(data);

    // not sure if status should just be bool,
    // it's unlikely that Status::Error will be useful.
    let status = match average_data.len() == 0 {
        true => Status::Fail,
        false => Status::Success,
    };

    Json(ApiResponse::new(status, average_data, errors))
}

#[get("/forecast/today/<location>")]
fn forecast_today(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_today(&location));
    response_handler(responses)
}

#[get("/forecast/tomorrow/<location>")]
fn forecast_tomorrow(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_tomorrow(&location));
    response_handler(responses)
}

#[get("/forecast/five-days/<location>")]
fn forecast_5_days(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
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
    env_logger::init();
    rocket().launch();
}
