use std::path::Path;

/// 图像数据结构体
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ImageRaw {
    /// 图像宽度
    pub width: u32,
    pub height: u32,
    /// 图像 RGBA 值
    pub data: Vec<u8>,
}

impl ImageRaw {
    pub fn new(image_path: &str) -> Self {
        Self::try_from_path(image_path).expect("cannot open image file")
    }

    pub fn try_from_path(image_path: &str) -> Option<Self> {
        let image_file = image::open(Path::new(image_path)).ok()?;
        Some(Self::from_dynamic(image_file))
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        let image_file = image::load_from_memory(bytes).ok()?;
        Some(Self::from_dynamic(image_file))
    }

    fn from_dynamic(image_file: image::DynamicImage) -> Self {
        let rgba = image_file.to_rgba8();
        let (width, height) = rgba.dimensions();
        ImageRaw {
            width,
            height,
            data: rgba.into_raw(),
        }
    }
}
