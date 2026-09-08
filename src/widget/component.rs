use std::fmt::Formatter;

use crate::event::{EventContext, EventType, Mouse, State};
use crate::graphic::base::{GCharMap, Point, Rectangle, RGBA};
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
    /// 一批事件处理完后调用：延迟的 layout 在此落地
    fn commit(&mut self) {}
    /// 当前焦点控件的插入符屏幕坐标（给 IME 候选框用）
    fn ime_caret(&self) -> Option<(Point<f32>, f32)> {
        None
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

/// 悬停 / 按下对应的填充色
pub fn pointer_fill(style: &Style, hover: bool, armed: bool) -> RGBA {
    if armed && hover {
        style.get_hover_color().darken(0.75)
    } else if hover {
        style.get_hover_color()
    } else {
        style.get_back_color()
    }
}

/// 把 display_color 设成悬停/按下色；有变化返回 true
pub fn apply_pointer_fill(style: &mut Style, hover: bool, armed: bool) -> bool {
    let target = pointer_fill(style, hover, armed);
    if style.get_display_color() != target {
        style.display_color(target);
        true
    } else {
        false
    }
}

fn update_armed(event: EventType, state: State, hover: bool, armed: &mut bool) {
    match (event, state) {
        (EventType::Mouse(Mouse::Left), State::Pressed) if hover => *armed = true,
        (EventType::Mouse(Mouse::Left), State::Released) => *armed = false,
        _ => {}
    }
}

pub fn sync_armed<M>(event_context: &dyn EventContext<M>, hover: bool, armed: &mut bool) {
    let g = event_context.get_event();
    update_armed(g.event.clone(), g.state, hover, armed);
}

/// 指针悬停/按下视觉，左键按下时可选发消息
pub fn action_animation<M>(
    event_context: &mut dyn EventContext<M>,
    style: &mut Style,
    position: &Rectangle,
    message: Option<M>,
    armed: &mut bool,
) -> bool {
    let hover = position.contain_coord(event_context.get_cursor_pos());
    let g_event = event_context.get_event();
    let mut fired = false;
    if matches!(
        (&g_event.event, &g_event.state),
        (EventType::Mouse(Mouse::Left), State::Pressed)
    ) && hover
    {
        if let Some(message) = message {
            event_context.send_message(message);
            fired = true;
        }
    }
    update_armed(g_event.event, g_event.state, hover, armed);
    apply_pointer_fill(style, hover, *armed) || fired
}
