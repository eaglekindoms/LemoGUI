use crate::adapter::*;
use crate::backend::wgpu_impl::*;
use crate::graphic::base::*;
use crate::graphic::style::Style;

/// 图形类型枚举
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShapeType {
    /// 纹理
    TEXTURE,
    /// 圆角
    ROUND,
    /// 线框
    BORDER,
    /// 点
    POINT,
    /// 圆
    Circle,
}

impl ShapeGraph for Rectangle {
    fn to_buffer(&self, gpu_context: &GPUContext, style: Style) -> VertexBuffer {
        let rect_vertex = RectVertex::new(&self, style);
        let rect_vertex = VertexBuffer::create_vertex_buf::<RectVertex>(
            &gpu_context.device,
            vec![rect_vertex],
            RECT_INDEX,
        );
        rect_vertex
    }
}

impl ShapeGraph for Circle {
    fn to_buffer(&self, gpu_context: &GPUContext, style: Style) -> VertexBuffer {
        let circle_vertex = CircleVertex::new(&self, 0, style.get_display_color());
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>(
            &gpu_context.device,
            vec![circle_vertex],
            RECT_INDEX,
        );
        cricle_buffer
    }
}

impl ShapeGraph for RegularPolygon {
    fn to_buffer(&self, gpu_context: &GPUContext, style: Style) -> VertexBuffer {
        let circle_vertex = CircleVertex::new(&self.point, self.edge, style.get_display_color());
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>(
            &gpu_context.device,
            vec![circle_vertex],
            RECT_INDEX,
        );
        cricle_buffer
    }
}

impl ShapeGraph for Polygon {
    fn to_buffer(&self, gpu_context: &GPUContext, style: Style) -> VertexBuffer {
        PointVertex::from_shape_to_vector(gpu_context, &self.points, style.get_display_color())
    }
}
