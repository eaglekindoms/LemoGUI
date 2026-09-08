use std::fmt::Debug;
use std::rc::Rc;

use simple_logger::SimpleLogger;

use LemoGUI::graphic::base::*;
use LemoGUI::graphic::style::*;
use LemoGUI::instance::*;
use LemoGUI::widget::*;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();
    Editor::run();
}

#[derive(Debug, Clone, PartialEq)]
enum DialogKind {
    OpenDoc,
    SaveDoc,
    OpenFont,
    OpenImage {
        display: ImageDisplay,
        embed: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum Msg {
    ToggleBold,
    ToggleItalic,
    ToggleUnderline,
    SetSize(usize),
    SizeMenu(bool),
    SetFont(usize),
    FontMenu(bool),
    SetColor(RGBA),
    SetAlign(Align),
    Doc(Rc<RichDocument>),
    Scroll(f32),
    New,
    Open,
    Save,
    SaveAs,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    AddFont,
    InsertImage {
        display: ImageDisplay,
        embed: bool,
    },
    MenuToggle(Option<usize>),
    DialogDir(String),
    DialogSelect(Option<usize>),
    DialogFilename(String),
    DialogConfirm(String),
    DialogCancel,
    DialogScroll(f32),
}

struct Editor {
    doc: Rc<RichDocument>,
    size_index: usize,
    size_open: bool,
    font_open: bool,
    menu_open: Option<usize>,
    undo: Vec<Rc<RichDocument>>,
    redo: Vec<Rc<RichDocument>>,
    clipboard: Vec<RichAtom>,
    file_path: Option<String>,
    dialog: Option<DialogKind>,
    dialog_dir: String,
    dialog_selected: Option<usize>,
    dialog_filename: String,
    scroll: Rc<ScrollState>,
    dialog_scroll: Rc<ScrollState>,
}

impl Editor {
    fn size_labels() -> Vec<String> {
        FONT_SIZE_BUCKETS.iter().map(|s| s.to_string()).collect()
    }

    fn font_labels(&self) -> Vec<String> {
        self.doc.fonts.iter().map(|p| font_file_name(p)).collect()
    }

    fn font_index(&self) -> usize {
        (self.doc.display_style().font as usize).min(self.doc.fonts.len().saturating_sub(1))
    }

    fn default_dir() -> String {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| ".".to_string())
    }

    fn push_undo(&mut self) {
        self.undo.push(Rc::clone(&self.doc));
        if self.undo.len() > 64 {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    fn replace_doc(&mut self, next: Rc<RichDocument>) {
        if !self.doc.content_eq(&next) {
            self.push_undo();
        }
        self.doc = next;
        self.sync_size_index();
        self.apply_scroll_from_doc();
    }

    fn apply_scroll_from_doc(&self) {
        self.scroll.value.set(self.doc.scroll.clamp(0.0, 1.0));
        self.scroll.dragging.set(false);
    }

    fn mutate_doc<F: FnOnce(&mut RichDocument)>(&mut self, f: F) {
        let before = Rc::clone(&self.doc);
        f(Rc::make_mut(&mut self.doc));
        if !before.content_eq(&self.doc) {
            self.undo.push(before);
            if self.undo.len() > 64 {
                self.undo.remove(0);
            }
            self.redo.clear();
        }
        self.sync_size_index();
    }

    fn sync_size_index(&mut self) {
        if let Some(pos) = FONT_SIZE_BUCKETS
            .iter()
            .position(|&s| s == quantize_size(self.doc.display_style().size))
        {
            self.size_index = pos;
        }
    }

    fn write_path(&mut self, path: &str) -> bool {
        match std::fs::write(path, self.doc.to_toml()) {
            Ok(()) => {
                self.file_path = Some(path.to_string());
                true
            }
            Err(_) => false,
        }
    }

    fn open_dialog(&mut self, kind: DialogKind) {
        self.dialog = Some(kind.clone());
        self.dialog_dir = self
            .file_path
            .as_ref()
            .and_then(|p| {
                std::path::Path::new(p)
                    .parent()
                    .map(|d| d.to_string_lossy().into_owned())
            })
            .filter(|d| !d.is_empty())
            .unwrap_or_else(Self::default_dir);
        self.dialog_selected = None;
        self.dialog_filename = match kind {
            DialogKind::SaveDoc => self
                .file_path
                .as_ref()
                .and_then(|p| {
                    std::path::Path::new(p)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "untitled.toml".to_string()),
            _ => String::new(),
        };
        self.menu_open = None;
        self.size_open = false;
        self.font_open = false;
        Rc::make_mut(&mut self.doc).is_focus = false;
        self.reset_dialog_scroll();
    }

    fn reset_dialog_scroll(&self) {
        self.dialog_scroll.value.set(0.0);
        self.dialog_scroll.dragging.set(false);
    }

    fn close_dialog(&mut self) {
        self.dialog = None;
        self.dialog_selected = None;
    }

    fn file_dialog(&self, mode: FileDialogMode, filter: Option<String>) -> FileDialog<Msg> {
        FileDialog::new(
            Rectangle::new(0.0, 0.0, 900, 600),
            mode,
            self.dialog_dir.clone(),
            filter,
            self.dialog_selected,
            self.dialog_filename.clone(),
            Msg::DialogConfirm,
            || Msg::DialogCancel,
            Msg::DialogDir,
            Msg::DialogSelect,
            Msg::DialogFilename,
        )
        .scroll_state(self.dialog_scroll.clone(), Msg::DialogScroll)
    }
}

impl Instance for Editor {
    type M = Msg;

    fn new() -> Self {
        let mut doc = RichDocument::from_plain("在此输入富文本");
        doc.current_style = TextStyle::default().with_size(16.0);
        for atom in &mut doc.atoms {
            if let RichAtom::Char(ch) = atom {
                ch.style = doc.current_style;
            }
        }
        Editor {
            doc: Rc::new(doc),
            size_index: FONT_SIZE_BUCKETS
                .iter()
                .position(|&s| s == 16)
                .unwrap_or(2),
            size_open: false,
            font_open: false,
            menu_open: None,
            undo: Vec::new(),
            redo: Vec::new(),
            clipboard: Vec::new(),
            file_path: None,
            dialog: None,
            dialog_dir: Self::default_dir(),
            dialog_selected: None,
            dialog_filename: String::new(),
            scroll: ScrollState::new(),
            dialog_scroll: ScrollState::new(),
        }
    }

    fn layout(&self) -> Panel<Msg> {
        if let Some(kind) = &self.dialog {
            let dialog = match kind {
                DialogKind::OpenDoc => self.file_dialog(FileDialogMode::Open, Some(".toml".into())),
                DialogKind::SaveDoc => self.file_dialog(FileDialogMode::Save, Some(".toml".into())),
                DialogKind::OpenFont => {
                    self.file_dialog(FileDialogMode::Open, Some(".ttf,.otf".into()))
                }
                DialogKind::OpenImage { .. } => self.file_dialog(
                    FileDialogMode::Open,
                    Some(".png,.jpg,.jpeg,.gif,.bmp,.webp".into()),
                ),
            };
            return Panel::new().push(dialog);
        }

        let style = self.doc.display_style();
        let size_labels = Self::size_labels();
        let font_labels = self.font_labels();

        let menu_h = 28u32;
        let file = Menu::new(
            "文件",
            Rectangle::new(0.0, 0.0, 56, menu_h),
            vec![
                MenuItem::new("新建", Msg::New),
                MenuItem::new("打开…", Msg::Open),
                MenuItem::new("保存", Msg::Save),
                MenuItem::new("另存为…", Msg::SaveAs),
            ],
            self.menu_open == Some(0),
        )
        .item_width(100);
        let edit = Menu::new(
            "编辑",
            Rectangle::new(56.0, 0.0, 56, menu_h),
            vec![
                MenuItem::new("撤销", Msg::Undo),
                MenuItem::new("重做", Msg::Redo),
                MenuItem::new("剪切", Msg::Cut),
                MenuItem::new("复制", Msg::Copy),
                MenuItem::new("粘贴", Msg::Paste),
                MenuItem::new("全选", Msg::SelectAll),
            ],
            self.menu_open == Some(1),
        )
        .item_width(80);
        let format = Menu::new(
            "格式",
            Rectangle::new(112.0, 0.0, 56, menu_h),
            vec![
                MenuItem::new("粗体", Msg::ToggleBold),
                MenuItem::new("斜体", Msg::ToggleItalic),
                MenuItem::new("下划线", Msg::ToggleUnderline),
                MenuItem::new("左对齐", Msg::SetAlign(Align::Left)),
                MenuItem::new("居中", Msg::SetAlign(Align::Center)),
                MenuItem::new("右对齐", Msg::SetAlign(Align::Right)),
                MenuItem::new("添加字体…", Msg::AddFont),
            ],
            self.menu_open == Some(2),
        )
        .item_width(110);
        let insert = Menu::new(
            "插入",
            Rectangle::new(168.0, 0.0, 56, menu_h),
            vec![
                MenuItem::new(
                    "图片链接（行内）…",
                    Msg::InsertImage {
                        display: ImageDisplay::Inline,
                        embed: false,
                    },
                ),
                MenuItem::new(
                    "图片链接（独占一行）…",
                    Msg::InsertImage {
                        display: ImageDisplay::Block,
                        embed: false,
                    },
                ),
                MenuItem::new(
                    "图片文件（行内）…",
                    Msg::InsertImage {
                        display: ImageDisplay::Inline,
                        embed: true,
                    },
                ),
                MenuItem::new(
                    "图片文件（独占一行）…",
                    Msg::InsertImage {
                        display: ImageDisplay::Block,
                        embed: true,
                    },
                ),
            ],
            self.menu_open == Some(3),
        )
        .item_width(200);
        let menu_bar = MenuBar::new(
            Rectangle::new(0.0, 0.0, 900, menu_h),
            vec![file, edit, format, insert],
            Msg::MenuToggle,
        );

        let toolbar_y = 32.0;
        let b = ToggleButton::new_with_rect(
            Rectangle::new(12.0, toolbar_y, 36, 32),
            "B",
            |_| Msg::ToggleBold,
        )
        .pressed(style.bold);
        let i = ToggleButton::new_with_rect(
            Rectangle::new(52.0, toolbar_y, 36, 32),
            "I",
            |_| Msg::ToggleItalic,
        )
        .pressed(style.italic);
        let u = ToggleButton::new_with_rect(
            Rectangle::new(92.0, toolbar_y, 36, 32),
            "U",
            |_| Msg::ToggleUnderline,
        )
        .pressed(style.underline);

        let size_combo = ComboBox::new(
            Rectangle::new(140.0, toolbar_y, 70, 32),
            size_labels,
            self.size_index,
            self.size_open,
            Msg::SetSize,
            Msg::SizeMenu,
        );

        let font_combo = ComboBox::new(
            Rectangle::new(218.0, toolbar_y, 170, 32),
            font_labels,
            self.font_index(),
            self.font_open,
            Msg::SetFont,
            Msg::FontMenu,
        );

        let palette = ColorPalette::new(Point::new(398.0, 36.0), Msg::SetColor)
            .selected_color(style.color);

        let clear = Button::new_with_style(
            Rectangle::new(600.0, toolbar_y, 70, 32),
            Style::default()
                .back_color(LIGHT_WHITE)
                .border(BLACK)
                .font_color(BLACK),
            "清空",
        )
        .action(Msg::New);

        let viewport = Rectangle::new(12.0, 72.0, 876, 516);
        let mut area = RichTextArea::new(
            Rectangle::new(12.0, 72.0, 860, 516),
            Rc::clone(&self.doc),
            Msg::Doc,
        )
        .clipboard(|| Msg::Copy, || Msg::Cut, || Msg::Paste)
        .scroll_state(self.scroll.clone());
        if let Some(path) = &self.file_path {
            if let Some(dir) = std::path::Path::new(path).parent() {
                area = area.base_dir(dir.to_string_lossy().into_owned());
            }
        }
        let viewer = ScrollViewer::new(viewport, area, self.scroll.clone(), Msg::Scroll);

        Panel::new()
            .push(b)
            .push(i)
            .push(u)
            .push(palette)
            .push(clear)
            .push(viewer)
            .push(size_combo)
            .push(font_combo)
            .push(menu_bar)
    }

    fn update(&mut self, broadcast: &Msg) {
        match broadcast {
            Msg::ToggleBold => {
                self.mutate_doc(|d| d.apply_to_selection(|s| s.bold = !s.bold));
                self.menu_open = None;
            }
            Msg::ToggleItalic => {
                self.mutate_doc(|d| d.apply_to_selection(|s| s.italic = !s.italic));
                self.menu_open = None;
            }
            Msg::ToggleUnderline => {
                self.mutate_doc(|d| d.apply_to_selection(|s| s.underline = !s.underline));
                self.menu_open = None;
            }
            Msg::SetSize(index) => {
                self.size_index = *index;
                self.size_open = false;
                self.menu_open = None;
                if let Some(&bucket) = FONT_SIZE_BUCKETS.get(*index) {
                    self.mutate_doc(|d| d.apply_to_selection(|s| s.size = bucket as f32));
                }
            }
            Msg::SizeMenu(open) => {
                self.size_open = *open;
                if *open {
                    self.font_open = false;
                    self.menu_open = None;
                }
            }
            Msg::SetFont(index) => {
                self.font_open = false;
                self.menu_open = None;
                if *index < self.doc.fonts.len() {
                    let font = *index as u8;
                    self.mutate_doc(|d| d.apply_to_selection(|s| s.font = font));
                }
            }
            Msg::FontMenu(open) => {
                self.font_open = *open;
                if *open {
                    self.size_open = false;
                    self.menu_open = None;
                }
            }
            Msg::SetColor(color) => {
                self.mutate_doc(|d| d.apply_to_selection(|s| s.color = *color));
                self.menu_open = None;
            }
            Msg::SetAlign(align) => {
                let align = *align;
                self.mutate_doc(|d| d.apply_to_selection(|s| s.align = align));
                self.menu_open = None;
            }
            Msg::Doc(doc) => {
                self.replace_doc(Rc::clone(doc));
            }
            Msg::Scroll(v) => {
                Rc::make_mut(&mut self.doc).scroll = *v;
                self.scroll.value.set(*v);
            }
            Msg::New => {
                let focus = self.doc.is_focus;
                let mut next = RichDocument::default();
                next.current_style = TextStyle::default().with_size(16.0);
                next.is_focus = focus;
                self.replace_doc(Rc::new(next));
                self.file_path = None;
                self.size_index = 2;
                self.menu_open = None;
            }
            Msg::Open => self.open_dialog(DialogKind::OpenDoc),
            Msg::Save => {
                self.menu_open = None;
                if let Some(path) = self.file_path.clone() {
                    self.write_path(&path);
                } else {
                    self.open_dialog(DialogKind::SaveDoc);
                }
            }
            Msg::SaveAs => self.open_dialog(DialogKind::SaveDoc),
            Msg::Undo => {
                if let Some(prev) = self.undo.pop() {
                    self.redo.push(Rc::clone(&self.doc));
                    if self.redo.len() > 64 {
                        self.redo.remove(0);
                    }
                    let focus = self.doc.is_focus;
                    self.doc = prev;
                    Rc::make_mut(&mut self.doc).is_focus = focus;
                    self.sync_size_index();
                    self.apply_scroll_from_doc();
                }
                self.menu_open = None;
            }
            Msg::Redo => {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(Rc::clone(&self.doc));
                    if self.undo.len() > 64 {
                        self.undo.remove(0);
                    }
                    let focus = self.doc.is_focus;
                    self.doc = next;
                    Rc::make_mut(&mut self.doc).is_focus = focus;
                    self.sync_size_index();
                    self.apply_scroll_from_doc();
                }
                self.menu_open = None;
            }
            Msg::Cut => {
                let copied = self.doc.copy_selection();
                if !copied.is_empty() {
                    self.mutate_doc(|d| {
                        d.delete_selection();
                    });
                    self.clipboard = copied;
                }
                self.menu_open = None;
            }
            Msg::Copy => {
                let copied = self.doc.copy_selection();
                if !copied.is_empty() {
                    self.clipboard = copied;
                }
                self.menu_open = None;
            }
            Msg::Paste => {
                if !self.clipboard.is_empty() {
                    let clip = self.clipboard.clone();
                    self.mutate_doc(|d| d.paste(&clip));
                }
                self.menu_open = None;
            }
            Msg::SelectAll => {
                Rc::make_mut(&mut self.doc).select_all();
                self.menu_open = None;
            }
            Msg::AddFont => self.open_dialog(DialogKind::OpenFont),
            Msg::InsertImage { display, embed } => self.open_dialog(DialogKind::OpenImage {
                display: *display,
                embed: *embed,
            }),
            Msg::MenuToggle(open) => {
                self.menu_open = *open;
                if open.is_some() {
                    self.size_open = false;
                    self.font_open = false;
                }
            }
            Msg::DialogDir(dir) => {
                self.dialog_dir = dir.clone();
                self.dialog_selected = None;
                self.reset_dialog_scroll();
            }
            Msg::DialogSelect(sel) => {
                self.dialog_selected = *sel;
                if matches!(self.dialog, Some(DialogKind::SaveDoc)) {
                    if let Some(i) = *sel {
                        if let Some(name) = dialog_file_name(&self.dialog_dir, Some(".toml"), i) {
                            self.dialog_filename = name;
                        }
                    }
                }
            }
            Msg::DialogFilename(name) => {
                self.dialog_filename = name.clone();
            }
            Msg::DialogScroll(v) => {
                self.dialog_scroll.value.set(*v);
            }
            Msg::DialogConfirm(path) => {
                let kind = self.dialog.clone();
                self.close_dialog();
                match kind {
                    Some(DialogKind::OpenDoc) => {
                        if let Ok(text) = std::fs::read_to_string(path) {
                            if let Some(doc) = RichDocument::from_toml(&text) {
                                self.replace_doc(Rc::new(doc));
                                self.file_path = Some(path.clone());
                            }
                        }
                    }
                    Some(DialogKind::SaveDoc) => {
                        if self.write_path(path) {
                            self.file_path = Some(path.clone());
                        }
                    }
                    Some(DialogKind::OpenFont) => {
                        if !self.doc.fonts.iter().any(|f| f == path) {
                            let path = path.clone();
                            self.mutate_doc(|d| {
                                d.fonts.push(path.clone());
                                let id = (d.fonts.len() - 1) as u8;
                                d.apply_to_selection(|s| s.font = id);
                            });
                        }
                    }
                    Some(DialogKind::OpenImage { display, embed }) => {
                        if embed {
                            if let Ok(bytes) = std::fs::read(path) {
                                let source = ImageSource::Embedded(Rc::from(bytes));
                                self.mutate_doc(|d| d.insert_image(source, display));
                            }
                        } else {
                            let source = ImageSource::Path(path.clone());
                            self.mutate_doc(|d| d.insert_image(source, display));
                        }
                    }
                    None => {}
                }
            }
            Msg::DialogCancel => self.close_dialog(),
        }
    }

    fn setting() -> Setting {
        let mut setting = Setting::default();
        setting.title = "LemoGUI Rich Editor".to_string();
        setting.size = Point::new(900., 600.);
        setting.icon_path = Some(concat!(env!("CARGO_MANIFEST_DIR"), "/res/icon.png").into());
        setting
    }
}

fn font_file_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}
