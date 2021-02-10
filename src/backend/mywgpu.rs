use wgpu::*;
use wgpu::util::BufferInitDescriptor;

/// 测试效果 底层绑定选择dx11效果最真实且内存占用最小
/// 即创建实例时选用instance = wgpu::Instance::new(wgpu::BackendBit::DX11);
/// RequestAdapterOptions选用PowerPreference::HighPerformance
/// 简化创建各类渲染设置的结构体描述符流程，定义各描述符的默认创建方法
/// 此结构体为元组结构体本身并不储存数据
pub struct description;

/// 用于简化纹理创建流程
pub struct texture;

impl<'a> description {
    #[deprecated]
    /// 创建使用高性能gpu的适配器配置
    /// 用途：配置渲染选项使用gpu or cpu
    /// 此配置为：gpu渲染
    pub fn create_adapter_descriptor(surface: &'a Surface) -> RequestAdapterOptions<'a> {
        RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }
    }
    #[deprecated]
    /// 创建默认的图形设备描述符
    /// 用途：不了解 应该是限制渲染数据量的
    pub fn create_device_descriptor() -> DeviceDescriptor<'a> {
        DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        }
    }
    #[deprecated]
    /// 创建默认采样器描述符
    /// 用途：配置纹理采样方式（环绕、过滤，多级渐远纹理过滤）
    /// 此配置为：环绕=ClampToEdge纹理被约束到0-1之间，造成拉伸效果（大图缩小，小图边缘重复填充）
    /// 过滤：纹理被缩小的时候使用邻近过滤Nearest，被放大时使用线性过滤Linear
    /// 多级渐远纹理过滤选项Nearest，多级渐远纹理主要是使用在纹理被缩小的情况下的：纹理放大不会使用多级渐远纹理
    /// GL_NEAREST产生颗粒状的图案，GL_LINEAR产生更平滑的图案
    /// 参考文档：["https://learnopengl-cn.github.io/01%20Getting%20started/06%20Textures/"]
    pub fn create_sample_descriptor() -> SamplerDescriptor<'a> {
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
    pub fn create_bind_group_layout_descriptor() -> BindGroupLayoutDescriptor<'a> {
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
    #[deprecated]
    /// 交换链描述符
    /// 用途：类似于opengl的context，
    /// 此配置：指定交换缓冲区的尺寸（x,y），颜色格式为rgba，结果为屏幕输出
    pub fn be_rgba_swap_chain_descriptor(w: u32, h: u32) -> SwapChainDescriptor {
        SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: w,
            height: h,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }

    #[deprecated]
    /// 创建初始化缓冲区描述符
    /// 参数：储存在[u8]类型数组中的顶点数据或顶点索引数据
    pub fn create_buffer_init_descriptor(data: &'a [u8], _usage: BufferUsage) -> BufferInitDescriptor<'a> {
        BufferInitDescriptor {
            label: Some("Buffer"),
            contents: data,
            usage: _usage,
        }
    }
    #[deprecated]
    /// 用途： 定义纹理颜色混合模式
    /// 此配置：支持alpha混合透明效果，可根据需求另用其他方法
    /// 参考文档：['https://learnopengl-cn.github.io/04%20Advanced%20OpenGL/03%20Blending/']
    pub fn be_blend_color_state_descriptor(_format: TextureFormat) -> ColorTargetState {
        ColorTargetState {
            format: _format,
            color_blend: wgpu::BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }
    }
}

impl<'a> texture {
    /// 定义纹理尺寸(x,y)默认深度设置为1
    pub fn create_texture_size(w: u32, h: u32) -> Extent3d {
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
    pub fn create_texture_descriptor(texture_size: &'a Extent3d) -> TextureDescriptor<'a> {
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
    pub fn create_texture_copy_view(diffuse_texture: &'a Texture) -> TextureCopyView<'a> {
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
    pub fn create_texture_data_layout(w: u32, h: u32) -> TextureDataLayout {
        TextureDataLayout {
            offset: 0,
            bytes_per_row: 4 * w,
            rows_per_image: h,
        }
    }
}

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct Vertex {
//     pub position: [f32; 3],
//     pub tex_coords: [f32; 2], // NEW!
// }
//
// impl Vertex {
//     pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::InputStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float3,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float2,
//                 },
//             ],
//         }
//     }
// }