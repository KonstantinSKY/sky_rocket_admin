// use rocket::serde::json::{json, Value};
// use rocket_okapi::openapi;


/// First api Hello world for testi

// #[openapi]
#[rocket::get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

// #[openapi]
// #[get("/doc/hello")]
// pub fn hello_openapi() -> &'static str {
//     "Hello, world!"
// }