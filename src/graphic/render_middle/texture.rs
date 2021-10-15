use std::num::NonZeroU32;

use wgpu::TextureFormat;

use crate::graphic::base::*;

#[derive(Debug)]
pub struct GTexture {
    // pub texture: wgpu::Texture,
    // pub view: wgpu::TextureView,
    // pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    // pub bind_group_layout:wgpu::BindGroupLayout,
    pub size: Point<u32>,
}

impl GTexture {
    pub fn from_char(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        ch: &Character,
    ) -> Self {
        let raw_data = ch.to_raw();

        Self::from_raw_image(device, queue, raw_data, wgpu::TextureFormat::R8Unorm)
    }

    pub fn from_text(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        char_map: &mut GCharMap,
        text: &str,
    ) -> Self {
        let raw_data = char_map.text_to_image(text);
        Self::from_raw_image(device, queue, raw_data, wgpu::TextureFormat::R8Unorm)
    }

    pub fn from_raw_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        raw_data: ImageRaw,
        texture_format: wgpu::TextureFormat,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: raw_data.width,
            height: raw_data.height,
            depth_or_array_layers: 1,
        };
        // 参数：纹理数据来源的尺寸
        // 用途：指定纹理数据的布局
        // 具体含义：偏移量，行数宽度，列数宽度
        // 注：图像纹理导入后会被转化为包含每个像素点rgba颜色值的一维数组
        // 因此行数宽度为图像宽度*4，列数宽度不变
        let image_width: u32;
        match texture_format {
            TextureFormat::R8Unorm => image_width = raw_data.width,
            _ => image_width = raw_data.width * 4
        }
        let image_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(image_width),
            rows_per_image: NonZeroU32::new(raw_data.height),
        };
        // 定义纹理描述符
        // 参数：纹理尺寸
        // 输出配置：定义纹理尺寸，维度：2d，颜色格式：rgba，纹理来源：sampled,copy_dst
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
        let view = writer_data_to_texture(queue, &texture, image_layout, size, raw_data);
        let sampler = device.create_sampler(DEFAULT_TEXTURE_SAMPLER);
        let layout = device.create_bind_group_layout(DEFAULT_BIND_GROUP_LAYOUT);
        let bind_group = bind_group(device, &layout, &view, &sampler);
        Self {
            bind_group,
            // bind_group_layout:layout,
            size: Point::new(size.width, size.height),
        }
    }
}

pub fn writer_data_to_texture(queue: &wgpu::Queue,
                              texture: &wgpu::Texture,
                              image_layout: wgpu::ImageDataLayout,
                              size: wgpu::Extent3d,
                              raw_data: ImageRaw) -> wgpu::TextureView
{
    queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        raw_data.data.as_slice(),
        image_layout,
        size,
    );
    return texture.create_view(&wgpu::TextureViewDescriptor::default());
}

/// 描述纹理顶点数据布局,用于着色器识别数据
pub fn bind_group(device: &wgpu::Device,
                  bind_group_layout: &wgpu::BindGroupLayout,
                  target: &wgpu::TextureView,
                  sampler: &wgpu::Sampler) -> wgpu::BindGroup
{
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(target),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                }
            ],
            label: None,
        }
    )
}

/// 默认采样器描述符
///
/// 用途：配置纹理采样方式（环绕、过滤，多级渐远纹理过滤）
/// 此配置为：环绕=ClampToEdge纹理被约束到0-1之间，造成拉伸效果（大图缩小，小图边缘重复填充）
/// 过滤：纹理被缩小的时候使用邻近过滤Nearest，被放大时使用线性过滤Linear
/// 多级渐远纹理过滤选项Nearest，多级渐远纹理主要是使用在纹理被缩小的情况下的：纹理放大不会使用多级渐远纹理
/// GL_NEAREST产生颗粒状的图案，GL_LINEAR产生更平滑的图案
/// 参考文档：["https://learnopengl-cn.github.io/01%20Getting%20started/06%20Textures/"]
pub const DEFAULT_TEXTURE_SAMPLER: &wgpu::SamplerDescriptor = &wgpu::SamplerDescriptor {
    label: None,
    address_mode_u: wgpu::AddressMode::ClampToEdge,
    address_mode_v: wgpu::AddressMode::ClampToEdge,
    address_mode_w: wgpu::AddressMode::ClampToEdge,
    mag_filter: wgpu::FilterMode::Linear,
    min_filter: wgpu::FilterMode::Nearest,
    mipmap_filter: wgpu::FilterMode::Nearest,
    lod_min_clamp: 0.0,
    lod_max_clamp: f32::MAX,
    compare: None,
    anisotropy_clamp: None,
    border_color: None,
};

/// 默认着色器绑定组描述符
///
/// 用途：设定片段着色器程序传入参数在数据中的位置
/// 此配置为：指定纹理二维坐标，及默认采样器配置
pub const DEFAULT_BIND_GROUP_LAYOUT: &wgpu::BindGroupLayoutDescriptor =
    &wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                count: None,
            },
        ],
        label: Some("default_bind_group_layout"),
    };