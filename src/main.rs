#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

const WELCOME_PAGE: &'static str = "Hello, World\n";

#[get("/")]
fn index() -> &'static str {
    WELCOME_PAGE
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
