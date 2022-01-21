use std::fmt::Debug;

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
    fn run()
    where
        Self: 'static + Sized,
    {
        let setting = Self::setting();
        let font_path = setting.clone().font_path;
        let window = DisplayWindow::new(setting);
        let mut frame = Frame::new(font_path);
        let instance = Self::new();
        frame.add_instance(instance);
        window.start(frame)
    }
}

/// 窗口配置结构体
#[derive(Clone)]
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
            font_path: concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/res/SourceHanSansCN-Regular.otf"
            )
            .into(),
            size: Point::new(40., 40.),
        }
    }
}
