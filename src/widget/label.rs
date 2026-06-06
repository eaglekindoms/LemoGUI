use std::fmt::Debug;
use std::option::Option::Some;

use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;

/// 控件面板结构体
#[derive(Debug)]
pub struct Label {
    /// 面板尺寸
    pub size: Rectangle,
    /// 面板样式
    pub style: Style,
    /// 面板文本
    pub text: Option<String>,
    /// 面板图像
    pub image: Option<ImageRaw>,
}

impl Label {
    /// 创建文本面板
    pub fn new_text_label(rect: Rectangle, style: Style, text: String) -> Self {
        log::info!("create text label");
        Self {
            size: rect,
            style,
            text: Some(text),
            image: None,
        }
    }
    /// 创建图像面板 By 图像路径
    pub fn new_image_label_by_path(rect: Rectangle, style: Style, image_path: String) -> Self {
        log::info!("create image label");
        Self {
            size: rect,
            style,
            text: None,
            image: Some(ImageRaw::new(image_path.as_str())),
        }
    }

    /// 创建图像面板
    pub fn new_image_label(rect: Rectangle, style: Style, image: ImageRaw) -> Self {
        log::info!("create image label");
        Self {
            size: rect,
            style,
            text: None,
            image: Some(image),
        }
    }
    /// 绘制label
    pub fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let shape: Box<dyn ShapeGraph> = Box::new(self.size);
        paint_brush.draw_shape(&shape, self.style);
        if let Some(text) = &self.text {
            log::info!("draw label's text");
            paint_brush.draw_text(
                font_map,
                &self.size,
                text.as_str(),
                self.style.get_font_color(),
            );
        }
        if let Some(image) = &self.image {
            log::info!("draw label's image");
            paint_brush.draw_image(&self.size, image.clone())
        }
    }
}
