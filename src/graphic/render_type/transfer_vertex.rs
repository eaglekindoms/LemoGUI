use crate::graphic::shape::point2d::{PointVertex, RGBA};
use crate::graphic::shape::rectangle::RectVertex;
use crate::graphic::shape::texture2d::TextureVertex;

pub trait TransferVertex {
    fn to_tex(&self, w_width: u32, w_height: u32) -> Vec<TextureVertex>;
    fn to_rect_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> Vec<PointVertex>;
    fn to_round_rect_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> RectVertex;
}