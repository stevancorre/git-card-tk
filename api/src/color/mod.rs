use self::{hex::is_valid_color_hex, names::is_valid_color_name};

pub mod hex;
pub mod names;

pub fn is_valid_color(color: &str) -> bool {
    is_valid_color_name(color) || is_valid_color_hex(color)
}
