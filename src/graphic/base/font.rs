use std::collections::HashMap;

use ab_glyph::*;

use crate::graphic::base::{BLACK, ImageRaw, RGBA};
use crate::graphic::render_middle::GTexture;

pub const DEFAULT_FONT_SIZE: f32 = 40.0;
pub const DEFAULT_FONT_COLOR: RGBA = BLACK;

#[derive(Debug)]
pub struct Character {
    // /// 字符
    // character:char,
    /// 字符范围
    scale: u32,
    ///    位图宽度（像素）
    pub width: u32,
    ///    位图高度（像素）
    pub height: u32,
    ///    水平距离，即位图相对于原点的水平位置（像素）
    pub bearingX: i32,
    ///    垂直距离，即位图相对于基准线的垂直位置（像素）
    pub bearingY: i32,
    ///    水平预留值，即原点到下一个字形原点的水平距离（单位：1/64像素）
    pub advance: u32,
    pub bitmap: Vec<u8>,
    texture: Option<GTexture>,
}

impl Character {
    /// 通过提供的字体和字符生成字形
    pub fn witch_scaled_font<F, SF>(scaled_font: &SF, character: char) -> Character
        where F: Font, SF: ScaleFont<F>
    {
        let glyph = scaled_font.scaled_glyph(character);
        let advance = scaled_font.h_advance(glyph.id) as u32;
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
        let bearing_y = self.bearingY;
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
}

/// save characters glyph map
#[derive(Debug)]
pub struct GCharMap<'font> {
    scaled_font: PxScaleFont<FontRef<'font>>,
    pub map: HashMap<char, Character>,
}

/// max glyph map count
pub const MAX_GLYPH_MAP_COUNT: usize = 400;

impl<'font> GCharMap<'font> {
    /// save ascii char map
    pub fn new(font: FontRef<'font>, font_size: f32) -> GCharMap<'font> {
        let mut characters = HashMap::<char, Character>::with_capacity(MAX_GLYPH_MAP_COUNT);
        let scale = PxScale::from(font_size);
        let scaled_font = font.into_scaled(scale);
        for c in 0u8..128 {
            if (c as char).is_control() || (c as char).is_whitespace() { continue; }
            let ch = Character::witch_scaled_font(&scaled_font, c as char);
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
            if c.is_whitespace() {
                let scale = self.scaled_font.scale().y as u32;
                self.map.insert(c, Character {
                    scale,
                    width: scale / 2,
                    height: scale,
                    bearingX: 0,
                    bearingY: 0,
                    advance: scale / 2,
                    bitmap: vec![0; (scale * scale / 2) as usize],
                    texture: None,
                });
            } else {
                let new_ch = Character::witch_scaled_font(&self.scaled_font, c);
                self.map.insert(c, new_ch);
            }
        }
        return self.map.get(&c).unwrap();
    }

    pub fn text_to_image(&mut self, text: &str) -> ImageRaw {
        let mut width = 0;
        let mut height = 0;
        let chars: Vec<ImageRaw> = text.chars()
            .map(|c| {
                let raw = self.character(c).to_raw();
                width += raw.width;
                height = raw.height;
                raw
            }).collect();
        println!("{:?}", width);

        let mut buffer = vec![0u8; (width * height) as usize];

        let mut offset = 0;
        for c in chars {
            for h in 0..c.height {
                for w in 0..c.width {
                    let pixel = w + c.width * h;
                    let offset_pixel = offset + w + h * width;
                    buffer[offset_pixel as usize] = c.data[pixel as usize];
                }
            }
            offset += c.width;
        }
        ImageRaw {
            width,
            height,
            data: buffer,
        }
    }
}
