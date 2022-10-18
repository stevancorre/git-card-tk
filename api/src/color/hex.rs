use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^[0-9A-Fa-f]{1}$|^[0-9A-Fa-f]{3}$|^[0-9A-Fa-f]{6}$").unwrap();
}

pub fn is_valid_color_hex(color: &str) -> bool {
    RE.is_match(color)
}
