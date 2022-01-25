use std::fmt::Formatter;

use crate::event::{EventContext, State};
use crate::graphic::base::{GCharMap, Rectangle};
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::Style;

/// 组件模型trait
/// 作用：定义组件必须的公共方法接口
pub trait ComponentModel<M> {
    /// 组件绘制方法实现
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap);
    fn listener(&mut self, _event_context: &mut dyn EventContext<M>) -> bool {
        false
    }
}

/// 封装组件接口
pub struct Component<M> {
    pub(crate) widget: Box<dyn ComponentModel<M>>,
}

impl<M: Clone + PartialEq> Component<M> {
    pub fn new(widget: impl ComponentModel<M> + 'static) -> Component<M> {
        Component {
            widget: Box::new(widget),
        }
    }
}

impl<M> std::fmt::Debug for Component<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("").finish()
    }
}

/// 键鼠单击动画效果
pub fn action_animation<M>(
    event_context: &dyn EventContext<M>,
    style: &mut Style,
    position: &Rectangle,
    message: Option<M>,
) -> bool {
    let input = position.contain_coord(event_context.get_cursor_pos());
    if input && message.is_some() {
        let message = message.unwrap();
        let hover_color = style.get_hover_color();
        let back_color = style.get_back_color();
        if event_context.get_event().state == State::Pressed {
            style.display_color(hover_color);
            event_context.send_message(message);
        } else if event_context.get_event().state == State::Released {
            style.display_color(back_color);
        }
        return true;
    }
    return false;
}
