use std::num::NonZeroU32;

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
    // pub fn from_bytes(
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    //     bytes: &[u8],
    //     label: &str,
    // ) -> Result<Self> {
    //     let img = image::load_from_memory(bytes)?;
    //     Self::from_image(device, queue, &img, Some(label))
    // }

    pub fn from_char(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        ch: &Character,
    ) -> Self {
        let raw_data = ch.to_raw();

        let size = wgpu::Extent3d {
            width: raw_data.width,
            height: raw_data.height,
            depth_or_array_layers: 1,
        };
        let image_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(raw_data.width),
            rows_per_image: NonZeroU32::new(raw_data.height),
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
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