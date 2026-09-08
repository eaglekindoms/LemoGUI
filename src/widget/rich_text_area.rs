use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

const PAD: f32 = 8.0;
const LINE_GAP: f32 = 4.0;

#[derive(Clone, Copy)]
struct GlyphPos {
    index: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

/// 多行富文本编辑区（文档快照用 Rc，拖选等瞬时状态留在 Cell）
pub struct RichTextArea<M: Clone> {
    pub bounds: Rectangle,
    pub doc: Rc<RichDocument>,
    pub on_change: Box<dyn Fn(Rc<RichDocument>) -> M>,
    on_copy: Option<Box<dyn Fn() -> M>>,
    on_cut: Option<Box<dyn Fn() -> M>>,
    on_paste: Option<Box<dyn Fn() -> M>>,
    scroll: Option<Rc<ScrollState>>,
    caret: Cell<usize>,
    sel_anchor: Cell<Option<usize>>,
    is_focus: Cell<bool>,
    dragging: Cell<bool>,
    layout_cache: RefCell<Vec<GlyphPos>>,
    layout_from: RefCell<Option<Rc<RichDocument>>>,
    layout_width: Cell<f32>,
    content_h: Cell<f32>,
}

impl<M: Clone + PartialEq> RichTextArea<M> {
    pub fn new<F>(bounds: Rectangle, doc: Rc<RichDocument>, on_change: F) -> Self
    where
        F: 'static + Fn(Rc<RichDocument>) -> M,
    {
        Self {
            caret: Cell::new(doc.caret),
            sel_anchor: Cell::new(doc.sel_anchor),
            is_focus: Cell::new(doc.is_focus),
            dragging: Cell::new(false),
            bounds,
            doc,
            on_change: Box::new(on_change),
            on_copy: None,
            on_cut: None,
            on_paste: None,
            scroll: None,
            layout_cache: RefCell::new(Vec::new()),
            layout_from: RefCell::new(None),
            layout_width: Cell::new(0.0),
            content_h: Cell::new(0.0),
        }
    }

    pub fn scroll_state(mut self, state: Rc<ScrollState>) -> Self {
        self.scroll = Some(state);
        self
    }

    pub fn clipboard<C, T, P>(mut self, copy: C, cut: T, paste: P) -> Self
    where
        C: 'static + Fn() -> M,
        T: 'static + Fn() -> M,
        P: 'static + Fn() -> M,
    {
        self.on_copy = Some(Box::new(copy));
        self.on_cut = Some(Box::new(cut));
        self.on_paste = Some(Box::new(paste));
        self
    }

    fn sync_fonts(&self, font_map: &mut GCharMap) {
        for (i, path) in self.doc.fonts.iter().enumerate() {
            if i > 255 {
                break;
            }
            font_map.ensure_font(i as u8, path);
        }
    }

    fn snapshot(&self) -> RichDocument {
        let mut doc = (*self.doc).clone();
        doc.caret = self.caret.get();
        doc.sel_anchor = self.sel_anchor.get();
        doc.is_focus = self.is_focus.get();
        doc.dragging = false;
        if let Some(s) = &self.scroll {
            doc.scroll = s.value.get();
        }
        doc
    }

    fn emit(&mut self, event_context: &mut dyn EventContext<M>) {
        event_context.send_message((self.on_change)(Rc::clone(&self.doc)));
    }

    fn sync_and_emit(&mut self, event_context: &mut dyn EventContext<M>) {
        self.doc = Rc::new(self.snapshot());
        self.emit(event_context);
    }

    fn edit<F: FnOnce(&mut RichDocument)>(&mut self, f: F) {
        let mut doc = self.snapshot();
        f(&mut doc);
        self.caret.set(doc.caret);
        self.sel_anchor.set(doc.sel_anchor);
        self.is_focus.set(doc.is_focus);
        self.doc = Rc::new(doc);
    }

    fn selection(&self) -> Option<(usize, usize)> {
        let a = self.sel_anchor.get()?;
        let len = self.doc.chars.len();
        let caret = self.caret.get().min(len);
        let lo = a.min(caret).min(len);
        let hi = a.max(caret).min(len);
        if lo == hi {
            None
        } else {
            Some((lo, hi))
        }
    }

    fn content_width(&self) -> f32 {
        self.bounds.width as f32 - PAD * 2.0
    }

    fn view_height(&self) -> f32 {
        self.bounds.height as f32 - PAD * 2.0
    }

    fn layout_is_current(&self) -> bool {
        let w = self.content_width();
        self.layout_width.get() == w
            && self
                .layout_from
                .borrow()
                .as_ref()
                .map(|d| Rc::ptr_eq(d, &self.doc))
                .unwrap_or(false)
    }

    fn relayout(&self, font_map: &mut GCharMap) -> (Vec<GlyphPos>, f32) {
        self.sync_fonts(font_map);
        let max_w = self.content_width();
        let mut out = Vec::with_capacity(self.doc.chars.len() + 1);
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut line_h = 0.0f32;

        for (i, sc) in self.doc.chars.iter().enumerate() {
            if sc.ch == '\n' {
                let h = line_h.max(sc.style.size);
                out.push(GlyphPos {
                    index: i,
                    x,
                    y,
                    w: 0.0,
                    h,
                });
                y += h + LINE_GAP;
                x = 0.0;
                line_h = 0.0;
                continue;
            }
            let m = font_map.metrics_font(sc.ch, sc.style.size, sc.style.font);
            let mut w = m.advance as f32;
            if sc.style.bold {
                w += 1.0;
            }
            let h = m.scale as f32;
            if x + w > max_w && x > 0.0 {
                y += line_h + LINE_GAP;
                x = 0.0;
                line_h = 0.0;
            }
            out.push(GlyphPos {
                index: i,
                x,
                y,
                w,
                h,
            });
            x += w;
            line_h = line_h.max(h);
        }
        let end_h = line_h.max(self.doc.current_style.size);
        out.push(GlyphPos {
            index: self.doc.chars.len(),
            x,
            y,
            w: 0.0,
            h: end_h,
        });
        apply_line_align(&mut out, &self.doc.chars, max_w);
        let content_h = y + end_h + PAD;
        (out, content_h)
    }

    fn ensure_layout(&self, font_map: &mut GCharMap) {
        if self.layout_is_current() {
            return;
        }
        let (layout, content_h) = self.relayout(font_map);
        self.content_h.set(content_h);
        self.layout_width.set(self.content_width());
        *self.layout_from.borrow_mut() = Some(Rc::clone(&self.doc));
        *self.layout_cache.borrow_mut() = layout;
    }

    fn scroll_value(&self) -> f32 {
        self.scroll
            .as_ref()
            .map(|s| s.value.get())
            .unwrap_or(self.doc.scroll)
            .clamp(0.0, 1.0)
    }

    fn scroll_px(&self, content_h: f32) -> f32 {
        let extra = (content_h - self.view_height()).max(0.0);
        self.scroll_value() * extra
    }

    fn origin(&self) -> Point<f32> {
        Point::new(
            self.bounds.position.x + PAD,
            self.bounds.position.y + PAD,
        )
    }

    fn local_pos(&self, cursor: Point<f32>, content_h: f32) -> Point<f32> {
        let o = self.origin();
        Point::new(
            cursor.x - o.x,
            cursor.y - o.y + self.scroll_px(content_h),
        )
    }

    fn hit_test(&self, layout: &[GlyphPos], local: Point<f32>) -> usize {
        if layout.is_empty() {
            return 0;
        }
        let mut best_index = 0usize;
        let mut best_d = f32::MAX;
        for g in layout {
            let cx = g.x + g.w * 0.5;
            let cy = g.y + g.h * 0.5;
            let d = (local.x - cx).abs() + (local.y - cy).abs();
            if d < best_d {
                best_d = d;
                if local.x > g.x + g.w * 0.5 {
                    best_index = (g.index + 1).min(self.doc.chars.len());
                } else {
                    best_index = g.index;
                }
            }
        }
        best_index
    }

    fn caret_glyph(layout: &[GlyphPos], caret: usize, fallback_h: f32) -> GlyphPos {
        if let Some(g) = layout.iter().find(|g| g.index == caret).copied() {
            return g;
        }
        if let Some(last) = layout.last().copied() {
            if caret >= last.index {
                return last;
            }
            return layout
                .iter()
                .min_by_key(|g| g.index.abs_diff(caret))
                .copied()
                .unwrap_or(last);
        }
        GlyphPos {
            index: caret,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: fallback_h,
        }
    }

    fn move_vertical(&mut self, layout: &[GlyphPos], dir: i32) {
        let cur = Self::caret_glyph(layout, self.caret.get(), self.doc.current_style.size);
        let mut best = self.caret.get();
        let mut best_d = f32::MAX;
        for g in layout {
            if dir < 0 && g.y >= cur.y - 0.5 {
                continue;
            }
            if dir > 0 && g.y <= cur.y + 0.5 {
                continue;
            }
            let d = (g.y - cur.y).abs() + (g.x - cur.x).abs() * 0.2;
            if d < best_d {
                best_d = d;
                best = g.index;
            }
        }
        self.caret.set(best.min(self.doc.chars.len()));
        self.sel_anchor.set(None);
    }

    fn visible(&self, g: &GlyphPos, content_h: f32) -> bool {
        let top = g.y - self.scroll_px(content_h);
        let bot = top + g.h;
        bot >= -2.0 && top <= self.view_height() + 2.0
    }

    fn apply_pointer(&mut self, cursor: Point<f32>, extend: bool) {
        let layout = self.layout_cache.borrow();
        let content_h = self.content_h.get();
        let local = self.local_pos(cursor, content_h);
        let idx = self.hit_test(&layout, local);
        if extend {
            if self.sel_anchor.get().is_none() {
                self.sel_anchor.set(Some(self.caret.get()));
            }
            self.caret.set(idx.min(self.doc.chars.len()));
        } else {
            self.caret.set(idx.min(self.doc.chars.len()));
            self.sel_anchor.set(None);
        }
    }

    fn caret_screen(&self) -> (Point<f32>, f32) {
        let content_h = self.content_h.get();
        let origin = self.origin();
        let scroll = self.scroll_px(content_h);
        let layout = self.layout_cache.borrow();
        let g = Self::caret_glyph(&layout, self.caret.get(), self.doc.current_style.size);
        (
            Point::new(origin.x + g.x, origin.y + g.y - scroll),
            g.h.max(16.0),
        )
    }

    fn sync_ime(&self, event_context: &mut dyn EventContext<M>) {
        let (pos, h) = self.caret_screen();
        event_context.set_ime_position(pos, h);
    }
}

impl<M: Clone + PartialEq + 'static> From<RichTextArea<M>> for Component<M> {
    fn from(rta: RichTextArea<M>) -> Self {
        Component::new(rta)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for RichTextArea<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let bg: Box<dyn ShapeGraph> = Box::new(self.bounds);
        paint_brush.draw_shape(&bg, Style::default().back_color(WHITE).border(BLACK));

        self.ensure_layout(font_map);
        let layout = self.layout_cache.borrow();
        let content_h = self.content_h.get();
        if let Some(s) = &self.scroll {
            s.content_h.set(content_h);
            s.view_h.set(self.view_height());
        }

        let scroll = self.scroll_px(content_h);
        let origin = self.origin();

        if let Some((lo, hi)) = self.selection() {
            for g in layout.iter() {
                if g.index >= lo && g.index < hi && self.visible(g, content_h) {
                    let rect = Rectangle::new(
                        origin.x + g.x,
                        origin.y + g.y - scroll,
                        g.w.max(2.0) as u32,
                        g.h.max(1.0) as u32,
                    );
                    let shape: Box<dyn ShapeGraph> = Box::new(rect);
                    paint_brush.draw_shape(
                        &shape,
                        Style::default().back_color(LIGHT_BLUE).no_border(),
                    );
                }
            }
        }

        let mut i = 0;
        while i < layout.len() {
            let g = layout[i];
            if g.index >= self.doc.chars.len() {
                break;
            }
            let sc = &self.doc.chars[g.index];
            if sc.ch == '\n' || !self.visible(&g, content_h) {
                i += 1;
                continue;
            }
            let style = sc.style;
            let line_y = g.y;
            let start_x = g.x;
            let mut text = String::new();
            while i < layout.len() {
                let gi = layout[i];
                if gi.index >= self.doc.chars.len() {
                    break;
                }
                let sci = &self.doc.chars[gi.index];
                if sci.ch == '\n' || sci.style != style || (gi.y - line_y).abs() > 0.1 {
                    break;
                }
                if !self.visible(&gi, content_h) {
                    break;
                }
                text.push(sci.ch);
                i += 1;
            }
            if !text.is_empty() {
                paint_brush.draw_styled_text(
                    font_map,
                    Point::new(origin.x + start_x, origin.y + line_y - scroll),
                    &text,
                    style,
                );
            }
        }

        if self.is_focus.get() {
            let caret = Self::caret_glyph(
                layout.as_slice(),
                self.caret.get(),
                self.doc.current_style.size,
            );
            if self.visible(&caret, content_h) {
                let rect = Rectangle::new(
                    origin.x + caret.x,
                    origin.y + caret.y - scroll,
                    2,
                    caret.h.max(1.0) as u32,
                );
                let shape: Box<dyn ShapeGraph> = Box::new(rect);
                paint_brush.draw_shape(&shape, Style::default().back_color(BLACK).no_border());
            }
        }
    }

    fn ime_caret(&self) -> Option<(Point<f32>, f32)> {
        if self.is_focus.get() {
            Some(self.caret_screen())
        } else {
            None
        }
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let hover = self.bounds.contain_coord(cursor);
        if hover {
            event_context.set_cursor_icon(Cursor::Text);
        }

        match g_event.event {
            EventType::Mouse(Mouse::Left) => {
                if g_event.state == State::Pressed {
                    if hover {
                        self.is_focus.set(true);
                        self.dragging.set(true);
                        self.apply_pointer(cursor, false);
                        self.sel_anchor.set(Some(self.caret.get()));
                        self.sync_ime(event_context);
                        return true;
                    }
                    if self.is_focus.get() {
                        self.is_focus.set(false);
                        self.dragging.set(false);
                        self.sync_and_emit(event_context);
                    }
                    return false;
                } else if g_event.state == State::Released && self.dragging.get() {
                    self.apply_pointer(cursor, true);
                    self.dragging.set(false);
                    self.sync_ime(event_context);
                    self.sync_and_emit(event_context);
                    return true;
                }
            }
            EventType::ReceivedCharacter(c) if self.is_focus.get() => {
                if c == '\u{8}' || c == '\u{7f}' || c == '\r' || c.is_control() {
                    return true;
                }
                self.edit(|d| d.insert(c));
                self.sync_ime(event_context);
                self.emit(event_context);
                return true;
            }
            EventType::KeyBoard(Some(key))
                if self.is_focus.get() && g_event.state == State::Pressed =>
            {
                match key {
                    KeyCode::Backspace => {
                        self.edit(|d| d.delete_backward());
                        self.sync_ime(event_context);
                        self.emit(event_context);
                        return true;
                    }
                    KeyCode::Delete => {
                        self.edit(|d| d.delete_forward());
                        self.sync_ime(event_context);
                        self.emit(event_context);
                        return true;
                    }
                    KeyCode::Return | KeyCode::NumpadEnter => {
                        self.edit(|d| d.insert('\n'));
                        self.sync_ime(event_context);
                        self.emit(event_context);
                        return true;
                    }
                    KeyCode::Left => {
                        let c = self.caret.get();
                        if c > 0 {
                            self.caret.set(c - 1);
                            self.sel_anchor.set(None);
                            self.sync_ime(event_context);
                            self.sync_and_emit(event_context);
                        }
                        return true;
                    }
                    KeyCode::Right => {
                        let c = self.caret.get();
                        if c < self.doc.chars.len() {
                            self.caret.set(c + 1);
                            self.sel_anchor.set(None);
                            self.sync_ime(event_context);
                            self.sync_and_emit(event_context);
                        }
                        return true;
                    }
                    KeyCode::Up => {
                        let layout = self.layout_cache.borrow().clone();
                        self.move_vertical(&layout, -1);
                        self.sync_ime(event_context);
                        self.sync_and_emit(event_context);
                        return true;
                    }
                    KeyCode::Down => {
                        let layout = self.layout_cache.borrow().clone();
                        self.move_vertical(&layout, 1);
                        self.sync_ime(event_context);
                        self.sync_and_emit(event_context);
                        return true;
                    }
                    KeyCode::Copy => {
                        if let Some(f) = &self.on_copy {
                            event_context.send_message(f());
                        }
                        return true;
                    }
                    KeyCode::Cut => {
                        if let Some(f) = &self.on_cut {
                            event_context.send_message(f());
                        }
                        return true;
                    }
                    KeyCode::Paste => {
                        if let Some(f) = &self.on_paste {
                            event_context.send_message(f());
                        }
                        return true;
                    }
                    _ => {}
                }
            }
            EventType::Other if self.dragging.get() && self.is_focus.get() => {
                self.apply_pointer(cursor, true);
                self.sync_ime(event_context);
                return true;
            }
            _ => {}
        }
        false
    }
}

fn apply_line_align(layout: &mut [GlyphPos], chars: &[StyledChar], max_w: f32) {
    let mut i = 0;
    while i < layout.len() {
        let line_y = layout[i].y;
        let start = i;
        while i < layout.len() && (layout[i].y - line_y).abs() < 0.1 {
            i += 1;
        }
        let first = layout[start].index;
        let align = chars
            .get(first)
            .map(|c| c.style.align)
            .unwrap_or(Align::Left);
        let mut line_w = 0.0f32;
        for g in &layout[start..i] {
            line_w = line_w.max(g.x + g.w);
        }
        let shift = match align {
            Align::Left => 0.0,
            Align::Center => ((max_w - line_w) * 0.5).max(0.0),
            Align::Right => (max_w - line_w).max(0.0),
        };
        if shift > 0.0 {
            for g in &mut layout[start..i] {
                g.x += shift;
            }
        }
    }
}
