use std::collections::HashMap;

use wgpu::*;
use wgpu::PrimitiveTopology::*;

use crate::backend::wgpu_impl::*;
use crate::graphic::base::ShapeType;

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
        glob_pipeline.set_pipeline::<RectVertex>(device);
        glob_pipeline.set_pipeline::<CircleVertex>(device);
        glob_pipeline.set_pipeline::<PointVertex>(device);
        glob_pipeline.set_pipeline::<TextureVertex>(device);
        glob_pipeline
    }
    /// 创建渲染管道
    /// 参数：全局状态，着色器，渲染类型
    pub fn set_pipeline<V>(&mut self, device: &Device) where V: VertexLayout {
        // 作用：绑定着色器，图形填充
        let render_pipeline = V::create_render_pipeline(device);
        let shape_type = V::get_shape_type();
        if self.context.get(&shape_type).is_none() {
            self.context.insert(shape_type, render_pipeline);
        }
    }
    /// 获取渲染管线
    pub fn get_pipeline(&self, shape_type: ShapeType) -> Option<&RenderPipeline> {
        self.context.get(&shape_type)
    }
}

