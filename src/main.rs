#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

// This is a get endpoint that can be triggered with a get HTTP test to /
#[get("/")]
fn index() -> Value {
    json!("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
