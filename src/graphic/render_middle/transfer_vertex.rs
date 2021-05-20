use crate::graphic::base::color::RGBA;
use crate::graphic::base::point2d::PointVertex;
use crate::graphic::base::rectangle::RectVertex;
use crate::graphic::render_middle::texture_buffer::TextureVertex;

pub trait TransferVertex {
    fn to_tex(&self, w_width: u32, w_height: u32) -> Vec<TextureVertex>;
    fn to_rect_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> Vec<PointVertex>;
    fn to_round_rect_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> RectVertex;
}