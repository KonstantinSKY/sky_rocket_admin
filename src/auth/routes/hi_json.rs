use rocket::serde::json::{json, Value, Json};
// use rocket_okapi::openapi;

// #[openapi]
#[rocket::get("/hi")]
pub fn hi_json() -> Json<Value> {
    Json(json!({"key": "Hello, world! JSON!"}))
}

