use crate::backend::global_setting::GlobalState;
use crate::backend::mywgpu;
use crate::backend::font::draw_image;
use wgpu::*;

pub struct TextureState {
    // pub texture_bind_group_layout: BindGroupLayout,
    pub diffuse_bind_group: BindGroup,
}

pub struct TextureBuffer<'a> {
    pub x: u32,
    pub y: u32,
    pub buf: &'a [u8],
}

impl<'a> TextureState {
    pub fn default(global_state: &'a GlobalState, texture_buf: &'a TextureBuffer) -> Self {
        let texture_size = Self::create_texture_size(texture_buf.x, texture_buf.y);
        let diffuse_texture = global_state.device.create_texture(
            &Self::create_texture_descriptor(&texture_size)
        );
        global_state.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            Self::create_texture_copy_view(&diffuse_texture),
            // The actual pixel data
            // diffuse_rgba,
            texture_buf.buf,
            // The layout of the texture
            Self::create_texture_data_layout(texture_buf.x, texture_buf.y),
            texture_size,
        );

        /// 默认纹理渲染配置
        let diffuse_texture_view = diffuse_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = global_state.device.create_sampler(&Self::create_sample_descriptor());
        let texture_bind_group_layout = global_state.device.create_bind_group_layout(
            &Self::create_bind_group_layout_descriptor()
        );
        /// 描述纹理顶点数据布局,用于着色器识别数据
        let diffuse_bind_group = global_state.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        Self {
            // texture_bind_group_layout,
            diffuse_bind_group,
        }
    }
    #[deprecated]
    pub fn create_text_texture(global_state: &'a GlobalState, text: &'a str) -> Self {
        // let text = "hello button";
        let (x, y, buf) = draw_image(45.0, text);
        let texture_buf = TextureBuffer { x, y, buf: buf.as_slice() };
        Self::default(global_state, &texture_buf)
    }
    #[deprecated]
    /// 创建默认采样器描述符
    /// 用途：配置纹理采样方式（环绕、过滤，多级渐远纹理过滤）
    /// 此配置为：环绕=ClampToEdge纹理被约束到0-1之间，造成拉伸效果（大图缩小，小图边缘重复填充）
    /// 过滤：纹理被缩小的时候使用邻近过滤Nearest，被放大时使用线性过滤Linear
    /// 多级渐远纹理过滤选项Nearest，多级渐远纹理主要是使用在纹理被缩小的情况下的：纹理放大不会使用多级渐远纹理
    /// GL_NEAREST产生颗粒状的图案，GL_LINEAR产生更平滑的图案
    /// 参考文档：["https://learnopengl-cn.github.io/01%20Getting%20started/06%20Textures/"]
    fn create_sample_descriptor() -> SamplerDescriptor<'a> {
        SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        }
    }
    #[deprecated]
    /// 创建着色器绑定组描述符
    /// 用途：设定片段着色器程序传入参数在数据中的位置
    /// 此配置为：指定纹理二维坐标，及默认采样器配置
    /// 默认配置无需修改
    fn create_bind_group_layout_descriptor() -> BindGroupLayoutDescriptor<'a> {
        BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        }
    }

    /// 定义纹理尺寸(x,y)默认深度设置为1
    fn create_texture_size(w: u32, h: u32) -> Extent3d {
        Extent3d {
            width: w,
            height: h,
            depth: 1,
        }
    }

    /// 定义纹理描述符
    /// 参数：纹理尺寸
    /// 输出配置：定义纹理尺寸，维度：2d，颜色格式：rgba，纹理来源：sampled,copy_dst
    /// 默认配置，无需修改
    fn create_texture_descriptor(texture_size: &'a Extent3d) -> TextureDescriptor<'a> {
        TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: *texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // SAMPLED tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("diffuse_texture"),
        }
    }

    /// 参数：纹理结构体
    /// 用途：指定纹理数据复制的来源为此纹理结构体
    /// 默认配置，无需修改
    fn create_texture_copy_view(diffuse_texture: &'a Texture) -> TextureCopyView<'a> {
        TextureCopyView {
            texture: diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        }
    }

    /// 参数：纹理数据来源的尺寸
    /// 用途：指定纹理数据的布局
    /// 具体含义：偏移量，行数宽度，列数宽度
    /// 注：图像纹理导入后会被转化为包含每个像素点rgba颜色值的一维数组
    /// 因此行数宽度为图像宽度*4，列数宽度不变
    fn create_texture_data_layout(w: u32, h: u32) -> TextureDataLayout {
        TextureDataLayout {
            offset: 0,
            bytes_per_row: 4 * w,
            rows_per_image: h,
        }
    }
}
