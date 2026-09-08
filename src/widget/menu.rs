use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

/// 菜单项
pub struct MenuItem<M: Clone> {
    pub title: String,
    pub message: M,
}

impl<M: Clone> MenuItem<M> {
    pub fn new<S: Into<String>>(title: S, message: M) -> Self {
        Self {
            title: title.into(),
            message,
        }
    }
}

/// 单个下拉菜单
pub struct Menu<M: Clone> {
    pub title: String,
    pub bounds: Rectangle,
    pub items: Vec<MenuItem<M>>,
    pub open: bool,
    pub item_height: u32,
    pub item_width: u32,
}

impl<M: Clone> Menu<M> {
    pub fn new<S: Into<String>, I>(title: S, bounds: Rectangle, items: I, open: bool) -> Self
    where
        I: IntoIterator<Item = MenuItem<M>>,
    {
        Self {
            title: title.into(),
            bounds,
            items: items.into_iter().collect(),
            open,
            item_height: 26,
            item_width: 88,
        }
    }

    pub fn item_width(mut self, w: u32) -> Self {
        self.item_width = w;
        self
    }

    fn dropdown_rect(&self) -> Rectangle {
        let h = self.item_height * self.items.len() as u32;
        let w = self.item_width.max(self.bounds.width);
        Rectangle::new(
            self.bounds.position.x,
            self.bounds.position.y + self.bounds.height as f32,
            w,
            h,
        )
    }

    fn item_rect(&self, index: usize) -> Rectangle {
        let drop = self.dropdown_rect();
        Rectangle::new(
            drop.position.x,
            drop.position.y + (index as u32 * self.item_height) as f32,
            drop.width,
            self.item_height,
        )
    }
}

/// 顶栏菜单条
pub struct MenuBar<M: Clone> {
    pub bounds: Rectangle,
    pub menus: Vec<Menu<M>>,
    pub on_toggle: Box<dyn Fn(Option<usize>) -> M>,
    hover_title: Option<usize>,
    hover_item: Option<(usize, usize)>,
    armed: bool,
}

impl<M: Clone + PartialEq> MenuBar<M> {
    pub fn new<F, I>(bounds: Rectangle, menus: I, on_toggle: F) -> Self
    where
        F: 'static + Fn(Option<usize>) -> M,
        I: IntoIterator<Item = Menu<M>>,
    {
        Self {
            bounds,
            menus: menus.into_iter().collect(),
            on_toggle: Box::new(on_toggle),
            hover_title: None,
            hover_item: None,
            armed: false,
        }
    }

    fn any_open(&self) -> bool {
        self.menus.iter().any(|m| m.open)
    }

    fn hit_title(&self, cursor: Point<f32>) -> Option<usize> {
        self.menus
            .iter()
            .position(|m| m.bounds.contain_coord(cursor))
    }

    fn hit_item(&self, cursor: Point<f32>) -> Option<(usize, usize)> {
        for (mi, menu) in self.menus.iter().enumerate() {
            if !menu.open {
                continue;
            }
            if !menu.dropdown_rect().contain_coord(cursor) {
                continue;
            }
            let rel_y = cursor.y - menu.dropdown_rect().position.y;
            let index = rel_y as usize / menu.item_height.max(1) as usize;
            if index < menu.items.len() {
                return Some((mi, index));
            }
        }
        None
    }
}

impl<M: Clone + PartialEq + 'static> From<MenuBar<M>> for Component<M> {
    fn from(bar: MenuBar<M>) -> Self {
        Component::new(bar)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for MenuBar<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let bar_shape: Box<dyn ShapeGraph> = Box::new(self.bounds);
        paint_brush.draw_shape(
            &bar_shape,
            Style::default().back_color(LIGHT_WHITE).border(BLACK),
        );
        for (mi, menu) in self.menus.iter().enumerate() {
            let title_hover = self.hover_title == Some(mi);
            let title_fill = if menu.open {
                pointer_fill(
                    &Style::default()
                        .back_color(LIGHT_BLUE)
                        .hover_color(LIGHT_BLUE),
                    title_hover,
                    self.armed,
                )
            } else {
                pointer_fill(&Style::default(), title_hover, self.armed)
            };
            let title_style = Style::default()
                .back_color(title_fill)
                .border(BLACK)
                .font_color(BLACK);
            let title_shape: Box<dyn ShapeGraph> = Box::new(menu.bounds);
            paint_brush.draw_shape(&title_shape, title_style);
            paint_brush.draw_text(
                font_map,
                &menu.bounds,
                menu.title.as_str(),
                title_style.get_font_color(),
            );
            if menu.open {
                let drop = menu.dropdown_rect();
                let bg: Box<dyn ShapeGraph> = Box::new(drop);
                paint_brush.draw_shape(&bg, Style::default().back_color(WHITE).border(BLACK));
                for (i, item) in menu.items.iter().enumerate() {
                    let row = menu.item_rect(i);
                    let fill = pointer_fill(
                        &Style::default().back_color(WHITE).hover_color(LIGHT_BLUE),
                        self.hover_item == Some((mi, i)),
                        self.armed,
                    );
                    let row_style = Style::default().back_color(fill).font_color(BLACK);
                    let row_shape: Box<dyn ShapeGraph> = Box::new(row);
                    paint_brush.draw_shape(&row_shape, row_style);
                    paint_brush.draw_text(
                        font_map,
                        &row,
                        item.title.as_str(),
                        row_style.get_font_color(),
                    );
                }
            }
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let on_title = self.hit_title(cursor);
        let on_item = self.hit_item(cursor);
        let prev_title = self.hover_title;
        let prev_item = self.hover_item;
        let prev_armed = self.armed;
        self.hover_title = on_title;
        self.hover_item = on_item;
        sync_armed(
            event_context,
            on_title.is_some() || on_item.is_some(),
            &mut self.armed,
        );
        let visual = self.hover_title != prev_title
            || self.hover_item != prev_item
            || self.armed != prev_armed;

        if let EventType::Mouse(Mouse::Left) = g_event.event {
            if g_event.state == State::Pressed {
                if let Some((mi, ii)) = on_item {
                    if let Some(item) = self.menus.get(mi).and_then(|m| m.items.get(ii)) {
                        event_context.send_message(item.message.clone());
                        return true;
                    }
                }
                if let Some(i) = on_title {
                    let next = if self.menus.get(i).map(|m| m.open).unwrap_or(false) {
                        None
                    } else {
                        Some(i)
                    };
                    event_context.send_message((self.on_toggle)(next));
                    return true;
                }
                if self.any_open() {
                    event_context.send_message((self.on_toggle)(None));
                    return true;
                }
            }
        }
        visual
    }
}
