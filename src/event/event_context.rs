use crate::event::{Cursor, GEvent};
use crate::graphic::base::Point;

/// 事件上下文接口
pub trait EventContext<M> {
    /// 获取窗口id
    fn get_window_id(&self) -> String;
    /// 设置鼠标位置
    fn set_cursor_pos(&mut self, pos: Point<f32>);
    /// 获取鼠标位置
    fn get_cursor_pos(&self) -> Point<f32>;
    /// 设置鼠标图标
    fn set_cursor_icon(&mut self, cursor: Cursor);
    /// 设置输入框位置
    fn set_ime_position(&mut self);
    /// 设置事件
    fn set_event(&mut self, event: GEvent);
    /// 获取当前事件
    fn get_event(&self) -> GEvent;
    /// 获取自定义消息
    fn get_message(&self) -> Option<&M>;
    /// 设置自定义消息
    fn set_message(&mut self, message: Option<M>);
    /// 发送自定义事件消息
    fn send_message(&self, message: M);
}
