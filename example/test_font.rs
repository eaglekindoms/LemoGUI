use ab_glyph::{Font, FontRef, PxScale};
use image::{DynamicImage, Rgba, RgbaImage};

use LemoGUI::graphic::base::{BLACK, Character, GCharMap, ImageRaw, RGBA};

fn main() {}

#[test]
fn text_to_image() {
    let font = FontRef::try_from_slice(include_bytes!("../res/SourceHanSansCN-Regular.otf")).unwrap();
    let font_size = 40;
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);
    let mut characters = GCharMap::new(font, font_size as f32);
    let text = "adadaon";
    let raw = single_text(&mut characters, text);
    let mut image = DynamicImage::new_rgba8(raw.width, raw.height).to_rgba8();

    for i in 0..raw.height {
        for j in 0..raw.width {
            let px = image.get_pixel_mut(j, i);
            let index = (j + i * raw.width) as usize;
            *px = Rgba([0, 0, 0, raw.data[index]]);
        }
    }
    image.save("test.png").unwrap();
}

fn single_text(char_map: &mut GCharMap, text: &str) -> ImageRaw {
    char_map.text_to_image(text)
}

#[test]
fn test() {
    let now = std::time::Instant::now();
    let font = FontRef::try_from_slice(include_bytes!("../res/SourceHanSansCN-Regular.otf")).unwrap();
    test_font_to_image(font, 40, BLACK);
    let end = std::time::Instant::now();
    println!("time: {:?}", end - now);
}


#[test]
fn print_char() {
    let now = std::time::Instant::now();
    let font = FontRef::try_from_slice(include_bytes!("../res/SourceHanSansCN-Regular.otf")).unwrap();
    let font_size = 40;
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);
    let ch = Character::witch_scaled_font(&scaled_font, '请');
    for i in 0..ch.height {
        for j in 0..ch.width {
            let index = j + i * ch.width;
            let char_s = ch.bitmap[index as usize];
            print!("\x1B[48;2;{};{};{}m   ", char_s, char_s, char_s);
        }
        println!("\x1B[0m");
    }
}

fn format_image(image: &mut RgbaImage, scale: i32) {
    for line in (0..image.height()).step_by(scale as usize) {
        for row in 0..image.width() {
            let px = image.get_pixel_mut(row, line);
            *px = Rgba([0, 0, 0, 100]);
        }
    }
}

fn test_font_to_image(font: FontRef, font_size: i32, font_color: RGBA) {
    // The font size to use
    let start = std::time::Instant::now();
    let color = font_color.to_u8();
    // let scale = PxScale::from(font_size as f32);
    // let scaled_font = font.as_scaled(scale);
    let characters = GCharMap::new(font, font_size as f32);
    let mut image = DynamicImage::new_rgba8(555, 555).to_rgba8();
    let mut x: i32 = 2;
    let mut y: i32 = 0;
    let mut column = 0;
    for (c, ch) in characters.map.iter() {
        print!(" {:?},bx:{:?},w:{:?},ad:{:?}##", c, ch.bearingX, ch.width, ch.advance);
        let raw = ch.to_raw();
        for i in 0..raw.height {
            for j in 0..raw.width {
                let px = image.get_pixel_mut(x as u32 + j, y as u32 + i);
                let index = (j + i * raw.width) as usize;
                *px = Rgba([0, 0, 0, raw.data[index]]);
            }
        }
        let xpos = (x + 260 + ch.bearingX) as u32;
        let ypos = (y + font_size - ch.bearingY) as u32;
        for i in 0..ch.height {
            for j in 0..ch.width {
                let px = image.get_pixel_mut(xpos + j, ypos + i);
                let index = j + i * ch.width;
                *px = Rgba([
                    color.0,
                    color.1,
                    color.2,
                    ch.bitmap[index as usize],
                ]);
            }
        }
        x += ch.advance as i32;
        column += 1;
        if column > 10 {
            y += font_size;
            column = 0;
            x = 0;
        }
        // }
    };
    format_image(&mut image, font_size);
    let end = std::time::Instant::now();
    image.save("test.png").unwrap();
    println!("generate bitmap time: {:?}", end - start);
}
