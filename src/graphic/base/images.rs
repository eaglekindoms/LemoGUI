use std::path::Path;

use image::GenericImageView;

/// 图像数据结构体
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ImageRaw {
    /// 图像宽度
    pub width: u32,
    /// 图像高度
    pub height: u32,
    /// 图像RGBA值
    pub data: Vec<u8>,
}

impl ImageRaw {
    pub fn new(image_path: &str) -> Self {
        let image_file = image::open(Path::new(image_path))
            .expect("cannot open image file");
        let raw_data = image_file.as_bytes();
        let (width, height) = image_file.dimensions();
        ImageRaw {
            width,
            height,
            data: raw_data.to_vec(),
        }
    }
}