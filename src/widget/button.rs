use std::fmt::Debug;

use winit::event::*;

use crate::device::display_window::WGContext;
use crate::graphic::render_middle::render_function::RenderGraph;
use crate::graphic::style::*;
use crate::widget::component::Component;
use crate::widget::component::ComponentModel;
use crate::widget::listener::{Listener, State};
use crate::graphic::base::shape::{Rectangle, Point, ShapeType};
use crate::graphic::render_middle::pipeline_state::PipelineState;

/// 按钮属性：矩形，背景颜色，聚焦颜色，文字颜色，文本内容
#[derive(Debug)]
pub struct Button {
    pub context: Component,
    pub text: String,
    pub index: Option<usize>,
    pub state: Option<State>,
}

impl<'a> Button {
    pub fn new_with_style<S: Into<String>>(rect: Rectangle, style: Style, text: S) -> Self {
        log::info!("create the Button obj use new");
        Self {
            context: Component::new(rect, style),
            text: text.into(),
            index: None,
            state: None,
        }
    }

    pub fn new<S: Into<String>>(pos: Point, text: S) -> Self {
        let text = text.into();
        let rect = Rectangle::new(pos.x, pos.y, (text.len() * 10) as u32, 40);
        log::info!("create the Button obj use default");
        Self {
            context: Component::new(rect, Style::default()),
            text,
            index: None,
            state: None,
        }
    }

    pub fn set_state(mut self, state: Option<State>) -> Self {
        self.state = state;
        self
    }

    pub fn update_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
        self.context.set_is_redraw(true);
    }

    // pub fn set_style(&mut self, style: Style) {
    //     self.style = style;
    // }
}

impl<'a> ComponentModel for Button {
    fn set_index(&mut self, index: usize) {
        self.index = Option::from(index);
    }

    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn to_graph(&mut self, wgcontext: &WGContext) -> Option<&RenderGraph> {
        let text = &self.text;
        Some(self.context.to_graph(text, wgcontext))
    }

    fn set_glob_pipeline(&self, wgcontext: &WGContext, glob_pipeline: &mut PipelineState) {
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::ROUND);
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::CIRCLE);
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::POLYGON);
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::BORDER);
        glob_pipeline.set_pipeline(&wgcontext.device, ShapeType::TEXTURE);
    }
}

impl<'a> Listener for Button {
    fn key_listener(&mut self, event: &WindowEvent) -> bool {
        // log::info!("---button--- {:?}", event);
        let mut input = false;
        match self.state.as_ref() {
            Some(state) => {
                let key = state.get_key();
                match event {
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                        ..
                    }if virtual_keycode.as_ref() == key => {
                        if *state == ElementState::Pressed {
                            let text = self.text.as_str().to_owned() + "2";
                            self.update_text(text);
                            input = true;
                        } else if *state == ElementState::Released {}
                    }

                    _ => {}
                }
            }
            None => {}
        }
        input
    }
}