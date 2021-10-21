use winit::dpi::PhysicalSize;
use winit::event::*;

use crate::graphic::base::Point;
use crate::widget::*;

impl From<Point<f32>> for winit::dpi::Position {
    #[inline]
    fn from(position: Point<f32>) -> winit::dpi::Position {
        winit::dpi::Position::Physical(
            winit::dpi::PhysicalPosition
            {
                x: position.x as i32,
                y: position.y as i32,
            })
    }
}

impl From<winit::dpi::PhysicalPosition<f64>> for Point<f32> {
    #[inline]
    fn from(position: winit::dpi::PhysicalPosition<f64>) -> Point<f32> {
        Point::new(position.x as f32, position.y as f32)
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Point<u32> {
    fn from(position: PhysicalSize<u32>) -> Self {
        Point::new(position.width, position.height)
    }
}

impl From<winit::event::MouseButton> for Mouse {
    fn from(winit_mouse: MouseButton) -> Self {
        match winit_mouse {
            MouseButton::Left => { Mouse::Left }
            MouseButton::Right => { Mouse::Right }
            MouseButton::Middle => { Mouse::Middle }
            MouseButton::Other(_) => { Mouse::Other }
        }
    }
}

impl From<winit::event::ElementState> for State {
    fn from(winit_state: ElementState) -> Self {
        match winit_state {
            ElementState::Pressed => { State::Pressed }
            ElementState::Released => { State::Released }
        }
    }
}

impl From<&winit::event::WindowEvent<'_>> for GEvent {
    fn from(winit_event: &WindowEvent) -> Self {
        match winit_event {
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => {
                GEvent {
                    event: EventType::Mouse((*button).into()),
                    state: (*state).into(),
                }
            }
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                },
                ..
            } => {
                GEvent {
                    event: EventType::KeyBoard(*virtual_keycode),
                    state: (*state).into(),
                }
            }
            WindowEvent::ReceivedCharacter(c) => {
                GEvent {
                    event: EventType::ReceivedCharacter(*c),
                    state: State::None,
                }
            }
            _ => {
                GEvent {
                    event: EventType::Other,
                    state: State::None,
                }
            }
        }
    }
}