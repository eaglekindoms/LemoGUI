use std::fmt::Formatter;

use crate::device::ELContext;
use crate::graphic::render_middle::RenderUtil;
use crate::widget::{KeyCode, Mouse};

/// 组件模型trait
/// 作用：定义组件必须的公共方法接口
pub trait ComponentModel<M> {
    /// 组件绘制方法实现
    fn draw(&self, render_utils: &mut RenderUtil);
    /// 键盘事件监听器
    fn key_listener(&mut self,
                    _el_context: &ELContext<'_, M>,
                    _virtual_keycode: Option<KeyCode>) -> bool {
        false
    }
    /// 鼠标点击事件监听器
    fn action_listener(&mut self,
                       _el_context: &ELContext<'_, M>,
                       _mouse: Mouse) -> bool
    { false }
    /// 鼠标悬停事件监听器
    fn hover_listener(&mut self,
                      _el_context: &ELContext<'_, M>) -> bool
    { false }
    fn received_character(&mut self,
                          _el_context: &ELContext<'_, M>, c: char) -> bool
    { false }
}

pub struct Component<M> {
    pub(crate) widget: Box<dyn ComponentModel<M>>,
}

impl<M: Copy + PartialEq> Component<M> {
    pub fn new(widget: impl ComponentModel<M> + 'static) -> Component<M> {
        Component {
            widget: Box::new(widget)
        }
    }
}

impl<M> std::fmt::Debug for Component<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("").finish()
    }
}