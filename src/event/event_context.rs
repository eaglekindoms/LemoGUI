use crate::event::{Cursor, GEvent};
use crate::graphic::base::Point;

/// 事件上下文接口
pub trait EventContext<M> {
    /// 设置鼠标位置
    fn set_cursor_pos(&mut self, pos: Point<f32>);
    /// 获取鼠标位置
    fn get_cursor_pos(&self) -> Point<f32>;
    /// 设置鼠标图标
    fn set_cursor_icon(&mut self, cursor: Cursor);
    /// 设置 IME 候选框位置（屏幕坐标，通常是文本插入符）
    fn set_ime_position(&mut self, pos: Point<f32>, height: f32);
    /// 设置事件
    fn set_event(&mut self, event: GEvent);
    /// 获取当前事件
    fn get_event(&self) -> GEvent;
    /// 获取自定义消息
    fn get_message(&self) -> Option<&M>;
    /// 设置自定义消息
    fn set_message(&mut self, message: Option<M>);
    /// 发送自定义事件消息（同帧写入，供 Frame 立刻 update）
    fn send_message(&mut self, message: M);
}
