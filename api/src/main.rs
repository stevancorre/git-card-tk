mod color;
mod gen;

#[macro_use]
extern crate rocket;

use color::is_valid_color;
use gen::{options::GenOptions, png::gen_png};
use rocket::{
    fs::{relative, NamedFile},
    get,
    http::Status,
};
use std::path::Path;

#[derive(Responder)]
#[response(status = 200, content_type = "image/png")]
struct IncludedHtml(&'static [u8]);

#[get("/png?<title>&<subtitle>&<color>")]
async fn png(
    title: String,
    subtitle: Option<String>,
    color: String,
) -> Result<Option<NamedFile>, Status> {
    if !is_valid_color(&*color) {
        // TODO: return error as image
        return Err(Status::BadRequest);
    }

    let options = GenOptions::new(title, subtitle, color);
    let file_name = format!("{}.png", options.hash);

    let path = Path::new(relative!("gen")).join(file_name);
    if !path.exists() {
        gen_png(&options);
    }

    Ok(NamedFile::open(path).await.ok())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![png])
}
