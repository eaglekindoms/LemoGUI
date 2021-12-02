use std::fmt::Debug;
use std::path::Path;

use winit::window::Icon;

use crate::device::DisplayWindow;
use crate::graphic::base::Point;
use crate::widget::{Frame, Panel};

/// 实例 trait
/// 用于定义具体应用
pub trait Instance {
    type M: 'static + Clone + PartialEq + Debug;
    /// 新建实例
    fn new() -> Self;
    /// 组件布局
    fn layout(&self) -> Panel<Self::M>;
    /// 状态更新
    fn update(&mut self, _broadcast: &Self::M) {}
    /// 窗体设置
    fn setting() -> Setting;
    /// 运行实例
    fn run() where Self: 'static + Sized {
        let setting = Self::setting();
        let mut builder = winit::window::WindowBuilder::new();
        let icon = if setting.icon_path.is_some() {
            load_icon(Path::new(setting.icon_path.unwrap().as_str()))
        } else {
            None
        };
        builder = builder.with_title(setting.title)
            .with_inner_size(winit::dpi::LogicalSize::new(setting.size.x, setting.size.y))
            .with_window_icon(icon);
        let window = DisplayWindow::new(builder);
        let mut frame = Frame::new(setting.font_path);
        let instance = Self::new();
        frame.add_instance(instance);
        window.start(frame)
    }
}

/// 加载icon
pub fn load_icon(path: &Path) -> Option<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Some(Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon"))
}


pub struct Setting {
    pub title: String,
    pub icon_path: Option<String>,
    pub font_path: String,
    pub size: Point<f32>,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            title: "untitled".to_string(),
            icon_path: None,
            font_path: concat!(env!("CARGO_MANIFEST_DIR"), "/res/SourceHanSansCN-Regular.otf").into(),
            size: Point::new(40., 40.),
        }
    }
}