use std::collections::HashMap;

use wgpu::*;
use wgpu::PrimitiveTopology::*;

use crate::graphic::base::poly_vertex::PolygonVertex;
use crate::graphic::base::image_vertex::TextureVertex;
use crate::graphic::base::triangle_vertex::PointVertex;
use crate::graphic::base::rect_vertex::RectVertex;
use crate::graphic::base::shape::ShapeType;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

/// 渲染管道状态元结构体
#[derive(Debug)]
pub struct PipelineState {
    context: HashMap<ShapeType, RenderPipeline>,
}

/// 图元渲染器
#[derive(Debug)]
pub struct Shader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl PipelineState {
    pub fn default(device: &Device) -> Self {
        // 固定渲染管道配置：纹理管道，矩形管道，边框管道。
        // 全局设置
        log::info!("create the PipelineState obj");
        let context = HashMap::with_capacity(4);
        let mut glob_pipeline = Self {
            context,
        };
        glob_pipeline.set_pipeline(device, ShapeType::ROUND);
        glob_pipeline.set_pipeline(device, ShapeType::POLYGON);
        glob_pipeline.set_pipeline(device, ShapeType::POINT);
        glob_pipeline.set_pipeline(device, ShapeType::BORDER);
        glob_pipeline.set_pipeline(device, ShapeType::TEXTURE);
        glob_pipeline
    }
    /// 创建渲染管道
    /// 参数：全局状态，着色器，渲染类型
    #[deprecated]
    pub fn create_pipeline_state<V>(device: &Device, fill_topology: PrimitiveTopology) -> RenderPipeline
        where V: VertexInterface {
        // 作用：绑定着色器，图形填充
        let render_pipeline = V::create_render_pipeline(device, fill_topology);
        return render_pipeline;
    }

    pub fn set_pipeline(&mut self, device: &Device, shape_type: ShapeType) {
        match shape_type {
            ShapeType::TEXTURE => {
                let pipeline = TextureVertex::create_render_pipeline(device, TriangleStrip);
                if self.context.get(&ShapeType::TEXTURE).is_none() {
                    self.context.insert(ShapeType::TEXTURE, pipeline);
                }
            }
            ShapeType::ROUND => {
                let pipeline = RectVertex::create_render_pipeline(device, TriangleStrip);
                if self.context.get(&ShapeType::ROUND).is_none() {
                    self.context.insert(ShapeType::ROUND, pipeline);
                }
            }
            ShapeType::BORDER => {
                let pipeline = PointVertex::create_render_pipeline(device, LineStrip);
                if self.context.get(&ShapeType::BORDER).is_none() {
                    self.context.insert(ShapeType::BORDER, pipeline);
                }
            }
            ShapeType::POINT => {
                let pipeline = PointVertex::create_render_pipeline(device, TriangleList);
                if self.context.get(&ShapeType::POINT).is_none() {
                    self.context.insert(ShapeType::POINT, pipeline);
                }
            }
            ShapeType::POLYGON => {
                let pipeline = PolygonVertex::create_render_pipeline(device, TriangleStrip);
                if self.context.get(&ShapeType::POLYGON).is_none() {
                    self.context.insert(ShapeType::POLYGON, pipeline);
                }
            }
        }
    }
    /// 获取渲染管线
    pub fn get_pipeline(&self, shape_type: ShapeType) -> Option<&RenderPipeline> {
        self.context.get(&shape_type)
    }
}

