#[macro_use]
extern crate rocket;
use rocket::{
    response::status,
    serde::json::{json, Value},
};

// This is a get endpoint that can be triggered with a get HTTP test to /
#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([{"id":1, "name": "John Doe"},{"id":2, "name": "Jane Dough"},{"id":3, "name": "Juan Doh"},{"id":4, "name": "Jan Do"},])
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({"id":id, "name": "John Doe", "email":"john@doe.com"})
}
#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({"id":5, "name": "Juanita Dow"})
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({"id":id, "name": "John Doe"})
}
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ],
    )
}
