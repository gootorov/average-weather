use std::env;
use std::env::VarError;

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
