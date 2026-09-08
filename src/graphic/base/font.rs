use std::collections::{HashMap, VecDeque};
use std::path::Path;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont};

use crate::backend::wgpu_impl::*;
use crate::graphic::base::{quantize_size, ImageRaw, BLACK, RGBA};

pub const DEFAULT_FONT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/res/SourceHanSansCN-Regular.otf");
pub const DEFAULT_FONT_SIZE: f32 = 40.0;
pub const DEFAULT_FONT_COLOR: RGBA = BLACK;
/// 非驻留字形 GPU 纹理上限
pub const GLYPH_TEXTURE_LIMIT: usize = 512;
/// max glyph map count
pub const DEFAULT_GLYPH_MAP_COUNT: usize = 400;

/// 字形度量（排版用，不含位图/纹理）
#[derive(Debug, Copy, Clone)]
pub struct GlyphMetrics {
    pub advance: u32,
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub scale: u32,
}

/// 字形结构体
#[derive(Debug)]
pub struct Character {
    pub character: char,
    scale: u32,
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance: u32,
    pub bitmap: Vec<u8>,
    pub texture: Option<TextureBufferData>,
}

impl Character {
    pub fn metrics(&self) -> GlyphMetrics {
        GlyphMetrics {
            advance: self.advance,
            width: self.width,
            height: self.height,
            bearing_x: self.bearing_x,
            bearing_y: self.bearing_y,
            scale: self.scale,
        }
    }

    /// 通过提供的字体和字符生成字形
    pub fn witch_scaled_font<F, SF>(scaled_font: &SF, character: char) -> Self
    where
        F: Font,
        SF: ScaleFont<F>,
    {
        rasterize_scaled(scaled_font, character)
            .unwrap_or_else(|| blank_character(scaled_font.scale().y as u32))
    }

    /// 将字形转为单通道二维图像(只有alpha值)
    pub fn to_raw(&self) -> ImageRaw {
        let mut advance = self.advance;
        let mut bearing_x = self.bearing_x;
        let bearing_y = self.bearing_y;
        if bearing_x < 0 {
            advance = (advance as i32 - bearing_x) as u32;
            bearing_x = 0;
        }
        if advance < self.width {
            advance = self.width;
        }
        if advance == 0 {
            advance = 1;
        }
        let scale = self.scale.max(1);
        let size = scale * advance;
        let mut buffer = vec![0; size as usize];
        if !self.bitmap.is_empty() && self.width > 0 {
            for column in 0..self.height {
                for row in 0..self.width {
                    let ch_index = row + column * self.width;
                    if ch_index as usize >= self.bitmap.len() {
                        continue;
                    }
                    let alpha = self.bitmap[ch_index as usize];
                    let index_x = (row as i32 + bearing_x) as u32;
                    let index_y = (scale as i32 * 3 / 4 + column as i32 - bearing_y) as u32;
                    if index_y < scale && index_x < advance {
                        let raw_index = index_x + advance * index_y;
                        if (raw_index as usize) < buffer.len() {
                            buffer[raw_index as usize] = alpha;
                        }
                    }
                }
            }
        }
        ImageRaw {
            width: advance,
            height: scale,
            data: buffer,
        }
    }

    /// 给字形生成纹理缓冲数据
    pub fn set_texture(
        &mut self,
        g_texture: &mut GTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> &TextureBufferData {
        if self.texture.is_none() {
            let raw_data = self.to_raw();
            self.texture = Some(g_texture.create_bind_group(device, queue, raw_data));
            self.bitmap.clear();
            self.bitmap.shrink_to_fit();
        }
        return self.texture.as_ref().unwrap();
    }
}

fn rasterize_scaled<F, SF>(scaled_font: &SF, character: char) -> Option<Character>
where
    F: Font,
    SF: ScaleFont<F>,
{
    let scale = scaled_font.scale().y as u32;
    if character.is_control() || character.is_whitespace() {
        return Some(blank_character(scale));
    }
    let glyph = scaled_font.scaled_glyph(character);
    let advance = scaled_font.h_advance(glyph.id) as u32;
    let outlined = scaled_font.outline_glyph(glyph)?;
    let bounds = outlined.px_bounds();
    let width = (bounds.max.x - bounds.min.x) as u32;
    let height = (bounds.max.y - bounds.min.y) as u32;
    if width == 0 || height == 0 {
        return Some(blank_character(scale));
    }
    let mut bitmap = vec![0; width as usize * height as usize];
    outlined.draw(|x, y, v| {
        let index = x + y * width;
        if (index as usize) < bitmap.len() {
            bitmap[index as usize] = (v * 255.0) as u8;
        }
    });
    Some(Character {
        character,
        scale,
        width,
        height,
        bearing_x: bounds.min.x as i32,
        bearing_y: height as i32 - bounds.max.y as i32,
        advance: advance.max(1),
        bitmap,
        texture: None,
    })
}

fn compute_metrics(font: &FontVec, c: char, bucket: u32) -> GlyphMetrics {
    if c.is_control() || c.is_whitespace() {
        return blank_metrics(bucket);
    }
    let scaled = font.as_scaled(PxScale::from(bucket as f32));
    let glyph = scaled.scaled_glyph(c);
    let advance = scaled.h_advance(glyph.id) as u32;
    if let Some(outlined) = scaled.outline_glyph(glyph) {
        let bounds = outlined.px_bounds();
        let width = (bounds.max.x - bounds.min.x) as u32;
        let height = (bounds.max.y - bounds.min.y) as u32;
        GlyphMetrics {
            advance: advance.max(1),
            width,
            height,
            bearing_x: bounds.min.x as i32,
            bearing_y: height as i32 - bounds.max.y as i32,
            scale: bucket,
        }
    } else {
        blank_metrics(bucket)
    }
}

fn blank_metrics(scale: u32) -> GlyphMetrics {
    let scale = scale.max(1);
    GlyphMetrics {
        advance: scale / 2,
        width: scale / 2,
        height: scale,
        bearing_x: 0,
        bearing_y: 0,
        scale,
    }
}

fn blank_character(scale: u32) -> Character {
    let m = blank_metrics(scale);
    Character {
        character: ' ',
        scale: m.scale,
        width: m.width,
        height: m.height,
        bearing_x: m.bearing_x,
        bearing_y: m.bearing_y,
        advance: m.advance,
        bitmap: Vec::new(),
        texture: None,
    }
}

fn rasterize(font: &FontVec, c: char, bucket: u32) -> Character {
    let scaled = font.as_scaled(PxScale::from(bucket as f32));
    rasterize_scaled(&scaled, c).unwrap_or_else(|| blank_character(bucket))
}

type GlyphKey = (char, u32, u8);

/// 手写 LRU：超限驱逐最旧 GPU 字形
#[derive(Debug)]
struct GlyphLru {
    cap: usize,
    order: VecDeque<GlyphKey>,
    slots: HashMap<GlyphKey, Character>,
}

impl GlyphLru {
    fn new(cap: usize) -> Self {
        Self {
            cap,
            order: VecDeque::with_capacity(cap),
            slots: HashMap::with_capacity(cap),
        }
    }

    fn contains(&self, key: &GlyphKey) -> bool {
        self.slots.contains_key(key)
    }

    fn peek(&self, key: &GlyphKey) -> Option<&Character> {
        self.slots.get(key)
    }

    fn get_mut(&mut self, key: GlyphKey) -> Option<&mut Character> {
        if self.slots.contains_key(&key) {
            self.touch(key);
            self.slots.get_mut(&key)
        } else {
            None
        }
    }

    fn insert(&mut self, key: GlyphKey, slot: Character) {
        if self.slots.contains_key(&key) {
            self.touch(key);
            self.slots.insert(key, slot);
            return;
        }
        while self.slots.len() >= self.cap {
            if let Some(old) = self.order.pop_front() {
                self.slots.remove(&old);
            } else {
                break;
            }
        }
        self.order.push_back(key);
        self.slots.insert(key, slot);
    }

    fn touch(&mut self, key: GlyphKey) {
        self.order.retain(|k| *k != key);
        self.order.push_back(key);
    }
}

/// 字形容器：多字库 FontVec + 度量缓存 + 驻留 ASCII + LRU 纹理
#[derive(Debug)]
pub struct GCharMap {
    pub scale: f32,
    faces: HashMap<u8, FontVec>,
    metrics: HashMap<GlyphKey, GlyphMetrics>,
    resident: HashMap<char, Character>,
    lru: GlyphLru,
}

impl GCharMap {
    /// 加载字体；只预热默认字号 ASCII
    pub fn new(font_path: String, font_size: f32) -> GCharMap {
        let path = Path::new(font_path.as_str());
        let font_bits = std::fs::read(path).unwrap();
        let font = FontVec::try_from_vec(font_bits).expect("import font failed");
        let bucket = quantize_size(font_size);
        let mut resident = HashMap::<char, Character>::with_capacity(96);
        let mut metrics = HashMap::<GlyphKey, GlyphMetrics>::with_capacity(DEFAULT_GLYPH_MAP_COUNT);
        for c in 0u8..128 {
            let ch = c as char;
            if ch.is_control() || ch.is_whitespace() {
                continue;
            }
            let glyph = rasterize(&font, ch, bucket);
            metrics.insert((ch, bucket, 0), glyph.metrics());
            resident.insert(ch, glyph);
        }
        let mut faces = HashMap::new();
        faces.insert(0, font);
        GCharMap {
            scale: font_size,
            faces,
            metrics,
            resident,
            lru: GlyphLru::new(GLYPH_TEXTURE_LIMIT),
        }
    }

    fn default_bucket(&self) -> u32 {
        quantize_size(self.scale)
    }

    fn resolve_font(&self, font_id: u8) -> u8 {
        if self.faces.contains_key(&font_id) {
            font_id
        } else {
            0
        }
    }

    /// 按需加载字库；失败则保持仅有 id 0
    pub fn ensure_font(&mut self, id: u8, path: &str) {
        if self.faces.contains_key(&id) {
            return;
        }
        if let Ok(bits) = std::fs::read(path) {
            if let Ok(font) = FontVec::try_from_vec(bits) {
                self.faces.insert(id, font);
            }
        }
    }

    fn is_resident_key(&self, c: char, bucket: u32, font_id: u8) -> bool {
        font_id == 0
            && bucket == self.default_bucket()
            && (c as u32) < 128
            && !c.is_control()
            && !c.is_whitespace()
    }

    fn has_slot(&self, c: char, bucket: u32, font_id: u8) -> bool {
        if font_id == 0 && bucket == self.default_bucket() && self.resident.contains_key(&c) {
            true
        } else {
            self.lru.contains(&(c, bucket, font_id))
        }
    }

    fn get_slot(&self, c: char, bucket: u32, font_id: u8) -> Option<&Character> {
        if font_id == 0 && bucket == self.default_bucket() {
            if let Some(s) = self.resident.get(&c) {
                return Some(s);
            }
        }
        self.lru.peek(&(c, bucket, font_id))
    }

    fn store_slot(&mut self, c: char, bucket: u32, font_id: u8, ch: Character) {
        if self.is_resident_key(c, bucket, font_id) {
            self.resident.insert(c, ch);
        } else {
            self.lru.insert((c, bucket, font_id), ch);
        }
    }

    fn ensure_rasterized(&mut self, c: char, bucket: u32, font_id: u8) {
        let font_id = self.resolve_font(font_id);
        if self.has_slot(c, bucket, font_id) {
            return;
        }
        let glyph = {
            let face = self.faces.get(&font_id).or_else(|| self.faces.get(&0)).unwrap();
            rasterize(face, c, bucket)
        };
        self.metrics.insert((c, bucket, font_id), glyph.metrics());
        self.store_slot(c, bucket, font_id, glyph);
    }

    fn ensure_textured(
        &mut self,
        c: char,
        bucket: u32,
        font_id: u8,
        g_texture: &mut GTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let font_id = self.resolve_font(font_id);
        self.ensure_rasterized(c, bucket, font_id);
        let slot = if font_id == 0 && bucket == self.default_bucket() && self.resident.contains_key(&c)
        {
            self.resident.get_mut(&c).unwrap()
        } else {
            self.lru
                .get_mut((c, bucket, font_id))
                .expect("slot just inserted")
        };
        if slot.texture.is_none() {
            slot.set_texture(g_texture, device, queue);
        }
    }

    /// 排版度量：不触发 GPU 上传
    pub fn metrics(&mut self, c: char, size: f32) -> GlyphMetrics {
        self.metrics_font(c, size, 0)
    }

    pub fn metrics_font(&mut self, c: char, size: f32, font_id: u8) -> GlyphMetrics {
        let bucket = quantize_size(size);
        let font_id = self.resolve_font(font_id);
        if let Some(m) = self.metrics.get(&(c, bucket, font_id)) {
            return *m;
        }
        let m = {
            let face = self.faces.get(&font_id).or_else(|| self.faces.get(&0)).unwrap();
            compute_metrics(face, c, bucket)
        };
        self.metrics.insert((c, bucket, font_id), m);
        m
    }

    /// 获取指定字符字形（默认 UI 字号）
    pub fn character(&mut self, c: char) -> &Character {
        let bucket = self.default_bucket();
        self.ensure_rasterized(c, bucket, 0);
        self.get_slot(c, bucket, 0).expect("slot just inserted")
    }

    /// 默认字号纹理（兼容现有 draw_text）
    pub fn character_texture(
        &mut self,
        c: char,
        g_texture: &mut GTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> &Character {
        self.character_texture_at(c, self.scale, 0, g_texture, device, queue)
    }

    /// 指定字号/字族纹理；命中驻留/LRU 则只查表
    pub fn character_texture_at(
        &mut self,
        c: char,
        size: f32,
        font_id: u8,
        g_texture: &mut GTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> &Character {
        let bucket = quantize_size(size);
        let font_id = self.resolve_font(font_id);
        self.ensure_textured(c, bucket, font_id, g_texture, device, queue);
        self.get_slot(c, bucket, font_id).expect("textured slot")
    }

    /// 把字符串文本转换成单通道图像数据
    pub fn text_to_image(&mut self, text: &str) -> ImageRaw {
        let mut width = 0;
        let mut height = 0;
        let chars: Vec<ImageRaw> = text
            .chars()
            .map(|c| {
                self.ensure_rasterized(c, self.default_bucket(), 0);
                let raw = self.character(c).to_raw();
                width += raw.width;
                height = raw.height;
                raw
            })
            .collect();

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
