use crate::backend::wgpu_impl::*;
use crate::device::*;
use crate::graphic::base::*;

impl ShapeGraph for Rectangle {
    fn to_buffer(&self, gpu_context: &GPUContext, color: RGBA) -> VertexBuffer {
        let rect_vertex = RectVertex::new(&self, gpu_context.get_surface_size(), color);
        let rect_vertex = VertexBuffer::create_vertex_buf::<RectVertex>
            (&gpu_context.device, vec![rect_vertex], RECT_INDEX);
        rect_vertex
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::ROUND
    }
}

impl ShapeGraph for Circle {
    fn to_buffer(&self, gpu_context: &GPUContext, color: RGBA) -> VertexBuffer {
        let circle_vertex
            = CircleVertex::new(&self, 0, color);
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>
            (&gpu_context.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }
}

impl ShapeGraph for RegularPolygon {
    fn to_buffer(&self, gpu_context: &GPUContext, color: RGBA) -> VertexBuffer {
        let circle_vertex
            = CircleVertex::new(&self.point, self.edge, color);
        let cricle_buffer = VertexBuffer::create_vertex_buf::<CircleVertex>
            (&gpu_context.device, vec![circle_vertex], RECT_INDEX);
        cricle_buffer
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }
}

impl ShapeGraph for Polygon {
    fn to_buffer(&self, gpu_context: &GPUContext, color: RGBA) -> VertexBuffer {
        PointVertex::from_shape_to_vector(gpu_context, &self.points, color)
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::POINT
    }
}