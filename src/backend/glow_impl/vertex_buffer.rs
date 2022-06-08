use crate::graphic::base::*;
use bytemuck::offset_of;
use glow::HasContext;

pub struct VertexLayoutInfo {
    pub location: u32,
    pub vector_size: i32,
    pub data_type: u32,
    pub normalized: bool,
    pub stride: i32,
    pub offset: i32,
}

impl VertexLayoutInfo {
    pub fn new(
        location: u32,
        vector_size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) -> Self {
        VertexLayoutInfo {
            location,
            vector_size,
            data_type,
            normalized,
            stride,
            offset,
        }
    }
}

pub trait VertexLayout: Sized {
    fn set_vertex_layout() -> Vec<VertexLayoutInfo>;
}

impl VertexLayout for CircleVertex {
    fn set_vertex_layout() -> Vec<VertexLayoutInfo> {
        let stride = std::mem::size_of::<CircleVertex>() as i32;
        vec![
            VertexLayoutInfo::new(
                0,
                2,
                glow::FLOAT,
                false,
                stride,
                offset_of!(CircleVertex, position) as i32,
            ),
            VertexLayoutInfo::new(
                1,
                4,
                glow::FLOAT,
                false,
                stride,
                offset_of!(CircleVertex, color) as i32,
            ),
            VertexLayoutInfo::new(
                2,
                1,
                glow::FLOAT,
                false,
                stride,
                offset_of!(CircleVertex, radius) as i32,
            ),
            VertexLayoutInfo::new(
                3,
                1,
                glow::UNSIGNED_INT,
                false,
                stride,
                offset_of!(CircleVertex, edge) as i32,
            ),
        ]
    }
}
