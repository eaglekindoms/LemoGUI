use std::fmt::Debug;

use crate::device::DisplayWindow;
use crate::widget::{Frame, Panel};

/// 实例 trait
/// 用于定义具体应用
pub trait Instance {
    type M: 'static + Copy + PartialEq + Debug;
    /// 新建实例
    fn new() -> Self;
    /// 组件布局
    fn layout(&self) -> Panel<Self::M>;
    /// 状态更新
    fn update(&mut self, _broadcast: &Self::M) {}
    /// 窗体设置
    fn setting() -> winit::window::WindowBuilder;
    /// 运行实例
    fn run() where Self: 'static + Sized {
        let window = DisplayWindow::new(Self::setting());
        let mut frame = Frame::new();
        let instance = Self::new();
        frame.add_widgets(instance.layout());
        window.start(frame, instance)
    }
}