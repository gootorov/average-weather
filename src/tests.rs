use std::env;
use super::INDEX;
use super::rocket;
use rocket::local::Client;
use rocket::http::Status;
use crate::data_source::weatherbit::WeatherBit;

#[test]
fn test_index() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(INDEX.into()));
}

#[test]
fn test_weatherbit_api_key_present() {
    let api_key = env::var("WEATHERBIT_KEY");
    assert!(api_key.is_ok());
}

#[test]
fn test_weatherbit_from_envvar() {
    let weatherbit = WeatherBit::from_envvar();
    assert!(weatherbit.is_ok());
}
