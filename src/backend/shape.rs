use super::style;
use super::style::*;

/// 颜色结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RGBA(pub [f32; 4]);

/// 二维顶点结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    x: f32,
    y: f32,
}

/// 2D纹理顶点数据
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturePoint {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

/// 2d图形缓存顶点结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BufferPoint {
    pub position: Point,
    pub color: RGBA,
}

impl BufferPoint {
    pub fn new(x: f32, y: f32, color: RGBA) -> Self {
        Self {
            position: Point { x, y },
            color,
        }
    }
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<BufferPoint>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

/// 矩形结构体
pub struct Rectangle {
    position: Point,
    width: u32,
    height: u32,
    style: style::Style,
}

pub trait TransferVertex {
    fn to_tex(&self, w_width: u32, w_height: u32) -> Vec<TexturePoint>;
    fn to_buff(&self, w_width: u32, w_height: u32, test_color: RGBA) -> Vec<BufferPoint>;
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rectangle {
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

impl TexturePoint {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<TexturePoint>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
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
}