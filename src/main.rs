#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_static_resources;

use std::env;
use rocket_include_static_resources::StaticResponse;

#[get("/favicon.ico")]
fn favicon() -> StaticResponse {
    static_response!("favicon")
}

#[get("/favicon-16.png")]
fn favicon_16() -> StaticResponse {
    static_response!("favicon-png")
}

#[get("/favicon-32.png")]
fn favicon_32() -> StaticResponse {
    static_response!("favicon-png")
}

#[get("/version")]
fn version() -> String {
    let v = env::var("CARGO_PKG_VERSION");
    format!("{}", v.unwrap())
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "favicon", "static/favicon.ico",
                "favicon-16", "static/favicon-16.png",
                "favicon-32", "static/favicon-32.png",
            );
        }))
        .mount("/", routes![favicon,favicon_16,favicon_32])
        .mount("/hello", routes![hello,version])
        .launch();
}
