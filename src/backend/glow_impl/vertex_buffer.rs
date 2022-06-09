use crate::backend::glow_impl::*;
use crate::backend::wgpu_impl::ShapeType;
use crate::graphic::base::*;
use bytemuck::offset_of;
use glow::HasContext;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GLBuffer<'a> {
    /// Contents of a buffer on creation.
    pub contents: &'a [u8],
}

/// 渲染顶点缓冲结构体
#[derive(Debug)]
pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub shape_type: ShapeType,
}

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

/// glow描述顶点布局信息的trait
pub trait VertexLayout: Sized {
    /// 返回顶点布局信息数组
    fn set_vertex_layout() -> Vec<VertexLayoutInfo>;
    fn set_shader(context: &Arc<glow::Context>) -> GLShader {
        GLShader::new(
            &context,
            include_str!("./shader/triangle.vert"),
            include_str!("./shader/triangle.frag"),
        )
    }

    fn create_render_pipeline(context: &Arc<glow::Context>) -> GLPipeline {
        let shader = Self::set_shader(context);
        let program = shader.link_program(&context);
        let vao = unsafe { context.create_vertex_array().unwrap() };
        // 顶点缓冲
        let vbo = unsafe { context.create_buffer().unwrap() };
        // 顶点索引缓冲
        let ebo = unsafe { context.create_buffer().unwrap() };
        unsafe {
            context.bind_vertex_array(Some(vao));
            context.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            context.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            for layout in Self::set_vertex_layout() {
                //对于数据是紧凑的，stride步长可取0，也可设为(vector_size * std::mem::size_of::<data_type>())
                context.vertex_attrib_pointer_f32(
                    layout.location,
                    layout.vector_size,
                    layout.data_type,
                    layout.normalized,
                    layout.stride,
                    layout.offset,
                );
                context.enable_vertex_attrib_array(layout.location);
            }
            context.bind_vertex_array(None);
        }
        GLPipeline {
            context: Arc::clone(context),
            program,
            vao,
            vbo,
            ebo,
        }
    }
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

impl VertexLayout for RectVertex {
    fn set_vertex_layout() -> Vec<VertexLayoutInfo> {
        let stride = std::mem::size_of::<RectVertex>() as i32;
        vec![
            VertexLayoutInfo::new(
                0,
                2,
                glow::FLOAT,
                false,
                stride,
                offset_of!(RectVertex, size) as i32,
            ),
            VertexLayoutInfo::new(
                1,
                2,
                glow::FLOAT,
                false,
                stride,
                offset_of!(RectVertex, position) as i32,
            ),
            VertexLayoutInfo::new(
                2,
                4,
                glow::FLOAT,
                false,
                stride,
                offset_of!(RectVertex, border_color) as i32,
            ),
            VertexLayoutInfo::new(
                3,
                4,
                glow::FLOAT,
                false,
                stride,
                offset_of!(RectVertex, rect_color) as i32,
            ),
            VertexLayoutInfo::new(
                4,
                2,
                glow::UNSIGNED_INT,
                false,
                stride,
                offset_of!(RectVertex, is_round_or_border) as i32,
            ),
        ]
    }
}

impl VertexLayout for PointVertex {
    fn set_vertex_layout() -> Vec<VertexLayoutInfo> {
        let stride = std::mem::size_of::<PointVertex>() as i32;
        vec![
            VertexLayoutInfo::new(
                0,
                2,
                glow::FLOAT,
                false,
                stride,
                offset_of!(PointVertex, position) as i32,
            ),
            VertexLayoutInfo::new(
                1,
                4,
                glow::FLOAT,
                false,
                stride,
                offset_of!(PointVertex, color) as i32,
            ),
        ]
    }
}
