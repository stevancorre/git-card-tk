mod color;

#[macro_use]
extern crate rocket;

use color::is_valid_color;
use rocket::{get, http::Status, serde::json::Json};

#[get("/svg?<title>&<color>")]
fn svg(title: &str, color: &str) -> Result<Json<String>, Status> {
    if !is_valid_color(color) {
        // TODO: return error as image
        return Err(Status::BadRequest);
    }

    Ok(Json(color.to_string()))
}

#[get("/")]
fn root() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Ok")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, svg])
}
