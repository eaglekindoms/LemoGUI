use crate::event::*;
use crate::graphic::base::*;
use crate::graphic::render_api::PaintBrush;
use crate::graphic::style::*;
use crate::widget::*;

const ROW_H: u32 = 24;
const BTN_W: u32 = 80;
const BTN_H: u32 = 28;
const PAD: f32 = 12.0;

/// 打开或保存
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileDialogMode {
    Open,
    Save,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EntryKind {
    Parent,
    Dir,
    File,
}

struct DirEntry {
    kind: EntryKind,
    name: String,
    path: String,
}

/// 库内文件选择对话框（状态在 Instance，layout 重建）
pub struct FileDialog<M: Clone> {
    pub bounds: Rectangle,
    pub mode: FileDialogMode,
    pub dir: String,
    pub filter: Option<String>,
    pub selected: Option<usize>,
    pub filename: String,
    on_confirm: Box<dyn Fn(String) -> M>,
    on_cancel: Box<dyn Fn() -> M>,
    on_dir: Box<dyn Fn(String) -> M>,
    on_select: Box<dyn Fn(Option<usize>) -> M>,
    on_filename: Box<dyn Fn(String) -> M>,
    hover_row: Option<usize>,
    hover_confirm: bool,
    hover_cancel: bool,
    hover_filename: bool,
    armed: bool,
}

impl<M: Clone + PartialEq> FileDialog<M> {
    pub fn new<FC, FX, FD, FS, FF>(
        bounds: Rectangle,
        mode: FileDialogMode,
        dir: String,
        filter: Option<String>,
        selected: Option<usize>,
        filename: String,
        on_confirm: FC,
        on_cancel: FX,
        on_dir: FD,
        on_select: FS,
        on_filename: FF,
    ) -> Self
    where
        FC: 'static + Fn(String) -> M,
        FX: 'static + Fn() -> M,
        FD: 'static + Fn(String) -> M,
        FS: 'static + Fn(Option<usize>) -> M,
        FF: 'static + Fn(String) -> M,
    {
        Self {
            bounds,
            mode,
            dir,
            filter,
            selected,
            filename,
            on_confirm: Box::new(on_confirm),
            on_cancel: Box::new(on_cancel),
            on_dir: Box::new(on_dir),
            on_select: Box::new(on_select),
            on_filename: Box::new(on_filename),
            hover_row: None,
            hover_confirm: false,
            hover_cancel: false,
            hover_filename: false,
            armed: false,
        }
    }

    fn panel(&self) -> Rectangle {
        let w = 520u32;
        let h = 400u32;
        let x = self.bounds.position.x + (self.bounds.width.saturating_sub(w) as f32) * 0.5;
        let y = self.bounds.position.y + (self.bounds.height.saturating_sub(h) as f32) * 0.5;
        Rectangle::new(x, y, w, h)
    }

    fn list_rect(&self) -> Rectangle {
        let p = self.panel();
        let extra = if self.mode == FileDialogMode::Save {
            36.0
        } else {
            0.0
        };
        Rectangle::new(
            p.position.x + PAD,
            p.position.y + 52.0,
            p.width - (PAD * 2.0) as u32,
            (p.height as f32 - 52.0 - 48.0 - extra - PAD) as u32,
        )
    }

    fn filename_rect(&self) -> Rectangle {
        let p = self.panel();
        Rectangle::new(
            p.position.x + PAD,
            p.position.y + p.height as f32 - 48.0 - 36.0,
            p.width - (PAD * 2.0) as u32,
            28,
        )
    }

    fn confirm_rect(&self) -> Rectangle {
        let p = self.panel();
        Rectangle::new(
            p.position.x + p.width as f32 - PAD - (BTN_W * 2 + 8) as f32,
            p.position.y + p.height as f32 - PAD - BTN_H as f32,
            BTN_W,
            BTN_H,
        )
    }

    fn cancel_rect(&self) -> Rectangle {
        let p = self.panel();
        Rectangle::new(
            p.position.x + p.width as f32 - PAD - BTN_W as f32,
            p.position.y + p.height as f32 - PAD - BTN_H as f32,
            BTN_W,
            BTN_H,
        )
    }

    fn title_rect(&self) -> Rectangle {
        let p = self.panel();
        Rectangle::new(p.position.x + PAD, p.position.y + 8.0, p.width - 24, 22)
    }

    fn dir_rect(&self) -> Rectangle {
        let p = self.panel();
        Rectangle::new(p.position.x + PAD, p.position.y + 28.0, p.width - 24, 20)
    }

    fn list_entries(&self) -> Vec<DirEntry> {
        list_dir(&self.dir, &self.filter)
    }

    fn visible_count(&self) -> usize {
        (self.list_rect().height / ROW_H.max(1)) as usize
    }

    fn list_offset(&self, count: usize) -> usize {
        let vis = self.visible_count().max(1);
        if let Some(sel) = self.selected {
            if sel >= vis {
                (sel + 1).saturating_sub(vis).min(count.saturating_sub(vis))
            } else {
                0
            }
        } else {
            0
        }
    }

    fn row_rect(&self, visible_index: usize) -> Rectangle {
        let list = self.list_rect();
        Rectangle::new(
            list.position.x,
            list.position.y + (visible_index as u32 * ROW_H) as f32,
            list.width,
            ROW_H,
        )
    }

    fn hit_row(&self, cursor: Point<f32>, entries: &[DirEntry]) -> Option<usize> {
        let list = self.list_rect();
        if !list.contain_coord(cursor) {
            return None;
        }
        let offset = self.list_offset(entries.len());
        let vis_i = ((cursor.y - list.position.y) as u32 / ROW_H.max(1)) as usize;
        let abs = offset + vis_i;
        (abs < entries.len()).then_some(abs)
    }

    fn confirm_path(&self, entries: &[DirEntry]) -> Option<String> {
        match self.mode {
            FileDialogMode::Open => {
                let i = self.selected?;
                let e = entries.get(i)?;
                if e.kind == EntryKind::File {
                    Some(e.path.clone())
                } else {
                    None
                }
            }
            FileDialogMode::Save => {
                let name = if self.filename.is_empty() {
                    self.selected
                        .and_then(|i| entries.get(i))
                        .filter(|e| e.kind == EntryKind::File)
                        .map(|e| e.name.clone())
                        .unwrap_or_default()
                } else {
                    self.filename.clone()
                };
                if name.is_empty() {
                    None
                } else {
                    Some(join_path(&self.dir, &name))
                }
            }
        }
    }
}

impl<M: Clone + PartialEq + 'static> From<FileDialog<M>> for Component<M> {
    fn from(fd: FileDialog<M>) -> Self {
        Component::new(fd)
    }
}

impl<M: Clone + PartialEq> ComponentModel<M> for FileDialog<M> {
    fn draw(&self, paint_brush: &mut dyn PaintBrush, font_map: &mut GCharMap) {
        let overlay: Box<dyn ShapeGraph> = Box::new(self.bounds);
        paint_brush.draw_shape(
            &overlay,
            Style::default()
                .back_color(RGBA(0.0, 0.0, 0.0, 0.35))
                .no_border(),
        );

        let panel = self.panel();
        let panel_shape: Box<dyn ShapeGraph> = Box::new(panel);
        paint_brush.draw_shape(
            &panel_shape,
            Style::default().back_color(WHITE).border(BLACK),
        );

        let title = match self.mode {
            FileDialogMode::Open => "打开",
            FileDialogMode::Save => "保存",
        };
        paint_brush.draw_text(font_map, &self.title_rect(), title, BLACK);
        paint_brush.draw_text(
            font_map,
            &self.dir_rect(),
            &truncate_chars(&self.dir, 42),
            BLACK,
        );

        let entries = self.list_entries();
        let list = self.list_rect();
        let list_shape: Box<dyn ShapeGraph> = Box::new(list);
        paint_brush.draw_shape(
            &list_shape,
            Style::default().back_color(WHITE).border(BLACK),
        );

        let offset = self.list_offset(entries.len());
        let vis = self.visible_count();
        for (vi, (abs, entry)) in entries
            .iter()
            .enumerate()
            .skip(offset)
            .take(vis)
            .enumerate()
        {
            let row = self.row_rect(vi);
            let hover = self.hover_row == Some(abs);
            let idle = if self.selected == Some(abs) {
                LIGHT_BLUE
            } else {
                WHITE
            };
            let fill = pointer_fill(
                &Style::default().back_color(idle).hover_color(LIGHT_BLUE),
                hover,
                self.armed,
            );
            let style = Style::default().back_color(fill).font_color(BLACK);
            let row_shape: Box<dyn ShapeGraph> = Box::new(row);
            paint_brush.draw_shape(&row_shape, style);
            let label = match entry.kind {
                EntryKind::Parent => "..".to_string(),
                EntryKind::Dir => format!("{}/", truncate_chars(&entry.name, 40)),
                EntryKind::File => truncate_chars(&entry.name, 42),
            };
            paint_brush.draw_text(font_map, &row, &label, style.get_font_color());
        }

        if self.mode == FileDialogMode::Save {
            let fr = self.filename_rect();
            let fill = pointer_fill(
                &Style::default().back_color(WHITE).hover_color(LIGHT_BLUE),
                self.hover_filename,
                self.armed,
            );
            let fr_shape: Box<dyn ShapeGraph> = Box::new(fr);
            paint_brush.draw_shape(
                &fr_shape,
                Style::default().back_color(fill).border(BLACK),
            );
            paint_brush.draw_text(font_map, &fr, &self.filename, BLACK);
        }

        let ok_fill = pointer_fill(&Style::default(), self.hover_confirm, self.armed);
        let ok: Box<dyn ShapeGraph> = Box::new(self.confirm_rect());
        paint_brush.draw_shape(
            &ok,
            Style::default()
                .back_color(ok_fill)
                .border(BLACK)
                .font_color(BLACK),
        );
        paint_brush.draw_text(font_map, &self.confirm_rect(), "确定", BLACK);

        let cancel_fill = pointer_fill(&Style::default(), self.hover_cancel, self.armed);
        let cancel: Box<dyn ShapeGraph> = Box::new(self.cancel_rect());
        paint_brush.draw_shape(
            &cancel,
            Style::default()
                .back_color(cancel_fill)
                .border(BLACK)
                .font_color(BLACK),
        );
        paint_brush.draw_text(font_map, &self.cancel_rect(), "取消", BLACK);
    }

    fn listener(&mut self, event_context: &mut dyn EventContext<M>) -> bool {
        let g_event = event_context.get_event();
        let cursor = event_context.get_cursor_pos();
        let on_overlay = self.bounds.contain_coord(cursor);
        let entries = self.list_entries();
        let on_row = self.hit_row(cursor, &entries);
        let on_confirm = self.confirm_rect().contain_coord(cursor);
        let on_cancel = self.cancel_rect().contain_coord(cursor);
        let on_filename =
            self.mode == FileDialogMode::Save && self.filename_rect().contain_coord(cursor);

        let prev_row = self.hover_row;
        let prev_confirm = self.hover_confirm;
        let prev_cancel = self.hover_cancel;
        let prev_filename = self.hover_filename;
        let prev_armed = self.armed;
        self.hover_row = on_row;
        self.hover_confirm = on_confirm;
        self.hover_cancel = on_cancel;
        self.hover_filename = on_filename;
        sync_armed(
            event_context,
            on_row.is_some() || on_confirm || on_cancel || on_filename,
            &mut self.armed,
        );
        let visual = self.hover_row != prev_row
            || self.hover_confirm != prev_confirm
            || self.hover_cancel != prev_cancel
            || self.hover_filename != prev_filename
            || self.armed != prev_armed;

        match g_event.event {
            EventType::Mouse(Mouse::Left) if g_event.state == State::Pressed => {
                if on_confirm {
                    if let Some(path) = self.confirm_path(&entries) {
                        event_context.send_message((self.on_confirm)(path));
                    }
                    return true;
                }
                if on_cancel {
                    event_context.send_message((self.on_cancel)());
                    return true;
                }
                if let Some(abs) = on_row {
                    if let Some(entry) = entries.get(abs) {
                        match entry.kind {
                            EntryKind::Parent | EntryKind::Dir => {
                                event_context.send_message((self.on_dir)(entry.path.clone()));
                            }
                            EntryKind::File => {
                                event_context.send_message((self.on_select)(Some(abs)));
                            }
                        }
                    }
                    return true;
                }
                return on_overlay || visual;
            }
            EventType::ReceivedCharacter(c) if self.mode == FileDialogMode::Save => {
                if c == '\u{8}' || c == '\u{7f}' || c == '\r' || c.is_control() {
                    return true;
                }
                let mut name = self.filename.clone();
                name.push(c);
                event_context.send_message((self.on_filename)(name));
                return true;
            }
            EventType::KeyBoard(Some(KeyCode::Backspace))
                if self.mode == FileDialogMode::Save && g_event.state == State::Pressed =>
            {
                let mut name = self.filename.clone();
                name.pop();
                event_context.send_message((self.on_filename)(name));
                return true;
            }
            EventType::KeyBoard(Some(KeyCode::Return | KeyCode::NumpadEnter))
                if g_event.state == State::Pressed =>
            {
                if let Some(path) = self.confirm_path(&entries) {
                    event_context.send_message((self.on_confirm)(path));
                }
                return true;
            }
            _ => {}
        }
        visual
    }
}

fn matches_filter(name: &str, filter: &Option<String>) -> bool {
    let Some(f) = filter else {
        return true;
    };
    let lower = name
        .chars()
        .flat_map(|c| c.to_lowercase())
        .collect::<String>();
    f.split(|c| c == ',' || c == '|').any(|ext| {
        let ext = ext.trim().chars().flat_map(|c| c.to_lowercase()).collect::<String>();
        !ext.is_empty() && lower.ends_with(&ext)
    })
}

fn parent_dir(dir: &str) -> String {
    std::path::Path::new(dir)
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .filter(|p| !p.is_empty())
        .unwrap_or_else(|| dir.to_string())
}

fn join_path(dir: &str, name: &str) -> String {
    std::path::Path::new(dir)
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn list_dir(dir: &str, filter: &Option<String>) -> Vec<DirEntry> {
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for ent in rd.flatten() {
            let name = ent.file_name().to_string_lossy().into_owned();
            let path = ent.path().to_string_lossy().into_owned();
            let is_dir = ent.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if is_dir {
                dirs.push(DirEntry {
                    kind: EntryKind::Dir,
                    name,
                    path,
                });
            } else if matches_filter(&name, filter) {
                files.push(DirEntry {
                    kind: EntryKind::File,
                    name,
                    path,
                });
            }
        }
    }
    dirs.sort_by(|a, b| a.name.cmp(&b.name));
    files.sort_by(|a, b| a.name.cmp(&b.name));
    let mut out = vec![DirEntry {
        kind: EntryKind::Parent,
        name: "..".to_string(),
        path: parent_dir(dir),
    }];
    out.extend(dirs);
    out.extend(files);
    out
}

/// 按对话框同一套排序取文件名（点选后带入保存框）
pub fn dialog_file_name(dir: &str, filter: Option<&str>, index: usize) -> Option<String> {
    let filter = filter.map(|s| s.to_string());
    list_dir(dir, &filter)
        .into_iter()
        .nth(index)
        .filter(|e| e.kind == EntryKind::File)
        .map(|e| e.name)
}

fn truncate_chars(s: &str, max: usize) -> String {
    let count = s.chars().count();
    if count <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}
