use super::options::GenOptions;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

const LETTER_SPACING: i32 = 7;

pub fn gen_png(options: &GenOptions) {
    let mut image = RgbImage::new(1500, 500);

    let mulish_bytes = Vec::from(include_bytes!("../resources/mulish.ttf") as &[u8]);
    let mulish = Font::try_from_vec(mulish_bytes).unwrap();

    let montserrat_bytes = Vec::from(include_bytes!("../resources/montserrat.ttf") as &[u8]);
    let montserrat = Font::try_from_vec(montserrat_bytes).unwrap();

    let title_width = write_title(&options.title, 90.0, mulish, &mut image);
    if !options.subtitle.is_none() {
        write_subtitle(
            &options.subtitle.to_owned().unwrap(),
            title_width,
            35.0,
            montserrat,
            &mut image,
        );
    }

    let _ = image.save(format!("gen/{}.png", options.hash)).unwrap();
}

fn write_title(text: &str, height: f32, font: Font, image: &mut RgbImage) -> i32 {
    let scale = Scale {
        x: height,
        y: height,
    };

    let mut letter_x = 0;
    let mut total_width = 0;
    for letter in text.chars() {
        let letter_str = &letter.to_string();

        let (w, _) = text_size(scale, &font, letter_str);
        draw_text_mut(
            image,
            Rgb([0u8, 0u8, 255u8]),
            letter_x,
            0,
            scale,
            &font,
            letter_str,
        );

        letter_x += w + LETTER_SPACING;
        total_width += w + LETTER_SPACING;
    }

    total_width - LETTER_SPACING
}

fn write_subtitle(text: &str, title_width: i32, height: f32, font: Font, image: &mut RgbImage) {
    let scale = Scale {
        x: height,
        y: height,
    };

    let (w, _) = text_size(scale, &font, text);

    draw_text_mut(
        image,
        Rgb([0u8, 0u8, 255u8]),
        title_width - w,
        80,
        scale,
        &font,
        text,
    );
}
