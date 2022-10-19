mod color;
mod gen;

#[macro_use]
extern crate rocket;

use color::is_valid_color;
use gen::{options::GenOptions};
use rocket::{
    get,
    http::{ContentType, Status},
    serde::json::Json,
};

#[get("/svg?<title>&<color>")]
fn svg(title: String, color: String) -> Result<(ContentType, String), Status> {
    if !is_valid_color(&*color) {
        // TODO: return error as image
        return Err(Status::BadRequest);
    }

    let options = GenOptions::new(title, color);
}

#[get("/")]
fn root() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Ok")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, svg])
}
