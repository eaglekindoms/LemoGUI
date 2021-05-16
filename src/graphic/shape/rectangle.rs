use crate::graphic::shape::round_rectangle::RectVertex;
use crate::graphic::shape::texture_point::TexturePoint;
use crate::graphic::shape::point2d::{BufferPoint, Point, RGBA};
use crate::graphic::style;
use crate::graphic::style::Bordering;

/// 矩形结构体
#[derive(Debug)]
pub struct Rectangle {
    position: Point,
    width: u32,
    height: u32,
    style: style::Style,
}

pub trait TransferVertex {
    fn to_tex(&self, w_width: u32, w_height: u32) -> Vec<TexturePoint>;
    fn to_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> Vec<BufferPoint>;
    fn to_round(&self, w_width: u32, w_height: u32, test_color: RGBA) -> RectVertex;
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rectangle {
        log::info!("create the Rectangle obj");
        Rectangle {
            position: Point { x: x, y: y },
            width: w,
            height: h,
            style: style::Style::default(),
        }
    }
    pub fn set_border(&mut self, border: Bordering) {
        self.style = style::Style::set_border(&mut self.style, border);
    }
}


impl TransferVertex for Rectangle {
    fn to_tex(&self, w_width: u32, w_height: u32) -> Vec<TexturePoint> {
        let (t_x, t_y, t_w, t_h) =
            (2.0 * self.position.x as f32 / w_width as f32 - 1.0,
             1.0 - 2.0 * self.position.y as f32 / w_height as f32,
             2.0 * self.width as f32 / w_width as f32,
             2.0 * self.height as f32 / w_height as f32);

        let vect: Vec<TexturePoint> = vec![
            TexturePoint { position: [t_x, t_y], tex_coords: [0.0, 0.0] }, // B  1
            TexturePoint { position: [t_x + t_w, t_y], tex_coords: [1.0, 0.0] }, // B  1
            TexturePoint { position: [t_x, t_y - t_h], tex_coords: [0.0, 1.0] }, // B  1
            TexturePoint { position: [t_x + t_w, t_y - t_h], tex_coords: [1.0, 1.0] }, // B  1
        ];
        return vect;
    }

    fn to_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> Vec<BufferPoint> {
        let (t_x, t_y, t_w, t_h) =
            (2.0 * self.position.x as f32 / w_width as f32 - 1.0,
             1.0 - 2.0 * self.position.y as f32 / w_height as f32,
             2.0 * self.width as f32 / w_width as f32,
             2.0 * self.height as f32 / w_height as f32);

        let vect: Vec<BufferPoint> = vec![
            BufferPoint::new(t_x, t_y, test_color), // 左上
            BufferPoint::new(t_x + t_w, t_y, test_color), // 右上
            BufferPoint::new(t_x, t_y - t_h, test_color), // 左下
            BufferPoint::new(t_x + t_w, t_y - t_h, test_color), // 右下
        ];
        return vect;
    }

    fn to_round(&self, w_width: u32, w_height: u32, test_color: RGBA) -> RectVertex {
        let (t_x, t_y, t_w, t_h) =
            (2.0 * self.position.x as f32 / w_width as f32 - 1.0,
             1.0 - 2.0 * self.position.y as f32 / w_height as f32,
             2.0 * self.width as f32 / w_width as f32,
             2.0 * self.height as f32 / w_height as f32);

        RectVertex {
            size: [t_w, t_h],
            position: [t_w / 2.0 + t_x, t_h / 2.0 + t_y],
            border_color: [0.0, 0.0, 0.0, 1.0],
            frame_color: test_color.0,
            border_radius: 0.05,
            border_width: 0.005,
        }
    }
}