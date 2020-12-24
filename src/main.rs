#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;

#[cfg(test)] mod tests;
mod data_source;
mod weather_data;

const INDEX: &'static str = "Hello, World\n";

#[get("/")]
fn index() -> &'static str {
    INDEX
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    env_logger::init();
    rocket().launch();
}
