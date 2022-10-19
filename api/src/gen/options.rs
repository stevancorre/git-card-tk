use crate::color::hex::is_valid_color_hex;

#[derive(Default)]
pub struct GenOptions {
    pub title: String,
    pub color: String,
}

impl GenOptions {
    pub fn new(title: String, color: String) -> Self {
        GenOptions {
            title,
            color: if is_valid_color_hex(&*color) {
                if color.len() == 1 {
                    format!("#{}{}{}", color, color, color)
                } else {
                    format!("#{}", color)
                }
            } else {
                color
            },
        }
    }
}
