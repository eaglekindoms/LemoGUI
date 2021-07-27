use std::collections::HashMap;

use wgpu::*;
use wgpu::PrimitiveTopology::*;

use crate::graphic::base::circle_vertex::CircleVertex;
use crate::graphic::base::image_vertex::TextureVertex;
use crate::graphic::base::poly_vertex::PolyVertex;
use crate::graphic::base::rect_vertex::RectVertex;
use crate::graphic::base::shape::ShapeType;
use crate::graphic::render_middle::vertex_buffer_layout::VertexInterface;

/// 渲染管道状态元结构体
#[derive(Debug)]
pub struct PipelineState {
    context: HashMap<ShapeType, RenderPipeline>,
    // device:&'a Device
}

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
        Self {
            context,
            // device
        }
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
                let pipeline = PolyVertex::create_render_pipeline(device, LineStrip);
                if self.context.get(&ShapeType::BORDER).is_none() {
                    self.context.insert(ShapeType::BORDER, pipeline);
                }
            }
            ShapeType::POLYGON => {
                let pipeline = PolyVertex::create_render_pipeline(device, TriangleList);
                if self.context.get(&ShapeType::POLYGON).is_none() {
                    self.context.insert(ShapeType::POLYGON, pipeline);
                }
            }
            ShapeType::CIRCLE => {
                let pipeline = CircleVertex::create_render_pipeline(device, TriangleStrip);
                if self.context.get(&ShapeType::CIRCLE).is_none() {
                    self.context.insert(ShapeType::CIRCLE, pipeline);
                }
            }
        }
    }
    pub fn get_pipeline(&self, shape_type: ShapeType) -> Option<&RenderPipeline> {
        self.context.get(&shape_type)
    }
}

