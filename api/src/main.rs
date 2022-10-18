#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn root() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Ok")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root])
}
