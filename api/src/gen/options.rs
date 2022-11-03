use crate::color::hex::is_valid_color_hex;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Default)]
pub struct GenOptions {
    pub hash: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub color: String,
}

impl GenOptions {
    pub fn new(title: String, subtitle: Option<String>, color: String) -> Self {
        GenOptions {
            hash: {
                let mut hasher = DefaultHasher::new();
                title.hash(&mut hasher);
                subtitle.hash(&mut hasher);
                color.hash(&mut hasher);

                hasher.finish().to_string()
            },
            title,
            subtitle,
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
