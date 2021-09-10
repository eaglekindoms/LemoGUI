use std::collections::HashMap;

use ab_glyph::*;
use image::{DynamicImage, Rgba, RgbaImage};

use crate::graphic::base::{BLACK, ImageRaw, Polygon, RGBA, ShapeGraph};
use crate::graphic::base;
use crate::graphic::render_middle::GTexture;
use crate::graphic::render_middle::VertexBuffer;

#[deprecated]
pub fn draw_text(f_scale: f32, font_color: RGBA, text: &str) -> ImageRaw {
    let font = FontRef::try_from_slice(include_bytes!("../../../res/SourceHanSansCN-Regular.otf")).unwrap();
    default_draw_text(font, f_scale, font_color, text)
}

pub fn default_draw_text(font: FontRef,
                         font_scale: f32, font_color: RGBA, text: &str) -> ImageRaw
{

    // The font size to use
    let scale = PxScale::from(font_scale);
    let scaled_font = font.as_scaled(scale);
    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, text, &mut glyphs);

    // Use a dark red colour
    let colour = font_color.to_u8();
    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = glyphs
        .iter()
        .last()
        .map(|g| g.position.x + scaled_font.h_advance(g.id))
        .unwrap_or(0.0)
        .ceil() as u32;

    // Create a witch_scaled_font rgba image with some padding
    // let mut image = DynamicImage::new_rgba8(glyphs_width + 20, glyphs_height-15).to_rgba8();
    let size = (glyphs_width) * (glyphs_height);
    let mut bufs1 = vec![0; (size * 4) as usize];

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                // println!("x: {} y: {}", x, y);
                let index = x + bounds.min.x as u32 - 10 + (glyphs_width) * (y + bounds.min.y as u32 - 22);
                bufs1[(index * 4) as usize] = colour.0;
                bufs1[(index * 4) as usize + 1] = colour.1;
                bufs1[(index * 4) as usize + 2] = colour.2;
                bufs1[(index * 4) as usize + 3] = (v * 255.0) as u8;
                // log::info!("+++++{} ++++++++",(v * 255.0) as u8);
            });
        }
    }
    ImageRaw {
        width: glyphs_width,
        height: glyphs_height,
        data: bufs1,
    }
}

fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}

#[derive(Debug)]
pub struct Character {
    // /// 字符
    // character:char,
    /// 字符范围
    scale: u32,
    ///    位图宽度（像素）
    width: u32,
    ///    位图高度（像素）
    height: u32,
    ///    水平距离，即位图相对于原点的水平位置（像素）
    bearingX: i32,
    ///    垂直距离，即位图相对于基准线的垂直位置（像素）
    bearingY: i32,
    ///    水平预留值，即原点到下一个字形原点的水平距离（单位：1/64像素）
    advance: u32,
    bitmap: Vec<u8>,
    texture: Option<GTexture>
}

impl Character {
    /// 通过提供的字体和字符生成字形
    pub fn witch_scaled_font<F, SF>(scaled_font: &SF, character: char) -> Character
        where F: Font, SF: ScaleFont<F>
    {
        let glyph = scaled_font.scaled_glyph(character);
        let mut advance = scaled_font.h_advance(glyph.id) as u32;
        let outlined = scaled_font.outline_glyph(glyph).expect("Failed to load Glyph! ");
        let bounds = outlined.px_bounds();
        let width = (bounds.max.x - bounds.min.x) as u32;
        let height = (bounds.max.y - bounds.min.y) as u32;
        let mut bitmap = vec![0; width as usize * height as usize];
        // Draw the glyph into the image per-pixel by using the draw closure
        outlined.draw(|x, y, v| {
            let index = x + y * width;
            bitmap[index as usize] = (v * 255.0) as u8;
        });
        // println!("bounds:{:?}", bounds);
        // println!("bitmap size: w: {},h: {}, by: {}", width, height, bounds.max.y);
        Character {
            scale: scaled_font.scale().y as u32,
            width,
            height,
            bearingX: bounds.min.x as i32,
            bearingY: height as i32 - bounds.max.y as i32,
            advance,
            bitmap,
            texture: None,
        }
    }

    pub fn to_raw(&self) -> ImageRaw {
        let mut advance = self.advance;
        let mut bearing_x = self.bearingX;
        let mut bearing_y = self.bearingY;
        if bearing_x < 0 {
            advance = (advance as i32 - bearing_x) as u32;
            bearing_x = 0;
        }
        if advance < self.width {
            advance = self.width;
        }
        let size = self.scale * advance;
        let mut buffer = vec![0; size as usize];
        for column in 0..self.height {
            for row in 0..self.width {
                let ch_index = row + column * self.width;
                let alpha = self.bitmap[ch_index as usize];
                let index_x = (row as i32 + bearing_x) as u32;
                let index_y = (self.scale as i32 * 3 / 4 + column as i32 - bearing_y) as u32;
                if index_y < self.scale {
                    // if index_x < advance {
                    let raw_index = index_x + advance * index_y;
                    buffer[raw_index as usize] = alpha;
                    // }
                }
            }
        }
        ImageRaw {
            width: advance,
            height: self.scale,
            data: buffer,
        }
    }

    pub fn texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> &GTexture {
        if self.texture.is_none() {
            let g_texture = GTexture::from_char(device, queue, &self);
            self.texture = Some(g_texture);
        }
        return self.texture.as_ref().unwrap();
    }

    pub fn get_scope(&self, x: i32, y: i32) {}
    // pub fn get_vertex_buffer(&self) -> VertexBuffer {
    //     let points = Polygon::new(vec![
    //         Point::new(0.2, -0.6),//0
    //         Point::new(0.4, -0.6),//1
    //         Point::new(0.5, -0.4),//2
    //         Point::new(0.4, -0.2),//3
    //         Point::new(0.2, -0.2),//4
    //         Point::new(0.1, -0.4),//5
    //     ]);
    //     points.to_buffer()
    // }
    pub fn draw(&self) {}
}

/// save characters glyph map
pub struct GCharMap<'a, F: Font> {
    scaled_font: &'a PxScaleFont<F>,
    map: HashMap<char, Character>,
}

/// max glyph map count
pub const MAX_GLYPH_MAP_COUNT: usize = 400;

impl<'a, F: Font> GCharMap<'a, F> {
    /// save ascii char map
    pub fn new(scaled_font: &'a PxScaleFont<F>) -> GCharMap<'a, F> {
        let mut characters = HashMap::<char, Character>::with_capacity(MAX_GLYPH_MAP_COUNT);
        for c in 0u8..128 {
            if (c as char).is_control() || (c as char).is_whitespace() { continue }
            let ch = Character::witch_scaled_font(scaled_font, c as char);
            characters.insert(c as char, ch);
        }
        GCharMap {
            scaled_font,
            map: characters,
        }
    }
    /// 获取指定字符字形
    pub fn character(&mut self, c: char) -> &Character {
        let ch = self.map.get(&c);
        if ch.is_none() {
            let new_ch = Character::witch_scaled_font(self.scaled_font, c);
            self.map.insert(c, new_ch);
        }
        return self.map.get(&c).unwrap();
    }

    pub fn text_to_image(&mut self, text: &str) -> ImageRaw {
        let mut width = 0;
        let chars: Vec<ImageRaw> = text.chars()
            .map(|c| {
                let raw = self.character(c).to_raw();
                width += raw.width;
                raw
            }).collect();
        println!("{:?}", width);

        ImageRaw {
            width: 0,
            height: 0,
            data: vec![],
        }
    }
}

fn test_font_to_image(font: FontRef, font_size: i32, font_color: RGBA) {
    // The font size to use
    let start = std::time::Instant::now();
    let color = font_color.to_u8();
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);
    let characters = GCharMap::new(&scaled_font);
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

#[test]
fn text_to_image() {
    let font = FontRef::try_from_slice(include_bytes!("../../../res/SourceHanSansCN-Regular.otf")).unwrap();
    let font_size = 40;
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);
    let mut characters = GCharMap::new(&scaled_font);
    let text = "adada";
    characters.text_to_image(text);
}

#[test]
fn test() {
    let now = std::time::Instant::now();
    let font = FontRef::try_from_slice(include_bytes!("../../../res/SourceHanSansCN-Regular.otf")).unwrap();
    test_font_to_image(font, 40, BLACK);
    let end = std::time::Instant::now();
    println!("time: {:?}", end - now);
}


pub fn one_char() -> Character {
    let font = FontRef::try_from_slice(include_bytes!("../../../res/SourceHanSansCN-Regular.otf")).unwrap();
    let font_size = 40;
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);
    let ch = Character::witch_scaled_font(&scaled_font, '请');
    return ch;
}

#[test]
fn print_char() {
    let now = std::time::Instant::now();
    let font = FontRef::try_from_slice(include_bytes!("../../../res/SourceHanSansCN-Regular.otf")).unwrap();
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